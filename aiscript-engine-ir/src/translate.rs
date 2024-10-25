use aiscript_engine_ast::{self as ast, Expression, NamespaceMember};
use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind, Result};

use crate::{scopes::Scopes, Ir};

pub fn translate(ast: &[ast::Node]) -> Ir {
    Ir {
        data: todo!(),
        functions: todo!(),
    }
}

struct Translator<'ast> {
    scopes: Scopes<'ast>,
}

impl<'ast> Translator<'ast> {
    fn collect_ns_member(&mut self, ns: &'ast ast::Namespace) -> Result<()> {
        let mut ns_scope = self.scopes.push_namespace_scope(&ns.name);

        for node in &ns.members {
            if let NamespaceMember::Ns(def) = node {
                // collect_ns
            }
        }

        for node in &ns.members {
            if let NamespaceMember::Def(node) = node {
                let Expression::Identifier(dest) = &node.dest else {
                    return Err(Box::new(AiScriptBasicError::new(
                        AiScriptBasicErrorKind::Namespace,
                        "Destructuring assignment is invalid in namespace declarations.",
                        Some(node.loc.start.clone()),
                    )));
                };
                if node.is_mut {
                    return Err(Box::new(AiScriptBasicError::new(
                        AiScriptBasicErrorKind::Namespace,
                        String::from("No \"var\" in namespace declaration: ")
                            + &dest.name.to_string(),
                        Some(node.loc.start.clone()),
                    )));
                }

                // TODO: node.exprを評価してns_scopeに定義
            }
        }

        Ok(())
    }
}
