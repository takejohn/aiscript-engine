use aiscript_engine_ir::{Instruction, Ir, Procedure};
use aiscript_engine_vm::{Vm, VmState};

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