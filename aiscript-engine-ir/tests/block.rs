use aiscript_engine_ir::{Instruction, Ir, Procedure};
use common::to_ir;

mod common;

#[test]
fn block() {
    let ir = to_ir("eval { let a = 1; a }");
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 2,
                instructions: vec![
                    Instruction::Num(1, 1.0),
                    Instruction::Null(0),
                    Instruction::Move(0, 1),
                ],
            }],
            entry_point: 0,
        }
    );
}