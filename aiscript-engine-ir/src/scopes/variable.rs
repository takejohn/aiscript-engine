use crate::value::Value;

/// 変数の型などを格納する。
pub(crate) struct Variable {
    pub is_mutable: bool,
    pub value: Value,
}

impl Default for Variable {
    fn default() -> Self {
        Self {
            is_mutable: false,
            value: Value,
        }
    }
}
