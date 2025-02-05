use std::{
    ops::{Index, IndexMut},
    rc::Rc,
};

use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind, Result};
use aiscript_engine_values::{
    require_array, require_boolean, require_function, require_number, require_object, FnIndex, VFn,
    Value,
};
use gc::{Gc, GcCell};

use super::utils::GetByF64;
use crate::ir::{Instruction, Register, UserFn};
use crate::library::NativeFn;

struct Registers {
    registers: Vec<Value>,
}

impl Registers {
    fn new(len: usize) -> Self {
        Registers {
            registers: vec![Value::Uninitialized; len],
        }
    }
}

impl Index<Register> for Registers {
    type Output = Value;

    fn index(&self, index: Register) -> &Self::Output {
        &self.registers[index]
    }
}

impl IndexMut<Register> for Registers {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.registers[index]
    }
}

pub(crate) struct Vm {
    native_functions: Vec<NativeFn>,
}

impl Vm {
    pub(crate) fn new() -> Self {
        Vm {
            native_functions: Vec::new(),
        }
    }

    pub(crate) fn register_native_fn(&mut self, native_fn: NativeFn) {
        self.native_functions.push(native_fn);
    }

    pub(crate) fn exec(&mut self, entry_point: &UserFn) -> Result<()> {
        self.exec_instructions(
            &entry_point.instructions,
            &mut Registers::new(entry_point.register_length),
        )
    }

    fn exec_instructions(
        &mut self,
        instructions: &[Instruction],
        registers: &mut Registers,
    ) -> Result<()> {
        for instruction in instructions {
            self.step(&instruction, registers)?;
        }
        Ok(())
    }

    fn step(&mut self, instruction: &Instruction, registers: &mut Registers) -> Result<()> {
        match instruction {
            Instruction::Nop => {}
            Instruction::Panic(ai_script_basic_error) => {
                return Err(Box::new(ai_script_basic_error.to_owned()))
            }
            Instruction::If(cond, then_code, else_code) => {
                let cond = require_boolean(&registers[*cond])?;
                if cond {
                    self.exec_instructions(then_code, registers)?;
                } else {
                    self.exec_instructions(else_code, registers)?;
                }
            }
            Instruction::Null(register) => {
                registers[*register] = Value::Null;
            }
            Instruction::Num(register, value) => {
                registers[*register] = Value::Num(*value);
            }
            Instruction::Bool(register, value) => {
                registers[*register] = Value::Bool(*value);
            }
            Instruction::Str(register, value) => {
                registers[*register] = Value::Str(Rc::clone(value));
            }
            Instruction::Arr(register, value) => {
                registers[*register] = Value::Arr(Gc::clone(value));
            }
            Instruction::Obj(register, value) => {
                registers[*register] = Value::Obj(Gc::clone(value));
            }
            Instruction::NativeFn(register, index) => {
                registers[*register] = Value::Fn(Gc::new(GcCell::new(VFn {
                    index: FnIndex::Native(*index),
                    capture: Vec::new(),
                })));
            }
            Instruction::Move(dest, src) => {
                registers[*dest] = registers[*src].clone();
            }
            Instruction::Add(dest, left, right) => {
                let left = require_number(&registers[*left])?;
                let right = require_number(&registers[*right])?;
                registers[*dest] = Value::Num(left + right);
            }
            Instruction::Sub(dest, left, right) => {
                let left = require_number(&registers[*left])?;
                let right = require_number(&registers[*right])?;
                registers[*dest] = Value::Num(left - right);
            }
            Instruction::Not(dest, src) => {
                let src = require_boolean(&registers[*src])?;
                registers[*dest] = Value::Bool(!src);
            }
            Instruction::Load(register, target, index) => {
                let target = registers[*target].clone();
                match target {
                    Value::Arr(target) => {
                        let index_float = require_number(&registers[*index])?;
                        if let Some(value) = target.as_ref().borrow().get_by_f64(index_float) {
                            let value = value.clone();
                            registers[*register] = value;
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
            }
            Instruction::LoadIndex(register, target, index) => {
                let target = require_array(&registers[*target])?;
                if let Some(value) = target.borrow().get(*index) {
                    registers[*register] = value.clone();
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
                };
            }
            Instruction::LoadProp(register, target, name) => {
                let target = require_object(&registers[*target])?;
                let value = target.borrow().0.get(name).map(|value| value.clone());
                registers[*register] = value.unwrap_or(Value::Null);
            }
            Instruction::Store(register, target, index) => {
                let target = registers[*target].clone();
                match target {
                    Value::Arr(target) => {
                        let index_float = require_number(&registers[*index])?;
                        if let Some(value) = target.borrow_mut().get_mut_by_f64(index_float) {
                            *value = registers[*register].clone();
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
            }
            Instruction::StoreIndex(register, target, index) => {
                let target = require_array(&registers[*target])?;
                if let Some(ptr) = target.borrow_mut().get_mut(*index) {
                    *ptr = registers[*register].clone();
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
                };
            }
            Instruction::StoreProp(register, target, name) => {
                let target = require_object(&registers[*target])?;
                target
                    .borrow_mut()
                    .0
                    .insert(Rc::clone(name), registers[*register].clone());
            }
            Instruction::Call(register, f, args) => {
                let closure = require_function(&registers[*f])?;
                let args = require_array(&registers[*args])?;
                let capture = closure.borrow().capture.clone();
                match closure.borrow().index {
                    FnIndex::Native(index) => {
                        let function = &self.native_functions[index];
                        match function {
                            NativeFn::Static(function) => {
                                registers[*register] = function(args.borrow().clone(), self)?;
                            }
                            NativeFn::Dynamic(function) => {
                                registers[*register] =
                                    Rc::clone(function)(args.borrow().clone(), self)?;
                            }
                        };
                    }
                    FnIndex::User(index) => todo!(),
                };
            }
        }

        return Ok(());
    }
}

impl Default for Vm {
    fn default() -> Self {
        Vm {
            native_functions: Vec::new(),
        }
    }
}
