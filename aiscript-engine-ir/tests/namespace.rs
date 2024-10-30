use aiscript_engine_ir::{Block, Instruction, Ir};
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
            functions: Vec::new(),
            entry_point: Block {
                register_length: 2,
                instructions: vec![Instruction::Num(0, 0.0), Instruction::Null(1)]
            },
        }
    )
}
