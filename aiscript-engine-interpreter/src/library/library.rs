use std::{collections::HashMap, fmt::Debug};

use aiscript_engine_common::{Result, Utf16String};
use aiscript_engine_values::{VArr, VObj, Value};
use gc::{Gc, GcCell};

pub type Library<'lib> = HashMap<&'static [u16], LibraryValue<'lib>>;

pub enum LibraryValue<'lib> {
    Null,
    Bool(bool),
    Num(f64),
    Str(Utf16String),
    Obj(Gc<GcCell<VObj>>),
    Arr(Gc<GcCell<VArr>>),
    Fn(NativeFn<'lib>),
    // TODO: Error
}

pub enum NativeFn<'lib> {
    Static(&'lib dyn Fn(Vec<Value>, Vec<Value>) -> Result<Value>),
    Dynamic(&'lib mut dyn FnMut(Vec<Value>, Vec<Value>) -> Result<Value>),
}

impl Debug for NativeFn<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<native function>")
    }
}

impl PartialEq for NativeFn<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Static(l), Self::Static(r)) => std::ptr::eq(l, r),
            (Self::Dynamic(l), Self::Dynamic(r)) => std::ptr::eq(l, r),
            _ => false,
        }
    }
}
