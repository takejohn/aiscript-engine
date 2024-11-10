use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind, Result};
use aiscript_engine_values::Value;

/// 関数の引数を取り出す。
pub(crate) struct Arguments {
    iterator: <Vec<Value> as IntoIterator>::IntoIter,
}

impl From<Vec<Value>> for Arguments {
    fn from(value: Vec<Value>) -> Self {
        Arguments {
            iterator: value.into_iter(),
        }
    }
}

impl Arguments {
    pub(crate) fn require_any(&mut self) -> Result<Value> {
        match self.iterator.next() {
            Some(value) => Ok(value),
            None => Err(Box::new(AiScriptBasicError::new(
                AiScriptBasicErrorKind::Runtime,
                "Expect anything, but got nothing.",
                None,
            ))),
        }
    }
}
