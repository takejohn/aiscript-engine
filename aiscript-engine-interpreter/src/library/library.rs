use std::{collections::HashMap, fmt::Debug, rc::Rc};

use aiscript_engine_common::{Result, Utf16String};
use aiscript_engine_values::{VArr, VObj, Value};
use gc::{Gc, GcCell};

use crate::vm::Vm;

pub(crate) type Library = HashMap<&'static [u16], LibraryValue>;

pub(crate) enum LibraryValue {
    Null,
    Bool(bool),
    Num(f64),
    Str(Utf16String),
    Obj(Gc<GcCell<VObj>>),
    Arr(Gc<GcCell<VArr>>),
    Fn(NativeFn),
    // TODO: Error
}

pub(crate) enum NativeFn {
    Static(fn(Vec<Value>, &mut Vm) -> Result<Value>),
    Dynamic(Rc<dyn Fn(Vec<Value>, &mut Vm) -> Result<Value>>),
}

impl Debug for NativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native function>")
    }
}

impl PartialEq for NativeFn {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Static(l), Self::Static(r)) => std::ptr::eq(l, r),
            (Self::Dynamic(l), Self::Dynamic(r)) => std::ptr::eq(l, r),
            _ => false,
        }
    }
}
