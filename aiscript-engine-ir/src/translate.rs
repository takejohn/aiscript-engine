use aiscript_engine_ast::{self as ast, Expression, NamespaceMember};
use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind, AiScriptError, Result};

use crate::{
    scope::{self, Scope},
    Ir,
};

pub fn translate(ast: &[ast::Node]) -> Ir {
    Ir {
        data: todo!(),
        functions: todo!(),
    }
}

struct Translator;

impl Translator {
    fn collect_ns_member<'sc, 'ast: 'sc>(
        scope: &'sc mut Scope<'sc, 'ast>,
        ns: &'ast ast::Namespace,
    ) -> Result<()> {
        let mut ns_scope = scope.create_child_namespace_scope(&ns.name, scope::ANONYMOUS);

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
