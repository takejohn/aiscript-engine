//! AiScript interpreter

use aiscript_engine_ast as ast;
use aiscript_engine_common::Result;
use aiscript_engine_ir::translate;
use aiscript_engine_vm::{Value, Vm, VmState};

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter
    }

    pub fn run(&mut self, program: &[ast::Node]) -> Result<Value> {
        let ir = translate(program);
        let mut vm = Vm::new(ir);
        while let VmState::Continue = vm.step()? {
            // nop
        }
        return Ok(Value::Null);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut interpreter = Interpreter::new();
        assert!(matches!(
            interpreter.run(&[] as &[ast::Node]),
            Ok(Value::Null)
        ));
    }
}
