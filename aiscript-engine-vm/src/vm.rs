use aiscript_engine_common::Result;
use aiscript_engine_ir::{FnIndex, InstructionAddress, Ir};

use crate::values::Value;

pub enum VmState {
    Exit,
    Continue,
}

pub struct Vm<'gc, 'ir: 'gc> {
    program: &'ir Ir,
    pc: ProgramCounter,
    stack: Vec<StackFrame<'gc>>,
}

impl<'gc, 'ir: 'gc> Vm<'gc, 'ir> {
    pub fn new(ir: &'ir Ir) -> Self {
        Vm {
            program: ir,
            pc: ProgramCounter {
                function: 0,
                instruction: 0,
            },
            stack: Vec::new(),
        }
    }

    pub fn step(&mut self) -> Result<VmState> {
        let Some(function) = self.program.functions.get(self.pc.function) else {
            return Ok(VmState::Exit);
        };
        let Some(instruction) = function.instructions.get(self.pc.instruction) else {
            return Ok(VmState::Exit);
        };

        match instruction.to_owned() {
            aiscript_engine_ir::Instruction::Nop => Ok(VmState::Continue),
            aiscript_engine_ir::Instruction::Panic(ai_script_basic_error) => {
                Err(Box::new(ai_script_basic_error))
            }
            _ => todo!(),
        }
    }
}

struct ProgramCounter {
    function: FnIndex,
    instruction: InstructionAddress,
}

struct StackFrame<'gc> {
    pub(super) return_address: FnIndex,
    pub(super) values: Vec<Value<'gc>>,
}
