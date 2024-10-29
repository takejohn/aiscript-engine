use aiscript_engine_ast::{self as ast, NamespaceMember};
use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind};

use crate::{
    scopes::{Scopes, Variable},
    Block, Instruction, Ir, Register,
};

pub fn translate(ast: &[ast::Node]) -> Ir {
    Translator::new().translate(ast)
}

struct Translator<'ast> {
    scopes: Scopes<'ast>,
    block: Block,
}

impl<'ast> Translator<'ast> {
    fn new() -> Self {
        Translator {
            scopes: Scopes::new(),
            block: Block::new(),
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
        // TODO: 名前空間以外の解析
        self.build()
    }

    fn build(self) -> Ir {
        Ir {
            data: Vec::new(),
            functions: Vec::new(),
            entry_point: self.block,
        }
    }

    fn collect_ns(&mut self, namespaces: impl IntoIterator<Item = &'ast ast::Namespace>) {
        for ns in namespaces {
            self.collect_ns_member(ns);
        }
    }

    fn collect_ns_member(&mut self, ns: &'ast ast::Namespace) {
        self.scopes.push_namespace_scope(&ns.name);

        self.collect_ns(ns.members.iter().filter_map(|member| match member {
            NamespaceMember::Ns(node) => Some(node),
            NamespaceMember::Def(_) => None,
        }));

        for node in &ns.members {
            if let NamespaceMember::Def(node) = node {
                let ast::Expression::Identifier(dest) = &node.dest else {
                    self.block
                        .instructions
                        .push(Instruction::Panic(AiScriptBasicError::new(
                            AiScriptBasicErrorKind::Namespace,
                            "Destructuring assignment is invalid in namespace declarations.",
                            Some(node.loc.start.clone()),
                        )));
                    self.scopes.drop_local_scope();
                    return;
                };
                if node.is_mut {
                    self.block
                        .instructions
                        .push(Instruction::Panic(AiScriptBasicError::new(
                            AiScriptBasicErrorKind::Namespace,
                            String::from("No \"var\" in namespace declaration: ")
                                + &dest.name.to_string(),
                            Some(node.loc.start.clone()),
                        )));
                    self.scopes.drop_local_scope();
                    return;
                }

                // TODO: node.exprの解析
                self.define_identifier(dest, &node.expr, node.is_mut);
            }
        }

        self.scopes.drop_local_scope();
    }

    fn eval(&mut self, register: Register, node: &'ast ast::Node) {
        match node {
            ast::Node::Ns(_) | ast::Node::Meta(_) => {
                self.append_instruction(Instruction::Null(register))
            }
            ast::Node::Attr(node) => todo!(),
            ast::Node::Statement(node) => todo!(),
            ast::Node::Expr(node) => self.eval_expr(register, node),
            ast::Node::TypeSource(_) => panic!("invalid node type"),
        }
    }

    fn eval_expr(&mut self, register: Register, node: &'ast ast::Expression) {
        match node {
            ast::Expression::If(node) => todo!(),
            ast::Expression::Fn(node) => todo!(),
            ast::Expression::Match(node) => todo!(),
            ast::Expression::Block(node) => todo!(),
            ast::Expression::Exists(node) => todo!(),
            ast::Expression::Tmpl(node) => todo!(),
            ast::Expression::Str(node) => todo!(),
            ast::Expression::Num(node) => {
                self.append_instruction(Instruction::Num(register, node.value));
            }
            ast::Expression::Bool(node) => todo!(),
            ast::Expression::Null(node) => {
                self.append_instruction(Instruction::Null(register));
            }
            ast::Expression::Obj(node) => todo!(),
            ast::Expression::Arr(node) => todo!(),
            ast::Expression::Not(node) => todo!(),
            ast::Expression::Identifier(node) => todo!(),
            ast::Expression::Call(node) => todo!(),
            ast::Expression::Index(node) => todo!(),
            ast::Expression::Prop(node) => todo!(),
            ast::Expression::Binary(node) => todo!(),
        }
    }

    fn define(
        &mut self,
        dest: &'ast ast::Expression,
        expr: &'ast ast::Expression,
        is_mutable: bool,
    ) {
        match dest {
            ast::Expression::Identifier(dest) => self.define_identifier(dest, expr, is_mutable),
            ast::Expression::Arr(dest) => self.define_arr(dest, expr, is_mutable),
            ast::Expression::Obj(dest) => self.define_obj(dest, expr, is_mutable),
            _ => {
                self.block
                    .instructions
                    .push(Instruction::Panic(AiScriptBasicError::new(
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
        expr: &'ast ast::Expression,
        is_mutable: bool,
    ) {
        let register = self.block.new_register();
        self.eval_expr(register, expr);
        self.scopes.add(
            &dest.name,
            Variable {
                is_mutable,
                register,
            },
        );
    }

    fn define_arr(&mut self, dest: &'ast ast::Arr, expr: &'ast ast::Expression, is_mutable: bool) {
        // TODO: exprが配列になり得るか解析
        for (_i, item) in dest.value.iter().enumerate() {
            self.define(item, todo!("expr[i]"), is_mutable);
        }
    }

    fn define_obj(&mut self, dest: &'ast ast::Obj, expr: &'ast ast::Expression, is_mutable: bool) {
        // TODO: exprがオブジェクトになり得るか解析
        for (_key, item) in &dest.value {
            self.define(item, todo!("expr[key]"), is_mutable);
        }
    }

    fn append_instruction(&mut self, instruction: Instruction) {
        self.block.instructions.push(instruction);
    }
}

#[cfg(test)]
mod tests {
    use aiscript_engine_common::Utf16String;
    use aiscript_engine_parser::Parser;
    use pretty_assertions::assert_eq;

    use crate::Block;

    use super::*;

    fn to_ir(source: &str) -> Ir {
        let mut parser = Parser::new();
        let ast = parser.parse(&Utf16String::from(source)).unwrap();
        return translate(&ast);
    }

    #[test]
    fn namespace() {
        let ir = to_ir(
            r#"
                :: Ns {
                    let a = 0
                }
            "#,
        );
        assert_eq!(
            ir,
            Ir {
                data: Vec::new(),
                functions: Vec::new(),
                entry_point: Block {
                    register_length: 1,
                    instructions: vec![Instruction::Num(0, 0.0),]
                },
            }
        )
    }
}
