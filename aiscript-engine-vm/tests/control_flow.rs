use aiscript_engine_ir::{Instruction, Ir, Procedure};
use aiscript_engine_vm::{Value, Vm};

#[test]
fn test_if_true() {
    let ir = Ir {
        data: Vec::new(),
        functions: vec![Procedure {
            register_length: 2,
            instructions: vec![
                Instruction::Bool(1, true),
                Instruction::If(
                    1,
                    vec![Instruction::Num(0, 1.0)],
                    vec![Instruction::Num(0, 2.0)],
                ),
            ],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&ir);
    vm.exec().unwrap();
    assert_eq!(vm.registers()[0], Value::Num(1.0));
}

#[test]
fn test_if_false() {
    let ir = Ir {
        data: Vec::new(),
        functions: vec![Procedure {
            register_length: 2,
            instructions: vec![
                Instruction::Bool(1, false),
                Instruction::If(
                    1,
                    vec![Instruction::Num(0, 1.0)],
                    vec![Instruction::Num(0, 2.0)],
                ),
            ],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&ir);
    vm.exec().unwrap();
    assert_eq!(vm.registers()[0], Value::Num(2.0));
}
