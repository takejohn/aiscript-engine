use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind, Result};
use gc::{Gc, GcCell};

use crate::{VArr, Value};

pub(crate) trait GetByF64<T> {
    fn get_by_f64(&self, index: f64) -> Option<&T>;
}

impl<T> GetByF64<T> for [T] {
    fn get_by_f64(&self, index: f64) -> Option<&T> {
        let index_int = index as usize;
        if index == index as f64 {
            self.get(index_int)
        } else {
            None
        }
    }
}

pub(crate) fn require_bool(val: Value) -> Result<bool> {
    if let Value::Bool(val) = val {
        Ok(val)
    } else {
        Err(Box::new(AiScriptBasicError::new(
            AiScriptBasicErrorKind::Runtime,
            format!("Expect array, but got {}.", val.type_name()),
            None,
        )))
    }
}

pub(crate) fn require_array(val: Value) -> Result<Gc<GcCell<VArr>>> {
    if let Value::Arr(val) = val {
        Ok(val)
    } else {
        Err(Box::new(AiScriptBasicError::new(
            AiScriptBasicErrorKind::Runtime,
            format!("Expect array, but got {}.", val.type_name()),
            None,
        )))
    }
}
