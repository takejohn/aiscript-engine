use aiscript_engine_common::{AiScriptBasicError, AiScriptBasicErrorKind, Result};
use aiscript_engine_values::{VArr, VFn, VObj, Value};
use gc::{Gc, GcCell};

pub(crate) trait GetByF64<T> {
    fn get_by_f64(&self, index: f64) -> Option<&T>;

    fn get_mut_by_f64(&mut self, index: f64) -> Option<&mut T>;
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

    fn get_mut_by_f64(&mut self, index: f64) -> Option<&mut T> {
        let index_int = index as usize;
        if index == index as f64 {
            self.get_mut(index_int)
        } else {
            None
        }
    }
}
