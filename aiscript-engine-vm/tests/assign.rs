use aiscript_engine_ir::{Instruction, Ir, UserFn};
use aiscript_engine_vm::{Value, Vm};

#[test]
fn assign() {
    let mut ir = Ir {
        data: Vec::new(),
        native_functions: Vec::new(),
        user_functions: vec![UserFn {
            register_length: 2,
            instructions: vec![Instruction::Num(0, 42.0), Instruction::Move(1, 0)],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&mut ir);
    vm.exec().unwrap();
    assert_eq!(vm.registers()[0], Value::Num(42.0));
    assert_eq!(vm.registers()[1], Value::Num(42.0));
}

#[test]
fn add_assign() {
    let mut ir = Ir {
        data: Vec::new(),
        native_functions: Vec::new(),
        user_functions: vec![UserFn {
            register_length: 2,
            instructions: vec![
                Instruction::Num(0, 1.0),
                Instruction::Num(1, 2.0),
                Instruction::Add(1, 0),
            ],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&mut ir);
    vm.exec().unwrap();
    assert_eq!(vm.registers()[0], Value::Num(1.0));
    assert_eq!(vm.registers()[1], Value::Num(3.0));
}

#[test]
fn sub_assign() {
    let mut ir = Ir {
        data: Vec::new(),
        native_functions: Vec::new(),
        user_functions: vec![UserFn {
            register_length: 2,
            instructions: vec![
                Instruction::Num(0, 1.0),
                Instruction::Num(1, 3.0),
                Instruction::Sub(1, 0),
            ],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&mut ir);
    vm.exec().unwrap();
    assert_eq!(vm.registers()[0], Value::Num(1.0));
    assert_eq!(vm.registers()[1], Value::Num(2.0));
}
