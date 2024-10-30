use std::borrow::Cow;

use aiscript_engine_common::{Utf16Str, Utf16String};
use aiscript_engine_ir::FnIndex;
use gc_arena::Gc;
use indexmap::IndexMap;
use utf16_literal::utf16;

#[derive(Clone, Debug)]
pub enum Value<'gc> {
    Null,
    Bool(bool),
    Num(f64),
    Str(Cow<'gc, Utf16Str>),
    Obj(Gc<'gc, IndexMap<Utf16String, Value<'gc>>>),
    Arr(Gc<'gc, Vec<Value<'gc>>>),
    Fn(Gc<'gc, VFn<'gc>>),

    /// Return文で値が返されたことを示すためのラッパー
    Return(Box<Value<'gc>>),

    Break,
    Continue,
    Error(Gc<'gc, VError<'gc>>),
}

impl<'gc> PartialEq for Value<'gc> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Num(a), Self::Num(b)) => a == b,
            (Self::Str(a), Self::Str(b)) => a == b,
            (Self::Obj(a), Self::Obj(b)) => a == b,
            (Self::Arr(a), Self::Arr(b)) => std::ptr::eq(a, b),
            (Self::Fn(a), Self::Fn(b)) => std::ptr::eq(a, b),
            (Self::Return(a), Self::Return(b)) => std::ptr::eq(a, b),
            (Self::Break, Self::Break) => true,
            (Self::Continue, Self::Continue) => true,
            (Self::Error(a), Self::Error(b)) => std::ptr::eq(a, b),
            _ => false,
        }
    }
}

impl Value<'_> {
    pub fn type_name(&self) -> &'static Utf16Str {
        match self {
            Value::Null => Utf16Str::new(&utf16!("null")),
            Value::Bool(_) => Utf16Str::new(&utf16!("bool")),
            Value::Num(_) => Utf16Str::new(&utf16!("num")),
            Value::Str(_) => Utf16Str::new(&utf16!("str")),
            Value::Obj(_) => Utf16Str::new(&utf16!("obj")),
            Value::Arr(_) => Utf16Str::new(&utf16!("arr")),
            Value::Fn(_) => Utf16Str::new(&utf16!("fn")),
            Value::Return(_) => Utf16Str::new(&utf16!("return")),
            Value::Break => Utf16Str::new(&utf16!("break")),
            Value::Continue => Utf16Str::new(&utf16!("continue")),
            Value::Error(_) => Utf16Str::new(&utf16!("error")),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VFn<'gc> {
    pub index: FnIndex,
    pub capture: Vec<Value<'gc>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct VError<'gc> {
    pub value: Utf16String,
    pub info: Option<Value<'gc>>,
}
