use std::collections::HashMap;

use utf16_literal::utf16;

use super::Library;

macro_rules! str {
    ($name: expr , $value: expr) => {
        (
            &$name as &'static [u16],
            $crate::library::LibraryValue::Str(::aiscript_engine_common::Utf16String::from(
                &$value as &'static [u16],
            )),
        )
    };
}

macro_rules! func {
    ($name: expr , $value: expr) => {
        (
            &$name as &'static [u16],
            $crate::library::LibraryValue::Fn($crate::library::NativeFn::Static($value)),
        )
    };
}

pub(crate) fn std_library() -> Library {
    HashMap::from([
        str!(utf16!("Core:ai"), utf16!("kawaii")),
        func!(utf16!("Core:not"), core::not),
    ])
}

mod core {
    use aiscript_engine_common::Result;
    use aiscript_engine_values::Value;

    use crate::{arguments::Arguments, vm::Vm};

    pub(super) fn not(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        Ok(Value::Bool(!args.expect_boolean()?))
    }
}
