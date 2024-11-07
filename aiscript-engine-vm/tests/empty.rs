use aiscript_engine_ir::Ir;
use aiscript_engine_vm::Vm;

#[test]
fn empty() {
    let mut ir = Ir::default();
    let mut vm = Vm::new(&mut ir);
    assert!(vm.exec().is_ok());
}
