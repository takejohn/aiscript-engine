use aiscript_engine_common::{Utf16Str, Utf16String};
use aiscript_engine_ir::{DataItem, Instruction, Ir, Procedure};
use aiscript_engine_vm::{Value, Vm};
use utf16_literal::utf16;

#[test]
fn obj_literal() {
    let ir = Ir {
        data: vec![
            DataItem::Str(Utf16String::from("a")),
            DataItem::Str(Utf16String::from("b")),
            DataItem::Str(Utf16String::from("c")),
        ],
        functions: vec![Procedure {
            register_length: 4,
            instructions: vec![
                Instruction::Num(1, 42.0),
                Instruction::Bool(2, true),
                Instruction::Null(3),
                Instruction::Obj(0, 3),
                Instruction::StoreProp(1, 0, 0),
                Instruction::StoreProp(2, 0, 1),
                Instruction::StoreProp(3, 0, 2),
            ],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&ir);
    vm.exec().unwrap();
    let Value::Obj(obj) = &vm.registers()[0] else {
        panic!();
    };
    let obj = &obj.borrow().0;
    assert_eq!(
        obj.get(Utf16Str::new(&utf16!("a"))),
        Some(&Value::Num(42.0))
    );
    assert_eq!(
        obj.get(Utf16Str::new(&utf16!("b"))),
        Some(&Value::Bool(true))
    );
    assert_eq!(obj.get(Utf16Str::new(&utf16!("c"))), Some(&Value::Null));
}

#[test]
fn store_and_load_prop() {
    let ir = Ir {
        data: vec![DataItem::Str(Utf16String::from("a"))],
        functions: vec![Procedure {
            register_length: 2,
            instructions: vec![
                Instruction::Obj(0, 1),
                Instruction::Num(1, 42.0),
                Instruction::StoreProp(1, 0, 0),
                Instruction::LoadProp(0, 0, 0),
            ],
        }],
        entry_point: 0,
    };
    let mut vm = Vm::new(&ir);
    vm.exec().unwrap();
    assert_eq!(vm.registers()[0], Value::Num(42.0));
}
