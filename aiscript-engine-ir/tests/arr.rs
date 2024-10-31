use aiscript_engine_ir::{Instruction, Ir, Procedure};
use common::to_ir;
use pretty_assertions::assert_eq;

mod common;

#[test]
fn arr_literal() {
    let ir = to_ir("[1, 2, 3]");
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 2,
                instructions: vec![
                    Instruction::Arr(0, 3),
                    Instruction::Num(1, 1.0),
                    Instruction::StoreImmediate(1, 0, 0),
                    Instruction::Num(1, 2.0),
                    Instruction::StoreImmediate(1, 0, 1),
                    Instruction::Num(1, 3.0),
                    Instruction::StoreImmediate(1, 0, 2),
                ]
            }],
            entry_point: 0,
        }
    );
}
