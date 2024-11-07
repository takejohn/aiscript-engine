use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind};
use aiscript_engine_ir::{Instruction, Ir, UserFn};
use aiscript_engine_vm::Vm;

#[test]
fn panics() {
    let mut ir = Ir {
        data: Vec::new(),
        native_functions: Vec::new(),
        user_functions: vec![UserFn {
            register_length: 1,
            instructions: vec![Instruction::Panic(AiScriptBasicError::new(
                AiScriptBasicErrorKind::Runtime,
                "abort",
                None,
            ))],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&mut ir);
    assert!(vm.exec().is_err());
}
