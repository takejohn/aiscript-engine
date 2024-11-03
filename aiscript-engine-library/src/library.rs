use std::collections::HashMap;

use aiscript_engine_common::{Result, Utf16String};
use aiscript_engine_values::{VArr, VObj, Value};
use gc::{Gc, GcCell};

use crate::context::Context;

pub type Library<'lib> = HashMap<&'static [u16], LibraryValue<'lib>>;

pub enum LibraryValue<'lib> {
    Null,
    Bool(bool),
    Num(f64),
    Str(Utf16String),
    Obj(Gc<GcCell<VObj>>),
    Arr(Gc<GcCell<VArr>>),
    Fn(&'lib NativeFn),
    // TODO: Error
}

pub type NativeFn = dyn Fn(Vec<Value>, Vec<Value>, &mut dyn Context) -> Result<Value>;
