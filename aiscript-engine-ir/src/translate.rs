use aiscript_engine_ast::{self as ast, NamespaceMember};
use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind, Utf16Str};

use crate::{
    scopes::{Scopes, Variable},
    DataIndex, DataItem, Instruction, Ir, Procedure, Register,
};

pub fn translate(ast: &[ast::Node]) -> Ir {
    Translator::new().translate(ast)
}

struct Translator<'ast> {
    scopes: Scopes<'ast>,
    data: Vec<DataItem>,
    register_length: usize,
    block: Vec<Instruction>,
    blocks: Vec<Vec<Instruction>>,
}

impl<'ast> Translator<'ast> {
    fn new() -> Self {
        Translator {
            scopes: Scopes::new(),
            data: Vec::new(),
            register_length: 0,
            block: Vec::new(),
            blocks: Vec::new(),
        }
    }

    fn translate(mut self, ast: &'ast [ast::Node]) -> Ir {
        if ast.is_empty() {
            return Ir::default();
        }
        self.collect_ns(ast.iter().filter_map(|ns| match ns {
            aiscript_engine_ast::Node::Ns(node) => Some(node),
            _ => None,
        }));
        let register = self.use_register();
        self.run(register, ast);
        return self.build();
    }

    fn build(self) -> Ir {
        Ir {
            data: self.data,
            functions: vec![Procedure {
                register_length: self.register_length,
                instructions: self.block,
            }],
            entry_point: 0,
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
            ast::Expression::Block(_node) => todo!(),
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
            ast::Expression::Obj(_node) => todo!(),
            ast::Expression::Arr(node) => {
                self.append_instruction(Instruction::Arr(register, node.value.len()));
                let value_register = self.use_register();
                for (index, value) in node.value.iter().enumerate() {
                    self.eval_expr(value_register, value);
                    self.append_instruction(Instruction::StoreImmediate(
                        value_register,
                        register,
                        index,
                    ));
                }
            }
            ast::Expression::Not(node) => {
                let src = self.use_register();
                self.eval_expr(src, &node.expr);
                self.append_instruction(Instruction::Not(register, src));
            }
            ast::Expression::Identifier(_node) => todo!(),
            ast::Expression::Call(_node) => todo!(),
            ast::Expression::Index(_node) => todo!(),
            ast::Expression::Prop(_node) => todo!(),
            ast::Expression::Binary(_node) => todo!(),
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
        // TODO: exprが配列になり得るか解析
        for (_i, item) in dest.value.iter().enumerate() {
            self.define(item, todo!("expr[i]"), is_mutable);
        }
    }

    fn define_obj(&mut self, dest: &'ast ast::Obj, register: Register, is_mutable: bool) {
        // TODO: exprがオブジェクトになり得るか解析
        for (_key, item) in &dest.value {
            self.define(item, todo!("expr[key]"), is_mutable);
        }
    }

    fn assign(&mut self, dest: &'ast ast::Expression, expr_register: Register) {
        match dest {
            ast::Expression::Identifier(dest) => match self.scopes.assign(&dest.name) {
                Ok(dest_register) => {
                    self.append_instruction(Instruction::Move(dest_register, expr_register))
                }
                Err(error) => self.append_instruction(Instruction::Panic(error)),
            },
            ast::Expression::Index(_dest) => todo!(),
            ast::Expression::Prop(_dest) => todo!(),
            ast::Expression::Arr(_dest) => todo!(),
            ast::Expression::Obj(_dest) => todo!(),
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
