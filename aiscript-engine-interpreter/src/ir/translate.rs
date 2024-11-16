use std::{borrow::Cow, collections::HashSet, rc::Rc};

use crate::library::{Library, LibraryValue, NativeFn};
use aiscript_engine_ast::{self as ast, NamespaceMember};
use aiscript_engine_common::{
    AiScriptBasicError, AiScriptBasicErrorKind, NamePath, Utf16Str, Utf16String,
};
use aiscript_engine_values::{VObj, Value};
use gc::{Gc, GcCell};
use indexmap::IndexMap;

use super::{
    reference::Reference,
    scopes::{Scopes, Variable},
    Instruction, Ir, Register, UserFn, UserFnIndex,
};

pub(crate) struct Translator<'ast> {
    scopes: Scopes<'ast>,
    native_functions: Vec<NativeFn>,
    strings: HashSet<Rc<[u16]>>,
    register_length: usize,
    block: Vec<Instruction>,
    procedures: Vec<Vec<Instruction>>,
}

impl<'ast> Translator<'ast> {
    pub(crate) fn new() -> Self {
        Translator {
            scopes: Scopes::new(),
            native_functions: Vec::new(),
            strings: HashSet::new(),
            register_length: 0,
            block: Vec::new(),
            procedures: Vec::new(),
        }
    }

    pub(crate) fn link_library(&mut self, library: Library) {
        for (name, value) in library {
            let register = self.use_register();
            match value {
                LibraryValue::Null => self.append_instruction(Instruction::Null(register)),
                LibraryValue::Bool(value) => {
                    self.append_instruction(Instruction::Bool(register, value))
                }
                LibraryValue::Num(value) => {
                    self.append_instruction(Instruction::Num(register, value))
                }
                LibraryValue::Str(value) => {
                    let value = self.str_literal(&value);
                    self.append_instruction(Instruction::Str(register, value));
                }
                LibraryValue::Obj(_value) => todo!(),
                LibraryValue::Arr(_value) => todo!(),
                LibraryValue::Fn(value) => {
                    let index = self.add_native_function(value);
                    self.append_instruction(Instruction::NativeFn(register, index));
                }
            }
            self.scopes.root.add(
                Cow::Owned(NamePath::from(Utf16String::from(name))),
                Variable {
                    register,
                    is_mutable: false,
                },
            );
        }
    }

    pub(crate) fn translate(&mut self, ast: &'ast [ast::Node]) {
        if ast.is_empty() {
            return;
        }
        self.collect_ns(ast.iter().filter_map(|ns| match ns {
            aiscript_engine_ast::Node::Ns(node) => Some(node),
            _ => None,
        }));
        let register = self.use_register();
        self.run(register, ast);
    }

    pub(crate) fn build(self) -> Ir {
        let entry_point = UserFn {
            register_length: self.register_length,
            instructions: self.block,
        };
        Ir {
            native_functions: self.native_functions,
            entry_point,
        }
    }

    fn collect_ns(&mut self, namespaces: impl IntoIterator<Item = &'ast ast::Namespace>) {
        for ns in namespaces {
            self.scopes.push_namespace_scope(&ns.name);
            self.collect_ns_member(ns);
            self.scopes.drop_local_scope();
        }
    }

    fn collect_ns_member(&mut self, ns: &'ast ast::Namespace) {
        self.collect_ns(ns.members.iter().filter_map(|member| match member {
            NamespaceMember::Ns(node) => Some(node),
            NamespaceMember::Def(_) => None,
        }));

        for node in &ns.members {
            if let NamespaceMember::Def(node) = node {
                let ast::Expression::Identifier(dest) = &node.dest else {
                    self.append_instruction(Instruction::Panic(AiScriptBasicError::new(
                        AiScriptBasicErrorKind::Namespace,
                        "Destructuring assignment is invalid in namespace declarations.",
                        Some(node.loc.start.clone()),
                    )));
                    return;
                };
                if node.is_mut {
                    self.append_instruction(Instruction::Panic(AiScriptBasicError::new(
                        AiScriptBasicErrorKind::Namespace,
                        String::from("No \"var\" in namespace declaration: ")
                            + &dest.name.to_string(),
                        Some(node.loc.start.clone()),
                    )));
                    return;
                }

                let register = self.use_register();
                self.eval_expr(register, &node.expr);
                self.define_identifier(dest, register, node.is_mut);
            }
        }
    }

