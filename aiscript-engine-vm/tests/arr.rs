use aiscript_engine_ir::{Instruction, Ir, UserFn};
use aiscript_engine_vm::{Value, Vm};

#[test]
fn arr_literal() {
    let mut ir = Ir {
        data: Vec::new(),
        native_functions: Vec::new(),
        user_functions: vec![UserFn {
            register_length: 4,
            instructions: vec![
                Instruction::Num(1, 42.0),
                Instruction::Bool(2, true),
                Instruction::Null(3),
                Instruction::Arr(0, 3),
                Instruction::StoreIndex(1, 0, 0),
                Instruction::StoreIndex(2, 0, 1),
                Instruction::StoreIndex(3, 0, 2),
            ],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&mut ir);
    vm.exec().unwrap();
    let Value::Arr(arr) = &vm.registers()[0] else {
        panic!();
    };
    let arr = arr.borrow();
    assert_eq!(arr[0], Value::Num(42.0));
    assert_eq!(arr[1], Value::Bool(true));
    assert_eq!(arr[2], Value::Null);
}

#[test]
fn store_and_load_index() {
    let mut ir = Ir {
        data: Vec::new(),
        native_functions: Vec::new(),
        user_functions: vec![UserFn {
            register_length: 2,
            instructions: vec![
                Instruction::Arr(0, 1),
                Instruction::Num(1, 42.0),
                Instruction::StoreIndex(1, 0, 0),
                Instruction::LoadIndex(0, 0, 0),
            ],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&mut ir);
    vm.exec().unwrap();
    assert_eq!(vm.registers()[0], Value::Num(42.0));
}

#[test]
fn store_and_load() {
    let mut ir = Ir {
        data: Vec::new(),
        native_functions: Vec::new(),
        user_functions: vec![UserFn {
            register_length: 3,
            instructions: vec![
                Instruction::Arr(0, 1),
                Instruction::Num(1, 42.0),
                Instruction::Num(2, 0.0),
                Instruction::Store(1, 0, 2),
                Instruction::Load(0, 0, 2),
            ],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&mut ir);
    vm.exec().unwrap();
    assert_eq!(vm.registers()[0], Value::Num(42.0));
}
