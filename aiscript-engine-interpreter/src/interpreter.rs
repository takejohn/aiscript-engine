//! AiScript interpreter

use aiscript_engine_ast as ast;
use aiscript_engine_common::Result;
use aiscript_engine_ir::translate;
use aiscript_engine_vm::{Value, Vm, VmState};

const IRQ_RATE: usize = 300;
const IRQ_AT: usize = IRQ_RATE - 1;

pub struct Interpreter {
    step_count: usize,
    // stop: bool,
    // scope: Scope<'gc>
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { step_count: 0 }
    }

    fn run(&mut self, program: &[ast::Node]) -> Result<Value> {
        let ir = translate(program);
        let mut vm = Vm::new(ir);
        while let VmState::Continue = vm.step()? {
            self.step_count += 1;
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
