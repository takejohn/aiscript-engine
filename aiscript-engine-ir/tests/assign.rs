use aiscript_engine_common::Utf16String;
use aiscript_engine_ir::{DataItem, Instruction, Ir, UserFn};
use common::to_ir;
use pretty_assertions::assert_eq;

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
            native_functions: Vec::new(),
            user_functions: vec![UserFn {
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

#[test]
fn assign_index() {
    let ir = to_ir("let a = [1]; a[0] = 2");
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            native_functions: Vec::new(),
            user_functions: vec![UserFn {
                register_length: 6,
                instructions: vec![
                    Instruction::Arr(1, 1),
                    Instruction::Num(2, 1.0),
                    Instruction::StoreIndex(2, 1, 0),
                    Instruction::Null(0),
                    Instruction::Num(3, 2.0),
                    Instruction::Move(4, 1),
                    Instruction::Num(5, 0.0),
                    Instruction::Store(3, 4, 5),
                    Instruction::Null(0),
                ]
            }],
            entry_point: 0,
        }
    )
}

#[test]
fn assign_prop() {
    let ir = to_ir("let o = {}; o.a = 1");
    assert_eq!(
        ir,
        Ir {
            data: vec![DataItem::Str(Utf16String::from("a"))],
            native_functions: Vec::new(),
            user_functions: vec![UserFn {
                register_length: 4,
                instructions: vec![
                    Instruction::Obj(1, 0),
                    Instruction::Null(0),
                    Instruction::Num(2, 1.0),
                    Instruction::Move(3, 1),
                    Instruction::StoreProp(2, 3, 0),
                    Instruction::Null(0),
                ]
            }],
            entry_point: 0,
        }
    );
}

#[test]
fn assign_arr() {
    let ir = to_ir("var a = null; var b = null; [a, b] = [1, 2]");
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            native_functions: Vec::new(),
            user_functions: vec![UserFn {
                register_length: 6,
                instructions: vec![
                    Instruction::Null(1),
                    Instruction::Null(0),
                    Instruction::Null(2),
                    Instruction::Null(0),
                    Instruction::Arr(3, 2),
                    Instruction::Num(4, 1.0),
                    Instruction::StoreIndex(4, 3, 0),
                    Instruction::Num(4, 2.0),
                    Instruction::StoreIndex(4, 3, 1),
                    Instruction::LoadIndex(5, 3, 0),
                    Instruction::Move(1, 5),
                    Instruction::LoadIndex(5, 3, 1),
                    Instruction::Move(2, 5),
                    Instruction::Null(0),
                ]
            }],
            entry_point: 0,
        },
    );
}

#[test]
fn assign_obj() {
    let ir = to_ir("var a = null; var b = null; { a: a, b: b } = { a: 1, b: 2 }");
    assert_eq!(
        ir,
        Ir {
            data: vec![
                DataItem::Str(Utf16String::from("a")),
                DataItem::Str(Utf16String::from("b")),
            ],
            native_functions: Vec::new(),
            user_functions: vec![UserFn {
                register_length: 6,
                instructions: vec![
                    Instruction::Null(1),
                    Instruction::Null(0),
                    Instruction::Null(2),
                    Instruction::Null(0),
                    Instruction::Obj(3, 2),
                    Instruction::Num(4, 1.0),
                    Instruction::StoreProp(4, 3, 0),
                    Instruction::Num(4, 2.0),
                    Instruction::StoreProp(4, 3, 1),
                    Instruction::LoadProp(5, 3, 0),
                    Instruction::Move(1, 5),
                    Instruction::LoadProp(5, 3, 1),
                    Instruction::Move(2, 5),
                    Instruction::Null(0),
                ]
            }],
            entry_point: 0,
        }
    )
}
