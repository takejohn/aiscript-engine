mod common;

use aiscript_engine_ir::{Instruction, Ir, Procedure};
use common::to_ir;
use pretty_assertions::assert_eq;

#[test]
fn exists_false() {
    let ir = to_ir("exists a");
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 1,
                instructions: vec![Instruction::Bool(0, false),]
            }],
            entry_point: 0,
        }
    );
}

#[test]
fn exists_true() {
    let ir = to_ir(
        "
        let a = 42
        exists a
    ",
    );
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 3,
                instructions: vec![
                    Instruction::Num(1, 42.0),
                    Instruction::Null(0),
                    Instruction::Bool(2, true),
                ]
            }],
            entry_point: 0,
        }
    );
}
