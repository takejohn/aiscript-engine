//! AiScript interpreter

use aiscript_engine_ast as ast;
use aiscript_engine_common::{Result, Utf16Str};

use crate::{values::Value, variable::Variable};

const IRQ_RATE: usize = 300;
const IRQ_AT: usize = IRQ_RATE - 1;

pub struct LogObject<'a> {
    scope: Option<&'a Utf16Str>,
    var: Option<&'a Utf16Str>,
    val: LogObjectVal<'a>,
}

pub enum LogObjectVal<'gc> {
    Value(Value<'gc>),
    Variable(Variable<'gc>),
}

pub struct Interpreter {
    // step_count: usize,
    // stop: bool,
    // scope: Scope<'gc>
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
        }
    }

    fn run(&self, program: &[ast::Node]) -> Result<Value> {
        // block:enter

        let mut v: Value = Value::Null;

        for node in program {
            v = self.eval(node)?;
            match v {
                Value::Return(_) => {
                    // block:return
                    return Ok(v);
                }
                Value::Break => {
                    // block:return
                    return Ok(v);
                }
                Value::Continue => {
                    // block:continue
                    return Ok(v);
                }
                _ => {}
            }
        }

        // block:leave
        return Ok(v);
    }

    fn eval(&self, node: &ast::Node) -> Result<Value> {
        match node {
            aiscript_engine_ast::Node::Ns(namespace) => todo!(),
            aiscript_engine_ast::Node::Meta(meta) => todo!(),
            aiscript_engine_ast::Node::TypeSource(type_source) => todo!(),
            aiscript_engine_ast::Node::Attr(attribute) => todo!(),
            aiscript_engine_ast::Node::Statement(statement) => todo!(),
            aiscript_engine_ast::Node::Expr(expression) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let interpreter = Interpreter::new();
        assert!(matches!(interpreter.run(&[] as &[ast::Node]), Ok(Value::Null)));
    }
}
