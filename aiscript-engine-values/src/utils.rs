use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind, Result};
use gc::{Gc, GcCell};

use crate::{VArr, VFn, VObj, Value};

pub fn require_any(val: &Value) -> Result<Value> {
    if let Value::Uninitialized = val {
        Err(Box::new(AiScriptBasicError::new(
            AiScriptBasicErrorKind::Runtime,
            "Expect any, but got nothing.",
            None,
        )))
    } else {
        Ok(val.clone())
    }
}

pub fn require_number(val: &Value) -> Result<f64> {
    if let Value::Num(val) = val {
        Ok(val.clone())
    } else {
        Err(Box::new(AiScriptBasicError::new(
            AiScriptBasicErrorKind::Runtime,
            format!("Expect number, but got {}.", val.type_name()),
            None,
        )))
    }
}

pub fn require_boolean(val: &Value) -> Result<bool> {
    if let Value::Bool(val) = val {
        Ok(val.clone())
    } else {
        Err(Box::new(AiScriptBasicError::new(
            AiScriptBasicErrorKind::Runtime,
            format!("Expect bool, but got {}.", val.type_name()),
            None,
        )))
    }
}

pub fn require_function(val: &Value) -> Result<Gc<GcCell<VFn>>> {
    if let Value::Fn(val) = val {
        Ok(val.clone())
    } else {
        Err(Box::new(AiScriptBasicError::new(
            AiScriptBasicErrorKind::Runtime,
            format!("Expect function, but got {}.", val.type_name()),
            None,
        )))
    }
}

pub fn require_object(val: &Value) -> Result<Gc<GcCell<VObj>>> {
    if let Value::Obj(val) = val {
        Ok(val.clone())
    } else {
        Err(Box::new(AiScriptBasicError::new(
            AiScriptBasicErrorKind::Runtime,
            format!("Expect object, but got {}.", val.type_name()),
            None,
        )))
    }
}

pub fn require_array(val: &Value) -> Result<Gc<GcCell<VArr>>> {
    if let Value::Arr(val) = val {
        Ok(val.clone())
    } else {
        Err(Box::new(AiScriptBasicError::new(
            AiScriptBasicErrorKind::Runtime,
            format!("Expect array, but got {}.", val.type_name()),
            None,
        )))
    }
}