    fn run(&mut self, register: Register, nodes: &'ast [ast::Node]) {
        for node in nodes {
            self.eval(register, node);
        }
    }

    fn eval(&mut self, register: Register, node: &'ast ast::Node) {
        match node {
            ast::Node::Ns(_) | ast::Node::Meta(_) => {
                self.append_instruction(Instruction::Null(register))
            }
            ast::Node::Statement(node) => self.eval_statement(register, node),
            ast::Node::Expr(node) => self.eval_expr(register, node),
            ast::Node::Attr(_) | ast::Node::TypeSource(_) => panic!("invalid node type"),
        }
    }

    fn eval_statement_or_expr(
        &mut self,
        register: Register,
        node: &'ast ast::StatementOrExpression,
    ) {
        match node {
            ast::StatementOrExpression::Statement(node) => self.eval_statement(register, node),
            ast::StatementOrExpression::Expression(node) => self.eval_expr(register, node),
        }
    }

    fn eval_statement(&mut self, register: Register, node: &'ast ast::Statement) {
        match node {
            ast::Statement::Def(node) => {
                let register = self.use_register();
                self.eval_expr(register, &node.expr);
                self.define(&node.dest, register, node.is_mut);
            }
            ast::Statement::Return(_node) => todo!(),
            ast::Statement::Each(_node) => todo!(),
            ast::Statement::For(_node) => todo!(),
            ast::Statement::Loop(_node) => todo!(),
            ast::Statement::Break(_node) => todo!(),
            ast::Statement::Continue(_node) => todo!(),
            ast::Statement::Assign(node) => {
                let right = self.use_register();
                self.eval_expr(right, &node.expr);
                match node.op {
                    aiscript_engine_ast::AssignOperator::Assign => {
                        if let Some(dest) = self.get_reference(&node.dest) {
                            self.assign(&dest, right);
                        }
                    }
                    aiscript_engine_ast::AssignOperator::AddAssign => {
                        if let Some(dest) = self.get_reference(&node.dest) {
                            let left = self.use_register();
                            self.dereference(left, &dest);
                            let result = self.use_register();
                            self.append_instruction(Instruction::Add(result, left, right));
                            self.assign(&dest, result);
                        }
                    }
                    aiscript_engine_ast::AssignOperator::SubAssign => {
                        if let Some(dest) = self.get_reference(&node.dest) {
                            let left = self.use_register();
                            self.dereference(left, &dest);
                            let result = self.use_register();
                            self.append_instruction(Instruction::Sub(result, left, right));
                            self.assign(&dest, result);
                        }
                    }
                }
            }
        }
        self.append_instruction(Instruction::Null(register));
    }

