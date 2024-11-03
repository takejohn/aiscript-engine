use aiscript_engine_ir::{Function, Instruction, Ir, UserFn};
use common::to_ir;
use pretty_assertions::assert_eq;

mod common;

#[test]
fn not() {
    let ir = to_ir("!true");
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            functions: vec![Function::User(UserFn {
                register_length: 2,
                instructions: vec![Instruction::Bool(1, true), Instruction::Not(0, 1)],
            })],
            entry_point: 0,
        }
    );
}
