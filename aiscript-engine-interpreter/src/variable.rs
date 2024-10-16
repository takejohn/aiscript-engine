use crate::values::Value;

pub struct Variable<'gc> {
    is_mutable: bool,
    value: Value<'gc>,
}