    fn eval_expr(&mut self, register: Register, node: &'ast ast::Expression) {
        match node {
            ast::Expression::If(node) => {
                self.eval_expr(register, &node.cond);

                // then節
                self.begin_block();
                self.eval_statement_or_expr(register, &node.then);
                let then_code = self.end_block();

                for _ in &node.elseif {
                    self.begin_block();
                }

                // else節
                let mut else_code = match &node.else_statement {
                    Some(else_statement) => {
                        self.begin_block();
                        self.eval_statement_or_expr(register, else_statement);
                        self.end_block()
                    }
                    None => vec![Instruction::Null(register)],
                };

                // elif節
                for elif in node.elseif.iter().rev() {
                    self.eval_expr(register, &elif.cond);

                    self.begin_block();
                    self.eval_statement_or_expr(register, &elif.then);
                    let elif_code = self.end_block();

                    self.append_instruction(Instruction::If(register, elif_code, else_code));
                    else_code = self.end_block();
                }

                self.append_instruction(Instruction::If(register, then_code, else_code));
            }
            ast::Expression::Fn(_node) => todo!(),
            ast::Expression::Match(_node) => todo!(),
            ast::Expression::Block(node) => {
                for statement in &node.statements {
                    self.eval_statement_or_expr(register, statement);
                }
            }
            ast::Expression::Exists(node) => {
                self.append_instruction(Instruction::Bool(
                    register,
                    self.scopes.exists(&node.identifier.name),
                ));
            }
            ast::Expression::Tmpl(_node) => todo!(),
            ast::Expression::Str(node) => {
                let value = self.str_literal(&node.value);
                self.append_instruction(Instruction::Str(register, value));
            }
            ast::Expression::Num(node) => {
                self.append_instruction(Instruction::Num(register, node.value));
            }
            ast::Expression::Bool(node) => {
                self.append_instruction(Instruction::Bool(register, node.value));
            }
            ast::Expression::Null(_) => {
                self.append_instruction(Instruction::Null(register));
            }
            ast::Expression::Obj(node) => {
                let len = node.value.len();
                self.append_instruction(Instruction::Obj(
                    register,
                    Gc::new(GcCell::new(VObj::new())),
                ));
                if len > 0 {
                    let value_register = self.use_register();
                    for (key, value) in &node.value {
                        self.eval_expr(value_register, value);
                        let key = self.str_literal(key);
                        self.append_instruction(Instruction::StoreProp(
                            value_register,
                            register,
                            key,
                        ));
                    }
                }
            }
            ast::Expression::Arr(node) => {
                self.eval_arr(register, &node.value);
            }
            ast::Expression::Not(node) => {
                let src = self.use_register();
                self.eval_expr(src, &node.expr);
                self.append_instruction(Instruction::Not(register, src));
            }
            ast::Expression::Identifier(node) => {
                let name = &node.name;
                if let Some(variable) = self.scopes.get(name) {
                    let src = variable.register;
                    self.append_instruction(Instruction::Move(register, src));
                } else {
                    self.append_instruction(Instruction::Panic(AiScriptBasicError::new(
                        AiScriptBasicErrorKind::Runtime,
                        format!(
                            "No such variable '{}' in scope '{}'",
                            name,
                            self.scopes.current_scope_name()
                        ),
                        None,
                    )));
                }
            }
            ast::Expression::Call(node) => {
                let target = self.use_register();
                self.eval_expr(target, &node.target);
                let args = self.use_register();
                self.eval_arr(args, &node.args);
                self.append_instruction(Instruction::Call(register, target, args));
            }
            ast::Expression::Index(node) => {
                let target = self.use_register();
                self.eval_expr(target, &node.target);
                let index = self.use_register();
                self.eval_expr(index, &node.index);
                self.append_instruction(Instruction::Load(register, target, index));
            }
            ast::Expression::Prop(node) => {
                let target = self.use_register();
                self.eval_expr(target, &node.target);
                let name = self.str_literal(&node.name);
                self.append_instruction(Instruction::LoadProp(register, target, name));
            }
            ast::Expression::Binary(node) => {
                match &node.op {
                    ast::BinaryOperator::Arithmetic(op) => {
                        let left = self.use_register();
                        let right = self.use_register();
                        self.eval_expr(left, &node.left);
                        self.eval_expr(right, &node.right);
                        let instruction = match op {
                            ast::BinaryArithmeticOperator::Pow => todo!(),
                            ast::BinaryArithmeticOperator::Mul => todo!(),
                            ast::BinaryArithmeticOperator::Div => todo!(),
                            ast::BinaryArithmeticOperator::Rem => todo!(),
                            ast::BinaryArithmeticOperator::Add => {
                                Instruction::Add(register, left, right)
                            }
                            ast::BinaryArithmeticOperator::Sub => {
                                Instruction::Sub(register, left, right)
                            }
                            ast::BinaryArithmeticOperator::Lt => todo!(),
                            ast::BinaryArithmeticOperator::Lteq => todo!(),
                            ast::BinaryArithmeticOperator::Gt => todo!(),
                            ast::BinaryArithmeticOperator::Gteq => todo!(),
                            ast::BinaryArithmeticOperator::Eq => todo!(),
                            ast::BinaryArithmeticOperator::Neq => todo!(),
                        };
                        self.append_instruction(instruction);
                    }
                    ast::BinaryOperator::Logical(op) => match op {
                        ast::BinaryLogicalOperator::And => {
                            self.eval_expr(register, &node.left);
                            // 右辺の処理
                            let right = {
                                self.begin_procedure();
                                self.eval_expr(register, &node.right);
                                self.end_procedure()
                            };
                            // 短絡処理: 左辺が真なら右辺を実行
                            self.append_instruction(Instruction::If(register, right, vec![]));
                        }
                        ast::BinaryLogicalOperator::Or => {
                            self.eval_expr(register, &node.left);
                            // 右辺の処理
                            let right = {
                                self.begin_procedure();
                                self.eval_expr(register, &node.right);
                                self.end_procedure()
                            };
                            // 短絡処理: 左辺が偽なら右辺を実行
                            self.append_instruction(Instruction::If(register, vec![], right));
                        }
                    },
                }
            }
        }
    }

