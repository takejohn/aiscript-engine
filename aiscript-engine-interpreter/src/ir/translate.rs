use std::borrow::Cow;

use crate::library::{Library, LibraryValue, NativeFn};
use aiscript_engine_ast::{self as ast, NamespaceMember};
use aiscript_engine_common::{
    AiScriptBasicError, AiScriptBasicErrorKind, NamePath, Utf16Str, Utf16String,
};

use super::{
    scopes::{Scopes, Variable},
    DataIndex, DataItem, Instruction, Ir, Register, UserFn, UserFnIndex,
};

pub fn translate(ast: &[ast::Node]) -> Ir<'static> {
    if ast.is_empty() {
        return Ir::default();
    }
    let mut translator = Translator::new();
    translator.translate(ast);
    return translator.build();
}

pub struct Translator<'ast, 'lib> {
    scopes: Scopes<'ast>,
    native_functions: Vec<NativeFn<'lib>>,
    data: Vec<DataItem>,
    register_length: usize,
    block: Vec<Instruction>,
    blocks: Vec<Vec<Instruction>>,
}

impl<'ast, 'lib> Translator<'ast, 'lib> {
    pub fn new() -> Self {
        Translator {
            scopes: Scopes::new(),
            native_functions: Vec::new(),
            data: Vec::new(),
            register_length: 0,
            block: Vec::new(),
            blocks: Vec::new(),
        }
    }

    pub fn link_library(&mut self, library: Library<'lib>) {
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
                    let index = self.str_literal(&value);
                    self.append_instruction(Instruction::Data(register, index));
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

    pub fn translate(&mut self, ast: &'ast [ast::Node]) {
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

    pub fn build(self) -> Ir<'lib> {
        let entry_point = UserFn {
            register_length: self.register_length,
            instructions: self.block,
        };
        Ir {
            data: self.data,
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
                let register = self.use_register();
                self.eval_expr(register, &node.expr);
                match node.op {
                    aiscript_engine_ast::AssignOperator::Assign => {
                        self.assign(&node.dest, register)
                    }
                    aiscript_engine_ast::AssignOperator::AddAssign => todo!(),
                    aiscript_engine_ast::AssignOperator::SubAssign => todo!(),
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
                self.enter_block();
                self.eval_statement_or_expr(register, &node.then);
                let then_code = self.exit_block();

                for _ in &node.elseif {
                    self.enter_block();
                }

                // else節
                let mut else_code = match &node.else_statement {
                    Some(else_statement) => {
                        self.enter_block();
                        self.eval_statement_or_expr(register, else_statement);
                        self.exit_block()
                    }
                    None => vec![Instruction::Null(register)],
                };

                // elif節
                for elif in node.elseif.iter().rev() {
                    self.eval_expr(register, &elif.cond);

                    self.enter_block();
                    self.eval_statement_or_expr(register, &elif.then);
                    let elif_code = self.exit_block();

                    self.append_instruction(Instruction::If(register, elif_code, else_code));
                    else_code = self.exit_block();
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
                let index = self.str_literal(&node.value);
                self.append_instruction(Instruction::Data(register, index));
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
                self.append_instruction(Instruction::Obj(register, len));
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
            ast::Expression::Binary(_node) => todo!(),
        }
    }

    fn eval_arr(&mut self, register: Register, exprs: &'ast [ast::Expression]) {
        let len = exprs.len();
        self.append_instruction(Instruction::Arr(register, len));
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

    fn assign(&mut self, dest: &'ast ast::Expression, src: Register) {
        match dest {
            ast::Expression::Identifier(dest) => match self.scopes.assign(&dest.name) {
                Ok(dest) => self.append_instruction(Instruction::Move(dest, src)),
                Err(error) => self.append_instruction(Instruction::Panic(error)),
            },
            ast::Expression::Index(dest) => {
                let target = self.use_register();
                self.eval_expr(target, &dest.target);
                let index = self.use_register();
                self.eval_expr(index, &dest.index);
                self.append_instruction(Instruction::Store(src, target, index));
            }
            ast::Expression::Prop(dest) => {
                let target = self.use_register();
                self.eval_expr(target, &dest.target);
                let name = self.str_literal(&dest.name);
                self.append_instruction(Instruction::StoreProp(src, target, name));
            }
            ast::Expression::Arr(dest) => {
                let temp = self.use_register();
                for (index, item) in dest.value.iter().enumerate() {
                    self.append_instruction(Instruction::LoadIndex(temp, src, index));
                    self.assign(item, temp);
                }
            }
            ast::Expression::Obj(dest) => {
                let temp = self.use_register();
                for (key, item) in &dest.value {
                    let key = self.str_literal(&key);
                    self.append_instruction(Instruction::LoadProp(temp, src, key));
                    self.assign(item, temp);
                }
            }
            _ => {
                self.append_instruction(Instruction::Panic(AiScriptBasicError::new(
                    AiScriptBasicErrorKind::Runtime,
                    "The left-hand side of an assignment expression must be a variable or a property/index access.",
                    None,
                )));
            }
        }
    }

    fn use_register(&mut self) -> Register {
        let index = self.register_length;
        self.register_length += 1;
        return index;
    }

    fn enter_block(&mut self) {
        self.blocks
            .push(std::mem::replace(&mut self.block, Vec::new()));
        self.scopes.push_block_scope();
    }

    fn exit_block(&mut self) -> Vec<Instruction> {
        self.scopes.drop_local_scope();
        std::mem::replace(&mut self.block, self.blocks.pop().expect("no outer blocks"))
    }

    fn append_instruction(&mut self, instruction: Instruction) {
        self.block.push(instruction);
    }

    fn add_native_function(&mut self, f: NativeFn<'lib>) -> UserFnIndex {
        let index = self.native_functions.len();
        self.native_functions.push(f);
        return index;
    }

    fn str_literal(&mut self, s: &Utf16Str) -> DataIndex {
        let existing = self.data.iter().enumerate().find_map(|(index, item)| {
            let DataItem::Str(str) = item;
            if str.as_utf16_str() == s {
                Some(index)
            } else {
                None
            }
        });
        if let Some(index) = existing {
            return index;
        }
        let index = self.data.len();
        self.data.push(DataItem::Str(s.to_owned()));
        return index;
    }
}
