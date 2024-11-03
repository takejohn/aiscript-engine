use aiscript_engine_ir::{Instruction, Ir, UserFn};
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
            native_functions: Vec::new(),
            user_functions: vec![UserFn {
                register_length: 2,
                instructions: vec![
                    Instruction::Arr(0, 3),
                    Instruction::Num(1, 1.0),
                    Instruction::StoreIndex(1, 0, 0),
                    Instruction::Num(1, 2.0),
                    Instruction::StoreIndex(1, 0, 1),
                    Instruction::Num(1, 3.0),
                    Instruction::StoreIndex(1, 0, 2),
                ]
            }],
            entry_point: 0,
        }
    );
}

#[test]
fn index() {
    let ir = to_ir("[1][0]");
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            native_functions: Vec::new(),
            user_functions: vec![UserFn {
                register_length: 4,
                instructions: vec![
                    Instruction::Arr(1, 1),
                    Instruction::Num(2, 1.0),
                    Instruction::StoreIndex(2, 1, 0),
                    Instruction::Num(3, 0.0),
                    Instruction::Load(0, 1, 3),
                ]
            }],
            entry_point: 0,
        },
    );
}
