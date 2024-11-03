use std::rc::Rc;

use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind, Result};
use aiscript_engine_ir::{DataItem, Instruction, InstructionAddress, Ir, Register, UserFn};
use aiscript_engine_library::NativeFn;
use aiscript_engine_values::{FnIndex, VFn, VObj, Value};
use gc::{Gc, GcCell};
use indexmap::IndexMap;

use crate::utils::{require_array, require_bool, require_function, require_object, GetByF64};

pub enum VmState {
    Exit,
    Continue,
}

pub struct Vm<'ir, 'lib: 'ir> {
    data: Vec<Rc<[u16]>>,
    native_functions: &'ir mut [NativeFn<'lib>],
    user_functions: &'ir [UserFn],
    pc: ProgramCounter<'ir>,
    pc_stack: Vec<ProgramCounter<'ir>>,
    registers: Vec<Value>,
}

impl<'ir, 'lib: 'ir> Vm<'ir, 'lib> {
    pub fn new(ir: &'ir mut Ir<'lib>) -> Self {
        let entry_point = &ir.user_functions[ir.entry_point];
        let register_length = entry_point.register_length;
        let data: Vec<Rc<[u16]>> = ir
            .data
            .iter()
            .map(|DataItem::Str(item)| Rc::from(item.as_u16s()))
            .collect();
        Vm {
            data,
            native_functions: &mut ir.native_functions,
            user_functions: &ir.user_functions,
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
                self.registers[*register] = Value::Str(Rc::clone(&self.data[*index]));
                self.pc.index += 1;
            }
            Instruction::Arr(register, len) => {
                self.registers[*register] =
                    Value::Arr(Gc::new(GcCell::new(vec![Value::Uninitialized; *len])));
                self.pc.index += 1;
            }
            Instruction::Obj(register, n) => {
                self.registers[*register] =
                    Value::Obj(Gc::new(GcCell::new(VObj(IndexMap::with_capacity(*n)))));
                self.pc.index += 1;
            }
            Instruction::NativeFn(register, index) => {
                self.registers[*register] = Value::Fn(Gc::new(GcCell::new(VFn {
                    index: FnIndex::Native(*index),
                    capture: Vec::new(),
                })));
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
                let target = self.registers[*target].clone();
                match target {
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
            Instruction::LoadIndex(register, target, index) => {
                let target = require_array(&self.registers[*target])?;
                if let Some(value) = target.borrow().get(*index) {
                    self.registers[*register] = value.clone();
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
            Instruction::LoadProp(register, target, name) => {
                let target = require_object(&self.registers[*target])?;
                let name = &self.data[*name];
                let value = target.borrow().0.get(name).map(|value| value.clone());
                self.registers[*register] = value.unwrap_or(Value::Null);
                self.pc.index += 1;
            }
            Instruction::Store(register, target, index) => {
                let target = self.registers[*target].clone();
                match target {
                    Value::Arr(target) => {
                        let index_float = self.require_num(*index)?;
                        if let Some(value) = target.borrow_mut().get_mut_by_f64(index_float) {
                            *value = self.registers[*register].clone();
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
            Instruction::StoreIndex(register, target, index) => {
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
            Instruction::StoreProp(register, target, name) => {
                let target = require_object(&self.registers[*target])?;
                let name = Rc::clone(&self.data[*name]);
                target
                    .borrow_mut()
                    .0
                    .insert(name, self.registers[*register].clone());
                self.pc.index += 1;
            }
            Instruction::Call(register, f, args) => {
                let closure = require_function(&self.registers[*f])?;
                let args = require_array(&self.registers[*args])?;
                let capture = closure.borrow().capture.clone();
                match closure.borrow().index {
                    FnIndex::Native(index) => {
                        let function = &mut self.native_functions[index];
                        match function {
                            NativeFn::Static(function) => {
                                self.registers[*register] =
                                    function(args.borrow().clone(), capture)?;
                            }
                            NativeFn::Dynamic(function) => {
                                self.registers[*register] =
                                    function(args.borrow().clone(), capture)?;
                            }
                        };
                    }
                    FnIndex::User(index) => todo!(),
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
