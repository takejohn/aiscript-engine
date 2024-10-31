use aiscript_engine_ir::{Instruction, Ir, Procedure};
use common::to_ir;

mod common;

#[test]
fn assign() {
    let ir = to_ir(
        "
        var a = 0
        a = 1
    ",
    );
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 3,
                instructions: vec![
                    Instruction::Num(1, 0.0),
                    Instruction::Null(0),
                    Instruction::Num(2, 1.0),
                    Instruction::Move(1, 2),
                    Instruction::Null(0),
                ]
            }],
            entry_point: 0,
        }
    )
}
