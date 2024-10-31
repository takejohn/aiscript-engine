use aiscript_engine_ir::{Instruction, Ir, Procedure};
use aiscript_engine_vm::{Value, Vm};

#[test]
fn not() {
    let ir = Ir {
        data: Vec::new(),
        functions: vec![Procedure {
            register_length: 2,
            instructions: vec![Instruction::Bool(1, true), Instruction::Not(0, 1)],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&ir);
    vm.exec().unwrap();
    assert_eq!(vm.registers()[0], Value::Bool(false));
}
