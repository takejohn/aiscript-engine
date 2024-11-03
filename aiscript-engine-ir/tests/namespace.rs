use aiscript_engine_ir::{Function, Instruction, Ir, UserFn};
use common::to_ir;

mod common;

#[test]
fn namespace() {
    let ir = to_ir(
        r#"
            :: Ns {
                let a = 0
            }
        "#,
    );
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            functions: vec![Function::User(UserFn {
                register_length: 2,
                instructions: vec![Instruction::Num(0, 0.0), Instruction::Null(1)]
            })],
            entry_point: 0,
        }
    )
}
