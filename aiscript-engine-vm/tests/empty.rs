use aiscript_engine_ir::Ir;
use aiscript_engine_vm::{Vm, VmState};

#[test]
fn empty() {
    let ir = Ir::default();
    let mut vm = Vm::new(&ir);
    assert!(matches!(vm.step().unwrap(), VmState::Exit));
}
