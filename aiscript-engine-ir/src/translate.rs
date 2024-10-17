
use aiscript_engine_ast as ast;

use crate::{scope::{self, Scope}, Ir};

pub fn translate(ast: &[ast::Node]) -> Ir {
    Ir {
        data: todo!(),
        functions: todo!(),
    }
}

struct Translator<'src> {
    scopes: Scope<'src>,
}

impl<'src> Translator<'src> {
    fn collect_ns(&mut self, ast: &'src[ast::Node], scope: &'src mut Scope) {
        for node in ast {
            if let ast::Node::Ns(ns) = node {
                todo!()
            }
        }
    }

    fn collect_ns_member(&mut self, ns: &'src ast::Namespace, scope: &'src mut Scope<'src>) {
        let ns_scope = scope.create_child_namespace_scope(&ns.name, scope::ANONYMOUS);
        // self.collect_ns(ns.members, &mut ns_scopescope);
    }
}
