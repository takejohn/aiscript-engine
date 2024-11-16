use std::collections::HashMap;

use aiscript_engine_common::Utf16String;
use utf16_literal::utf16;

use super::{Library, LibraryValue};

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
        (
            &utf16!("Core:v") as &'static [u16],
            LibraryValue::Str(version()),
        ),
        str!(utf16!("Core:ai"), utf16!("kawaii")),
        func!(utf16!("Core:not"), core::not),
    ])
}

fn version() -> Utf16String {
    let version_str = env!("CARGO_PKG_VERSION");
    let separator = version_str
        .find('+')
        .expect("no plus sign in package version");
    return Utf16String::from(&version_str[separator + 1..]);
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
