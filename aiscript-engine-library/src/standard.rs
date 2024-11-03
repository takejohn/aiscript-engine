use std::{collections::HashMap, sync::LazyLock};

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
        (&$name as &'static [u16], $crate::LibraryValue::Fn(&$value))
    };
}

pub const STD: LazyLock<Library> = LazyLock::new(|| {
    HashMap::from([
        str!(utf16!("Core:ai"), utf16!("kawaii")),
        func!(utf16!("Core:not"), core::not),
    ])
});

mod core {
    use aiscript_engine_common::Result;
    use aiscript_engine_values::Value;

    use crate::Context;

    pub(super) fn not(
        args: Vec<Value>,
        captures: Vec<Value>,
        context: &mut dyn Context,
    ) -> Result<Value> {
        // todo
        Ok(Value::Bool(true))
    }
}
