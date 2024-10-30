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
                register_length: 4,
                instructions: vec![
                    Instruction::Num(1, 0.0),
                    Instruction::Null(0),
                    Instruction::Num(3, 1.0),
                    Instruction::Move(1, 3),
                    Instruction::Null(2),
                ]
            }],
            entry_point: 0,
        }
    )
}
