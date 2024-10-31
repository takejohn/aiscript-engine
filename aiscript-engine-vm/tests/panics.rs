use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind};
use aiscript_engine_ir::{Instruction, Ir, Procedure};
use aiscript_engine_vm::Vm;

#[test]
fn panics() {
    let ir = Ir {
        data: Vec::new(),
        functions: vec![Procedure {
            register_length: 1,
            instructions: vec![Instruction::Panic(AiScriptBasicError::new(
                AiScriptBasicErrorKind::Runtime,
                "abort",
                None,
            ))],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&ir);
    assert!(vm.step().is_err());
}
