use crate::Register;

/// 変数の型などを格納する。
pub(crate) struct Variable {
    pub is_mutable: bool,
    pub register: Register,
}

impl Default for Variable {
    fn default() -> Self {
        Self {
            is_mutable: false,
            register: 0,
        }
    }
}
