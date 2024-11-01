use aiscript_engine_ir::{Instruction, Ir, Procedure};
use common::to_ir;
use pretty_assertions::assert_eq;

mod common;

#[test]
fn define_arr() {
    let ir = to_ir("let [a, b] = [1, 2]");
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            functions: vec![Procedure {
                register_length: 5,
                instructions: vec![
                    Instruction::Arr(1, 2),
                    Instruction::Num(2, 1.0),
                    Instruction::StoreImmediate(2, 1, 0),
                    Instruction::Num(2, 2.0),
                    Instruction::StoreImmediate(2, 1, 1),
                    Instruction::LoadImmediate(3, 1, 0),
                    Instruction::LoadImmediate(4, 1, 1),
                    Instruction::Null(0),
                ]
            }],
            entry_point: 0,
        },
    );
}
