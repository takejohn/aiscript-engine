/// 変数の型などを格納する。
pub(crate) struct Variable {
    pub is_mutable: bool,
}

impl Default for Variable {
    fn default() -> Self {
        Self { is_mutable: false }
    }
}
