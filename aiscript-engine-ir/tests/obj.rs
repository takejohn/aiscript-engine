use aiscript_engine_common::Utf16String;
use aiscript_engine_ir::{DataItem, Instruction, Ir, Procedure};
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
            functions: vec![Procedure {
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
