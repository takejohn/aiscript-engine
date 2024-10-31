use std::borrow::Cow;

use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind, Result};
use aiscript_engine_ir::{DataItem, FnIndex, Instruction, InstructionAddress, Ir, Register};

use crate::{utils::GetByF64, values::Value};

pub enum VmState {
    Exit,
    Continue,
}

pub struct Vm<'gc, 'ir: 'gc> {
    program: &'ir Ir,
    pc: ProgramCounter,
    registers: Vec<Value<'gc>>,
    stack: Vec<StackFrame<'gc>>,
}

impl<'gc, 'ir: 'gc> Vm<'gc, 'ir> {
    pub fn new(ir: &'ir Ir) -> Self {
        let register_length = ir.functions[ir.entry_point].register_length;
        Vm {
            program: ir,
            pc: ProgramCounter {
                function: 0,
                instruction: 0,
            },
            registers: vec![Value::Null; register_length],
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
                self.registers[register] = Value::Str(Cow::Borrowed(value));
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
                let dest = &self.registers[target];
                match dest {
                    Value::Arr(target) => {
                        let target = *target;
                        let index_float = self.require_num(index)?;
                        if let Some(value) = target.get_by_f64(index_float) {
                            self.registers[register] = value.clone();
                        } else {
                            return Err(Box::new(AiScriptBasicError::new(
                                AiScriptBasicErrorKind::Runtime,
                                format!(
                                    "Index out of range. index: {} max: {}",
                                    index_float,
                                    target.len() - 1
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
        }

        return Ok(VmState::Continue);
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

struct StackFrame<'gc> {
    pub(super) return_address: FnIndex,
    pub(super) values: Vec<Value<'gc>>,
}

#[cfg(test)]
mod tests {
    use aiscript_engine_common::{Utf16Str, Utf16String};
    use aiscript_engine_ir::Procedure;
    use utf16_literal::utf16;

    use super::*;

    #[test]
    fn empty() {
        let ir = Ir::default();
        let mut vm = Vm::new(&ir);
        assert!(matches!(vm.step().unwrap(), VmState::Exit));
    }

    #[test]
    fn nop() {
        let ir = Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 1,
                instructions: vec![Instruction::Nop],
            }],
            entry_point: 0,
        };
        let mut vm = Vm::new(&ir);
        assert!(matches!(vm.step().unwrap(), VmState::Continue));
        assert!(matches!(vm.step().unwrap(), VmState::Exit));
    }

    #[test]
    fn panics() {
        let ir = Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 1,
                instructions: vec![Instruction::Panic(AiScriptBasicError::new(
                    AiScriptBasicErrorKind::Runtime,
                    "abort",
                    None,
                ))],
            }],
            entry_point: 0,
        };
        let mut vm = Vm::new(&ir);
        assert!(vm.step().is_err());
    }

    #[test]
    fn const_null() {
        let ir = Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 1,
                instructions: vec![Instruction::Null(0)],
            }],
            entry_point: 0,
        };
        let mut vm = Vm::new(&ir);
        vm.exec().unwrap();
        assert_eq!(vm.registers[0], Value::Null);
    }

    #[test]
    fn const_num() {
        let ir = Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 1,
                instructions: vec![Instruction::Num(0, 42.0)],
            }],
            entry_point: 0,
        };
        let mut vm = Vm::new(&ir);
        vm.exec().unwrap();
        assert_eq!(vm.registers[0], Value::Num(42.0));
    }

    #[test]
    fn const_bool() {
        let ir = Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 1,
                instructions: vec![Instruction::Bool(0, true)],
            }],
            entry_point: 0,
        };
        let mut vm = Vm::new(&ir);
        vm.exec().unwrap();
        assert_eq!(vm.registers[0], Value::Bool(true));
    }

    #[test]
    fn const_str() {
        let ir = Ir {
            data: vec![DataItem::Str(Utf16String::from("Hello, world!"))],
            functions: vec![Procedure {
                register_length: 1,
                instructions: vec![Instruction::Data(0, 0)],
            }],
            entry_point: 0,
        };
        let mut vm = Vm::new(&ir);
        vm.exec().unwrap();
        assert_eq!(
            vm.registers[0],
            Value::Str(Cow::Borrowed(Utf16Str::new(&utf16!("Hello, world!"))))
        );
    }

    #[test]
    fn assign() {
        let ir = Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 2,
                instructions: vec![Instruction::Num(0, 42.0), Instruction::Move(1, 0)],
            }],
            entry_point: 0,
        };
        let mut vm = Vm::new(&ir);
        vm.exec().unwrap();
        assert_eq!(vm.registers[0], Value::Num(42.0));
        assert_eq!(vm.registers[1], Value::Num(42.0));
    }

    #[test]
    fn add_assign() {
        let ir = Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 2,
                instructions: vec![
                    Instruction::Num(0, 1.0),
                    Instruction::Num(1, 2.0),
                    Instruction::Add(1, 0),
                ],
            }],
            entry_point: 0,
        };
        let mut vm = Vm::new(&ir);
        vm.exec().unwrap();
        assert_eq!(vm.registers[0], Value::Num(1.0));
        assert_eq!(vm.registers[1], Value::Num(3.0));
    }

    #[test]
    fn sub_assign() {
        let ir = Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 2,
                instructions: vec![
                    Instruction::Num(0, 1.0),
                    Instruction::Num(1, 3.0),
                    Instruction::Sub(1, 0),
                ],
            }],
            entry_point: 0,
        };
        let mut vm = Vm::new(&ir);
        vm.exec().unwrap();
        assert_eq!(vm.registers[0], Value::Num(1.0));
        assert_eq!(vm.registers[1], Value::Num(2.0));
    }
}
