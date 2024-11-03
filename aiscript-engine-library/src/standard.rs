use std::collections::HashMap;

use utf16_literal::utf16;

use crate::Library;

macro_rules! str {
    ($name: expr , $value: expr) => {
        (
            &$name as &'static [u16],
            $crate::LibraryValue::Str(::aiscript_engine_common::Utf16String::from(
                &$value as &'static [u16],
            )),
        )
    };
}

macro_rules! func {
    ($name: expr , $value: expr) => {
        (
            &$name as &'static [u16],
            $crate::LibraryValue::Fn($crate::NativeFn::Static(&$value)),
        )
    };
}

pub fn std_library<'lib>() -> Library<'lib> {
    HashMap::from([
        str!(utf16!("Core:ai"), utf16!("kawaii")),
        func!(utf16!("Core:not"), core::not),
    ])
}

mod core {
    use aiscript_engine_common::Result;
    use aiscript_engine_values::Value;

    pub(super) fn not(args: Vec<Value>, captures: Vec<Value>) -> Result<Value> {
        // todo
        Ok(Value::Bool(true))
    }
}
