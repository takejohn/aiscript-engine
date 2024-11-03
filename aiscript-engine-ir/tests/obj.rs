use aiscript_engine_common::Utf16String;
use aiscript_engine_ir::{DataItem, Instruction, Ir, UserFn};
use common::to_ir;
use pretty_assertions::assert_eq;

mod common;

#[test]
fn obj_literal() {
    let ir = to_ir("{ a: 1, b: 2, c: 3 }");
    assert_eq!(
        ir,
        Ir {
            data: vec![
                DataItem::Str(Utf16String::from("a")),
                DataItem::Str(Utf16String::from("b")),
                DataItem::Str(Utf16String::from("c")),
            ],
            native_functions: Vec::new(),
            user_functions: vec![UserFn {
                register_length: 2,
                instructions: vec![
                    Instruction::Obj(0, 3),
                    Instruction::Num(1, 1.0),
                    Instruction::StoreProp(1, 0, 0),
                    Instruction::Num(1, 2.0),
                    Instruction::StoreProp(1, 0, 1),
                    Instruction::Num(1, 3.0),
                    Instruction::StoreProp(1, 0, 2),
                ]
            }],
            entry_point: 0,
        }
    );
}

#[test]
fn prop() {
    let ir = to_ir(r#"{ a: 1 }.a"#);
    assert_eq!(
        ir,
        Ir {
            data: vec![DataItem::Str(Utf16String::from("a"))],
            native_functions: Vec::new(),
            user_functions: vec![UserFn {
                register_length: 3,
                instructions: vec![
                    Instruction::Obj(1, 1),
                    Instruction::Num(2, 1.0),
                    Instruction::StoreProp(2, 1, 0),
                    Instruction::LoadProp(0, 1, 0),
                ],
            }],
            entry_point: 0,
        },
    );
}

#[test]
fn index() {
    let ir = to_ir(r#"{ a: 1 }["a"]"#);
    assert_eq!(
        ir,
        Ir {
            data: vec![DataItem::Str(Utf16String::from("a"))],
            native_functions: Vec::new(),
            user_functions: vec![UserFn {
                register_length: 4,
                instructions: vec![
                    Instruction::Obj(1, 1),
                    Instruction::Num(2, 1.0),
                    Instruction::StoreProp(2, 1, 0),
                    Instruction::Data(3, 0),
                    Instruction::Load(0, 1, 3),
                ],
            }],
            entry_point: 0,
        },
    );
}
