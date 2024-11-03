use aiscript_engine_common::Utf16String;
use aiscript_engine_ir::{DataItem, Function, Instruction, Ir, UserFn};
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
            functions: vec![Function::User(UserFn {
                register_length: 5,
                instructions: vec![
                    Instruction::Arr(1, 2),
                    Instruction::Num(2, 1.0),
                    Instruction::StoreIndex(2, 1, 0),
                    Instruction::Num(2, 2.0),
                    Instruction::StoreIndex(2, 1, 1),
                    Instruction::LoadIndex(3, 1, 0),
                    Instruction::LoadIndex(4, 1, 1),
                    Instruction::Null(0),
                ]
            })],
            entry_point: 0,
        },
    );
}

#[test]
fn define_obj() {
    let ir = to_ir("let { a: a, b: b } = { a: 1, b: 2 }");
    assert_eq!(
        ir,
        Ir {
            data: vec![
                DataItem::Str(Utf16String::from("a")),
                DataItem::Str(Utf16String::from("b")),
            ],
            functions: vec![Function::User(UserFn {
                register_length: 5,
                instructions: vec![
                    Instruction::Obj(1, 2),
                    Instruction::Num(2, 1.0),
                    Instruction::StoreProp(2, 1, 0),
                    Instruction::Num(2, 2.0),
                    Instruction::StoreProp(2, 1, 1),
                    Instruction::LoadProp(3, 1, 0),
                    Instruction::LoadProp(4, 1, 1),
                    Instruction::Null(0),
                ]
            })],
            entry_point: 0,
        }
    )
}
