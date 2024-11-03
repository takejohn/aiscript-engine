use aiscript_engine_ir::Ir;
use aiscript_engine_vm::{Vm, VmState};

#[test]
fn empty() {
    let mut ir = Ir::default();
    let mut vm = Vm::new(&mut ir);
    assert!(matches!(vm.step().unwrap(), VmState::Exit));
}
