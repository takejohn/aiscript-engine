mod standard;

use std::collections::HashMap;

use aiscript_engine_values::Value;
pub use standard::STD;

pub type Library = HashMap<&'static [u16], Value>;
