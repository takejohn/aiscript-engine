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

pub const STD: LazyLock<Library> =
    LazyLock::new(|| HashMap::from([str!(utf16!("Core:ai"), utf16!("kawaii"))]));
