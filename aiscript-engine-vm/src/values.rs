use std::rc::Rc;

use aiscript_engine_common::{Utf16Str, Utf16String};
use aiscript_engine_ir::FnIndex;
use gc::{custom_trace, Finalize, Gc, GcCell, Trace};
use indexmap::IndexMap;
use utf16_literal::utf16;

#[derive(Clone, Debug, Finalize)]
pub enum Value {
    /// 未初期化値
    Uninitialized,

    Null,
    Bool(bool),
    Num(f64),
    Str(Rc<Utf16String>),
    Obj(Gc<GcCell<VObj>>),
    Arr(Gc<GcCell<Vec<Value>>>),
    Fn(Gc<VFn>),

    /// Return文で値が返されたことを示すためのラッパー
    Return(Box<Value>),

    Break,
    Continue,
    Error(Gc<VError>),
}

unsafe impl Trace for Value {
    custom_trace!(this, {
        match this {
            Value::Uninitialized => {}
            Value::Null => {}
            Value::Bool(_) => {}
            Value::Num(_) => {}
            Value::Str(_) => {}
            Value::Obj(gc) => mark(gc),
            Value::Arr(gc) => mark(gc),
            Value::Fn(gc) => mark(gc),
            Value::Return(value) => mark(value),
            Value::Break => {}
            Value::Continue => {}
            Value::Error(gc) => mark(gc),
        }
    });
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Null, Self::Null) => true,
            (Self::Bool(a), Self::Bool(b)) => a == b,
            (Self::Num(a), Self::Num(b)) => a == b,
            (Self::Str(a), Self::Str(b)) => a == b,
            (Self::Obj(a), Self::Obj(b)) => std::ptr::eq(a, b),
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

impl Value {
    pub fn type_name(&self) -> &'static Utf16Str {
        match self {
            Value::Uninitialized => panic!("Reading uninitialized value"),
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

#[derive(Clone, Debug, Finalize)]
pub struct VObj(IndexMap<Utf16String, Value>);

unsafe impl Trace for VObj {
    custom_trace!(this, {
        for value in this.0.values() {
            mark(value)
        }
    });
}

#[derive(Clone, Debug, Trace, Finalize)]
pub struct VFn {
    pub index: FnIndex,
    pub capture: Vec<Value>,
}

#[derive(Clone, Debug, PartialEq, Finalize)]
pub struct VError {
    pub value: Utf16String,
    pub info: Option<Value>,
}

unsafe impl Trace for VError {
    custom_trace!(this, {
        if let Some(info) = &this.info {
            mark(info)
        }
    });
}
