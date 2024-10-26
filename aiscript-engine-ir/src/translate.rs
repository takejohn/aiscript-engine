use aiscript_engine_ast::{self as ast, NamespaceMember};
use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind};

use crate::{
    scopes::{Scopes, Variable},
    value::Value,
    Block, Instruction, Ir,
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
                self.define_identifier(dest, Value, node.is_mut);
            }
        }

        self.scopes.drop_local_scope();
    }

    fn define(&mut self, dest: &'ast ast::Expression, value: Value, is_mutable: bool) {
        match dest {
            ast::Expression::Identifier(dest) => self.define_identifier(dest, value, is_mutable),
            ast::Expression::Arr(dest) => self.define_arr(dest, value, is_mutable),
            ast::Expression::Obj(dest) => self.define_obj(dest, value, is_mutable),
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

    fn define_identifier(&mut self, dest: &'ast ast::Identifier, value: Value, is_mutable: bool) {
        self.block.register_length += 1;
        self.scopes.add(&dest.name, Variable { is_mutable, value });
    }

    fn define_arr(&mut self, dest: &'ast ast::Arr, value: Value, is_mutable: bool) {
        // TODO: valueが配列になり得るか解析
        for item in &dest.value {
            self.define(item, Value, is_mutable);
        }
    }

    fn define_obj(&mut self, dest: &'ast ast::Obj, value: Value, is_mutable: bool) {
        // TODO: valueがオブジェクトになり得るか解析
        for (_key, item) in &dest.value {
            self.define(item, Value, is_mutable);
        }
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
                    instructions: Vec::new()
                },
            }
        )
    }
}