    fn eval_arr(&mut self, register: Register, exprs: &'ast [ast::Expression]) {
        let len = exprs.len();
        self.append_instruction(Instruction::Arr(
            register,
            Gc::new(GcCell::new(vec![Value::Uninitialized; len])),
        ));
        if len > 0 {
            let value_register = self.use_register();
            for (index, value) in exprs.iter().enumerate() {
                self.eval_expr(value_register, value);
                self.append_instruction(Instruction::StoreIndex(value_register, register, index));
            }
        }
    }

    fn define(&mut self, dest: &'ast ast::Expression, register: Register, is_mutable: bool) {
        match dest {
            ast::Expression::Identifier(dest) => self.define_identifier(dest, register, is_mutable),
            ast::Expression::Arr(dest) => self.define_arr(dest, register, is_mutable),
            ast::Expression::Obj(dest) => self.define_obj(dest, register, is_mutable),
            _ => {
                self.append_instruction(Instruction::Panic(AiScriptBasicError::new(
                    AiScriptBasicErrorKind::Runtime,
                    "The left-hand side of an definition expression must be a variable.",
                    None,
                )));
            }
        }
    }

    fn define_identifier(
        &mut self,
        dest: &'ast ast::Identifier,
        register: Register,
        is_mutable: bool,
    ) {
        self.scopes.add(
            &dest.name,
            Variable {
                is_mutable,
                register,
            },
        );
    }

    fn define_arr(&mut self, dest: &'ast ast::Arr, register: Register, is_mutable: bool) {
        for (i, item) in dest.value.iter().enumerate() {
            let dest_register = self.use_register();
            self.append_instruction(Instruction::LoadIndex(dest_register, register, i));
            self.define(item, dest_register, is_mutable);
        }
    }

    fn define_obj(&mut self, dest: &'ast ast::Obj, register: Register, is_mutable: bool) {
        for (key, item) in &dest.value {
            let key_str = self.str_literal(&key);
            let dest_register = self.use_register();
            self.append_instruction(Instruction::LoadProp(dest_register, register, key_str));
            self.define(item, dest_register, is_mutable);
        }
    }

    fn get_reference(&mut self, dest: &'ast ast::Expression) -> Option<Reference> {
        match dest {
            ast::Expression::Identifier(dest) => {
                let result = self.scopes.assign(&dest.name);
                match result {
                    Ok(register) => Some(Reference::Variable { register }),
                    Err(err) => {
                        self.append_instruction(Instruction::Panic(err));
                        None
                    }
                }
            }
            ast::Expression::Index(dest) => {
                let target = self.use_register();
                self.eval_expr(target, &dest.target);
                let index = self.use_register();
                self.eval_expr(index, &dest.index);
                Some(Reference::Index { target, index })
            }
            ast::Expression::Prop(dest) => {
                let target = self.use_register();
                self.eval_expr(target, &dest.target);
                let name = self.str_literal(&dest.name);
                Some(Reference::Prop { target, name })
            }
            ast::Expression::Arr(dest) => {
                let items: Option<Vec<_>> = dest
                    .value
                    .iter()
                    .map(|item| self.get_reference(item))
                    .collect();
                Some(Reference::Arr { items: items? })
            }
            ast::Expression::Obj(dest) => {
                let entries: Option<IndexMap<_, _>> = dest
                    .value
                    .iter()
                    .map(|(key, item)| {
                        let key = self.str_literal(&key);
                        let item = self.get_reference(item)?;
                        Some((key, item))
                    })
                    .collect();
                Some(Reference::Obj { entries: entries? })
            }
            _ => {
                self.append_instruction(Instruction::Panic(AiScriptBasicError::new(
                    AiScriptBasicErrorKind::Runtime,
                    "The left-hand side of an assignment expression must be a variable or a property/index access.",
                    None,
                )));
                None
            }
        }
    }

