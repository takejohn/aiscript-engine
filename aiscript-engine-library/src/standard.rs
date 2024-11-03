use std::{collections::HashMap, sync::LazyLock};

use utf16_literal::utf16;

use crate::Library;

macro_rules! str {
    ($name: expr , $value: expr) => {
        (
            &$name as &[u16],
            aiscript_engine_values::Value::Str(::std::rc::Rc::from(&$value as &[u16])),
        )
    };
}

pub const STD: LazyLock<Library> =
    LazyLock::new(|| HashMap::from([str!(utf16!("Core:ai"), utf16!("kawaii"))]));
