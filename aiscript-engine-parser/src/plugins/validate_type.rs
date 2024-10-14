use aiscript_engine_ast as ast;
use aiscript_engine_common::Result;
use aiscript_engine_types::get_type_by_source;

use super::visit::{RecursiveVisitor, Visitor};

struct NodeValidator;

impl Visitor for NodeValidator {
    fn visit_def(&mut self, node: &mut ast::Definition) -> Result<()> {
        if let Some(var_type) = &node.var_type {
            get_type_by_source(&var_type)?;
        }
        return Ok(());
    }

    fn visit_fn(&mut self, node: &mut ast::Fn) -> Result<()> {
        for arg in &node.args {
            if let Some(arg_type) = &arg.arg_type {
                get_type_by_source(&arg_type)?;
            }
        }
        if let Some(ret_type) = &node.ret_type {
            get_type_by_source(&ret_type)?;
        }
        return Ok(());
    }
}

pub(crate) fn validate_type(nodes: &mut Vec<ast::Node>) -> Result<()> {
    let mut node_validator = NodeValidator;
    let mut validator = RecursiveVisitor::new(&mut node_validator);
    for node in nodes {
        validator.visit(node)?;
    }
    return Ok(());
}