    fn dereference(&mut self, dest: Register, reference: &Reference) {
        match reference {
            Reference::Variable { register } => {
                self.append_instruction(Instruction::Move(dest, *register));
            }
            Reference::Index { target, index } => {
                self.append_instruction(Instruction::Load(dest, *target, *index));
            }
            Reference::Prop { target, name } => {
                self.append_instruction(Instruction::LoadProp(dest, *target, Rc::clone(name)));
            }
            Reference::Arr { items } => {
                self.append_instruction(Instruction::Arr(
                    dest,
                    Gc::new(GcCell::new(vec![Value::Uninitialized; items.len()])),
                ));
                let temp = self.use_register();
                for (index, item) in items.iter().enumerate() {
                    self.dereference(temp, item);
                    self.append_instruction(Instruction::StoreIndex(temp, dest, index));
                }
            }
            Reference::Obj { entries } => {
                self.append_instruction(Instruction::Obj(dest, Gc::new(GcCell::new(VObj::new()))));
                let temp = self.use_register();
                for (key, item) in entries {
                    self.dereference(temp, item);
                    self.append_instruction(Instruction::StoreProp(temp, dest, Rc::clone(key)));
                }
            }
        }
    }

    fn assign(&mut self, dest: &Reference, src: Register) {
        match dest {
            Reference::Variable { register } => {
                self.append_instruction(Instruction::Move(*register, src));
            }
            Reference::Index { target, index } => {
                self.append_instruction(Instruction::Store(src, *target, *index));
            }
            Reference::Prop { target, name } => {
                self.append_instruction(Instruction::StoreProp(src, *target, Rc::clone(name)));
            }
            Reference::Arr { items } => {
                let temp = self.use_register();
                for (index, item) in items.iter().enumerate() {
                    self.append_instruction(Instruction::LoadIndex(temp, src, index));
                    self.assign(item, temp);
                }
            }
            Reference::Obj { entries } => {
                let temp = self.use_register();
                for (key, item) in entries {
                    self.append_instruction(Instruction::LoadProp(temp, src, Rc::clone(key)));
                    self.assign(item, temp);
                }
            }
        }
    }

    fn use_register(&mut self) -> Register {
        let index = self.register_length;
        self.register_length += 1;
        return index;
    }

    /// 新しく命令列を開始します。
    fn begin_procedure(&mut self) {
        self.procedures
            .push(std::mem::replace(&mut self.block, Vec::new()));
    }

    /// ローカルスコープを生成して新しく命令列を開始します。
    fn begin_block(&mut self) {
        self.begin_procedure();
        self.scopes.push_block_scope();
    }

    /// ローカルスコープを破棄し、命令列を終了して返します。
    fn end_block(&mut self) -> Vec<Instruction> {
        self.scopes.drop_local_scope();
        self.end_procedure()
    }

    /// 命令列を終了して返します。
    fn end_procedure(&mut self) -> Vec<Instruction> {
        std::mem::replace(
            &mut self.block,
            self.procedures.pop().expect("no outer blocks"),
        )
    }

    fn append_instruction(&mut self, instruction: Instruction) {
        self.block.push(instruction);
    }

    fn add_native_function(&mut self, f: NativeFn) -> UserFnIndex {
        let index = self.native_functions.len();
        self.native_functions.push(f);
        return index;
    }

    fn str_literal(&mut self, s: &Utf16Str) -> Rc<[u16]> {
        match self.strings.get(s.as_u16s()) {
            Some(existing) => Rc::clone(existing),
            None => {
                let rc = Rc::from(s.as_u16s());
                self.strings.insert(Rc::clone(&rc));
                rc
            }
        }
    }
}
