use std::rc::Rc;

use aiscript_engine_common::Utf16String;
use aiscript_engine_ir::{DataItem, Instruction, Ir, UserFn};
use aiscript_engine_vm::{Value, Vm};
use utf16_literal::utf16;

#[test]
fn const_null() {
    let mut ir = Ir {
        data: Vec::new(),
        native_functions: Vec::new(),
        user_functions: vec![UserFn {
            register_length: 1,
            instructions: vec![Instruction::Null(0)],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&mut ir);
    vm.exec().unwrap();
    assert_eq!(vm.registers()[0], Value::Null);
}

#[test]
fn const_num() {
    let mut ir = Ir {
        data: Vec::new(),
        native_functions: Vec::new(),
        user_functions: vec![UserFn {
            register_length: 1,
            instructions: vec![Instruction::Num(0, 42.0)],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&mut ir);
    vm.exec().unwrap();
    assert_eq!(vm.registers()[0], Value::Num(42.0));
}

#[test]
fn const_bool() {
    let mut ir = Ir {
        data: Vec::new(),
        native_functions: Vec::new(),
        user_functions: vec![UserFn {
            register_length: 1,
            instructions: vec![Instruction::Bool(0, true)],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&mut ir);
    vm.exec().unwrap();
    assert_eq!(vm.registers()[0], Value::Bool(true));
}

#[test]
fn const_str() {
    let mut ir = Ir {
        data: vec![DataItem::Str(Utf16String::from("Hello, world!"))],
        native_functions: Vec::new(),
        user_functions: vec![UserFn {
            register_length: 1,
            instructions: vec![Instruction::Data(0, 0)],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&mut ir);
    vm.exec().unwrap();
    assert_eq!(
        vm.registers()[0],
        Value::Str(Rc::from(&utf16!("Hello, world!") as &[_]))
    );
}
