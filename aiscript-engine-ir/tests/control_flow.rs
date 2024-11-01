use aiscript_engine_ir::{Instruction, Ir, Procedure};
use common::to_ir;
use pretty_assertions::assert_eq;

mod common;

#[test]
fn if_only() {
    let ir = to_ir("if false 1");
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 1,
                instructions: vec![
                    Instruction::Bool(0, false),
                    Instruction::If(
                        0,
                        vec![Instruction::Num(0, 1.0)],
                        vec![Instruction::Null(0)],
                    ),
                ],
            }],
            entry_point: 0,
        }
    );
}

#[test]
fn if_else() {
    let ir = to_ir("if true 1 else 2");
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 1,
                instructions: vec![
                    Instruction::Bool(0, true),
                    Instruction::If(
                        0,
                        vec![Instruction::Num(0, 1.0)],
                        vec![Instruction::Num(0, 2.0)],
                    ),
                ],
            }],
            entry_point: 0,
        }
    );
}

#[test]
fn if_elif_else() {
    let ir = to_ir("if false 1 elif true 2 else 3");
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 1,
                instructions: vec![
                    Instruction::Bool(0, false),
                    Instruction::If(
                        0,
                        vec![Instruction::Num(0, 1.0)],
                        vec![
                            Instruction::Bool(0, true),
                            Instruction::If(
                                0,
                                vec![Instruction::Num(0, 2.0)],
                                vec![Instruction::Num(0, 3.0)],
                            )
                        ],
                    ),
                ],
            }],
            entry_point: 0,
        },
    );
}
