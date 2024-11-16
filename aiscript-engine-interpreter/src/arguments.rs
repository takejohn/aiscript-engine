use std::borrow::Cow;

use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind, Result};
use aiscript_engine_values::{require_any, require_boolean, require_number, Value};

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
    fn next(&mut self) -> Value {
        self.iterator.next().unwrap_or(Value::Uninitialized)
    }

    pub(crate) fn expect_any(&mut self) -> Result<Value> {
        require_any(&self.next())
    }

    pub(crate) fn expect_boolean(&mut self) -> Result<bool> {
        require_boolean(&self.next())
    }

    pub(crate) fn expect_number(&mut self) -> Result<f64> {
        require_number(&self.next())
    }
}
