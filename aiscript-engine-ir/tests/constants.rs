use aiscript_engine_common::Utf16String;
use aiscript_engine_ir::{DataItem, Instruction, Ir, Procedure};
use common::to_ir;

mod common;

#[test]
fn const_null() {
    let ir = to_ir("let a = null");
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            functions: Vec::new(),
            entry_point: Procedure {
                register_length: 2,
                instructions: vec![Instruction::Null(1), Instruction::Null(0)]
            }
        }
    )
}

#[test]
fn const_num() {
    let ir = to_ir("let a = 42");
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            functions: Vec::new(),
            entry_point: Procedure {
                register_length: 2,
                instructions: vec![Instruction::Num(1, 42.0), Instruction::Null(0)]
            }
        }
    )
}

#[test]
fn const_bool() {
    let ir = to_ir("let a = true");
    assert_eq!(
        ir,
        Ir {
            data: Vec::new(),
            functions: Vec::new(),
            entry_point: Procedure {
                register_length: 2,
                instructions: vec![Instruction::Bool(1, true), Instruction::Null(0)]
            }
        }
    )
}

#[test]
fn const_str() {
    let ir = to_ir("let a = 'Hello'");
    assert_eq!(
        ir,
        Ir {
            data: vec![DataItem::Str(Utf16String::from("Hello"))],
            functions: Vec::new(),
            entry_point: Procedure {
                register_length: 2,
                instructions: vec![Instruction::Data(1, 0), Instruction::Null(0)]
            }
        }
    )
}
