use std::rc::Rc;

use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind, Result};
use aiscript_engine_ir::{DataItem, Instruction, InstructionAddress, Ir, Register};
use gc::{Gc, GcCell};

use crate::{
    utils::{require_array, require_bool, GetByF64},
    values::Value,
};

pub enum VmState {
    Exit,
    Continue,
}

pub struct Vm<'ir> {
    program: &'ir Ir,
    pc: ProgramCounter<'ir>,
    pc_stack: Vec<ProgramCounter<'ir>>,
    registers: Vec<Value>,
}

impl<'ir> Vm<'ir> {
    pub fn new(ir: &'ir Ir) -> Self {
        let entry_point = &ir.functions[ir.entry_point];
        let register_length = entry_point.register_length;
        Vm {
            program: ir,
            pc: ProgramCounter {
                instructions: &entry_point.instructions,
                index: 0,
            },
            pc_stack: Vec::new(),
            registers: vec![Value::Uninitialized; register_length],
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
        let Some(instruction) = self.get_instruction() else {
            return Ok(VmState::Exit);
        };

        match instruction {
            Instruction::Nop => {
                self.pc.index += 1;
            }
            Instruction::Panic(ai_script_basic_error) => {
                return Err(Box::new(ai_script_basic_error.to_owned()))
            }
            Instruction::If(cond, then_code, else_code) => {
                let cond = require_bool(&self.registers[*cond])?;
                self.pc.index += 1;
                if cond {
                    self.pc_stack.push(std::mem::replace(
                        &mut self.pc,
                        ProgramCounter {
                            instructions: &then_code,
                            index: 0,
                        },
                    ));
                } else {
                    self.pc_stack.push(std::mem::replace(
                        &mut self.pc,
                        ProgramCounter {
                            instructions: &else_code,
                            index: 0,
                        },
                    ));
                }
            }
            Instruction::Null(register) => {
                self.registers[*register] = Value::Null;
                self.pc.index += 1;
            }
            Instruction::Num(register, value) => {
                self.registers[*register] = Value::Num(*value);
                self.pc.index += 1;
            }
            Instruction::Bool(register, value) => {
                self.registers[*register] = Value::Bool(*value);
                self.pc.index += 1;
            }
            Instruction::Data(register, index) => {
                let DataItem::Str(value) = &self.program.data[*index];
                self.registers[*register] = Value::Str(Rc::new(value.to_owned()));
                self.pc.index += 1;
            }
            Instruction::Arr(register, len) => {
                self.registers[*register] =
                    Value::Arr(Gc::new(GcCell::new(vec![Value::Uninitialized; *len])));
                self.pc.index += 1;
            }
            Instruction::Move(dest, src) => {
                self.registers[*dest] = self.registers[*src].clone();
                self.pc.index += 1;
            }
            Instruction::Add(dest, src) => {
                let dest = *dest;
                let left = self.require_num(dest)?;
                let right = self.require_num(*src)?;
                self.registers[dest] = Value::Num(left + right);
                self.pc.index += 1;
            }
            Instruction::Sub(dest, src) => {
                let dest = *dest;
                let left = self.require_num(dest)?;
                let right = self.require_num(*src)?;
                self.registers[dest] = Value::Num(left - right);
                self.pc.index += 1;
            }
            Instruction::Not(dest, src) => {
                let src = require_bool(&self.registers[*src])?;
                self.registers[*dest] = Value::Bool(!src);
                self.pc.index += 1;
            }
            Instruction::Load(register, target, index) => {
                let dest = self.registers[*target].clone();
                match dest {
                    Value::Arr(target) => {
                        let index_float = self.require_num(*index)?;
                        if let Some(value) = target.as_ref().borrow().get_by_f64(index_float) {
                            let value = value.clone();
                            self.registers[*register] = value;
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
                self.pc.index += 1;
            }
            Instruction::StoreImmediate(register, target, index) => {
                let target = require_array(&self.registers[*target])?;
                if let Some(ptr) = target.borrow_mut().get_mut(*index) {
                    *ptr = self.registers[*register].clone();
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
                self.pc.index += 1;
            }
        }

        return Ok(VmState::Continue);
    }

    pub fn registers(&self) -> &[Value] {
        &self.registers
    }

    fn get_instruction(&mut self) -> Option<&'ir Instruction> {
        if let Some(instruction) = self.pc.get() {
            return Some(instruction);
        }
        if let Some(pc) = self.pc_stack.pop() {
            self.pc = pc;
            return self.pc.get();
        }
        return None;
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

struct ProgramCounter<'ir> {
    instructions: &'ir [Instruction],
    index: InstructionAddress,
}

impl<'ir> ProgramCounter<'ir> {
    fn get(&self) -> Option<&'ir Instruction> {
        self.instructions.get(self.index)
    }
}
