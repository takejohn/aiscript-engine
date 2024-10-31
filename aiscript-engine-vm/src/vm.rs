use std::rc::Rc;

use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind, Result};
use aiscript_engine_ir::{DataItem, FnIndex, Instruction, InstructionAddress, Ir, Register};
use gc::{Gc, GcCell};

use crate::{
    utils::{require_array, GetByF64},
    values::Value,
};

pub enum VmState {
    Exit,
    Continue,
}

pub struct Vm<'ir> {
    program: &'ir Ir,
    pc: ProgramCounter,
    registers: Vec<Value>,
    stack: Vec<StackFrame>,
}

impl<'ir> Vm<'ir> {
    pub fn new(ir: &'ir Ir) -> Self {
        let register_length = ir.functions[ir.entry_point].register_length;
        Vm {
            program: ir,
            pc: ProgramCounter {
                function: 0,
                instruction: 0,
            },
            registers: vec![Value::Uninitialized; register_length],
            stack: Vec::new(),
        }
    }

    pub fn exec(&mut self) -> Result<()> {
        loop {
            if let VmState::Exit = self.step()? {
                return Ok(());
            }
        }
    }

    pub fn step(&mut self) -> Result<VmState> {
        let function = &self.program.functions[self.pc.function];
        let Some(instruction) = function.instructions.get(self.pc.instruction) else {
            return Ok(VmState::Exit);
        };

        match instruction.to_owned() {
            Instruction::Nop => {
                self.pc.instruction += 1;
            }
            Instruction::Panic(ai_script_basic_error) => {
                return Err(Box::new(ai_script_basic_error))
            }
            Instruction::Null(register) => {
                self.registers[register] = Value::Null;
                self.pc.instruction += 1;
            }
            Instruction::Num(register, value) => {
                self.registers[register] = Value::Num(value);
                self.pc.instruction += 1;
            }
            Instruction::Bool(register, value) => {
                self.registers[register] = Value::Bool(value);
                self.pc.instruction += 1;
            }
            Instruction::Data(register, index) => {
                let DataItem::Str(value) = &self.program.data[index];
                self.registers[register] = Value::Str(Rc::new(value.to_owned()));
                self.pc.instruction += 1;
            }
            Instruction::Arr(register, len) => {
                self.registers[register] =
                    Value::Arr(Gc::new(GcCell::new(vec![Value::Uninitialized; len])));
                self.pc.instruction += 1;
            }
            Instruction::Move(dest, src) => {
                self.registers[dest] = self.registers[src].clone();
                self.pc.instruction += 1;
            }
            Instruction::Add(dest, src) => {
                let left = self.require_num(dest)?;
                let right = self.require_num(src)?;
                self.registers[dest] = Value::Num(left + right);
                self.pc.instruction += 1;
            }
            Instruction::Sub(dest, src) => {
                let left = self.require_num(dest)?;
                let right = self.require_num(src)?;
                self.registers[dest] = Value::Num(left - right);
                self.pc.instruction += 1;
            }
            Instruction::Load(register, target, index) => {
                let dest = self.registers[target].clone();
                match dest {
                    Value::Arr(target) => {
                        let index_float = self.require_num(index)?;
                        if let Some(value) = target.as_ref().borrow().get_by_f64(index_float) {
                            let value = value.clone();
                            self.registers[register] = value;
                        } else {
                            return Err(Box::new(AiScriptBasicError::new(
                                AiScriptBasicErrorKind::Runtime,
                                format!(
                                    "Index out of range. index: {} max: {}",
                                    index_float,
                                    target.as_ref().borrow().len() - 1
                                ),
                                None,
                            )));
                        }
                    }
                    Value::Obj(_) => todo!(),
                    _ => todo!(),
                }
                self.pc.instruction += 1;
            }
            Instruction::StoreImmediate(register, target, index) => {
                let target = require_array(self.registers[target].clone())?;
                if let Some(ptr) = target.borrow_mut().get_mut(index) {
                    *ptr = self.registers[register].clone();
                } else {
                    return Err(Box::new(AiScriptBasicError::new(
                        AiScriptBasicErrorKind::Runtime,
                        format!(
                            "Index out of range. index: {} max: {}",
                            index,
                            target.as_ref().borrow().len() - 1
                        ),
                        None,
                    )));
                }
                self.pc.instruction += 1;
            }
        }

        return Ok(VmState::Continue);
    }

    pub fn registers(&self) -> &[Value] {
        &self.registers
    }

    fn require_num(&self, register: Register) -> Result<f64> {
        let value = &self.registers[register];
        match value {
            Value::Num(value) => Ok(*value),
            _ => Err(Box::new(AiScriptBasicError::new(
                AiScriptBasicErrorKind::Runtime,
                format!("Expect number, but got {}.", value.type_name()),
                None,
            ))),
        }
    }
}

struct ProgramCounter {
    function: FnIndex,
    instruction: InstructionAddress,
}

struct StackFrame {
    pub(super) return_address: FnIndex,
    pub(super) values: Vec<Value>,
}
