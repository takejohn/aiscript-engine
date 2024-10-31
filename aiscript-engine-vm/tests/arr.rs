use aiscript_engine_ir::{Instruction, Ir, Procedure};
use aiscript_engine_vm::{Value, Vm};

#[test]
fn arr_literal() {
    let ir = Ir {
        data: Vec::new(),
        functions: vec![Procedure {
            register_length: 4,
            instructions: vec![
                Instruction::Num(1, 42.0),
                Instruction::Bool(2, true),
                Instruction::Null(3),
                Instruction::Arr(0, 3),
                Instruction::StoreImmediate(1, 0, 0),
                Instruction::StoreImmediate(2, 0, 1),
                Instruction::StoreImmediate(3, 0, 2),
            ],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&ir);
    vm.exec().unwrap();
    let Value::Arr(arr) = &vm.registers()[0] else {
        panic!();
    };
    let arr = arr.borrow();
    assert_eq!(arr[0], Value::Num(42.0));
    assert_eq!(arr[1], Value::Bool(true));
    assert_eq!(arr[2], Value::Null);
}
