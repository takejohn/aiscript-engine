use std::borrow::Cow;

use aiscript_engine_common::Utf16String;
use aiscript_engine_ir::FnIndex;
use aiscript_engine_types::Type;
use gc_arena::Gc;
use indexmap::IndexMap;

pub enum Value<'gc> {
    Null,
    Bool(bool),
    Num(f64),
    Str(Cow<'gc, Utf16String>),
    Obj(Gc<'gc, IndexMap<Utf16String, Value<'gc>>>),
    Arr(Gc<'gc, Vec<Value<'gc>>>),
    Fn(Gc<'gc, VFn<'gc>>),

    /// Return文で値が返されたことを示すためのラッパー
    Return(Box<Value<'gc>>),

    Break,
    Continue,
    Error(Gc<'gc, VError<'gc>>),
    Attr(Vec<AttrElement<'gc>>),
}

pub enum VFn<'gc> {
    User(VUserFn<'gc>),
    Native(&'static dyn VNativeFn),
}

pub struct VUserFn<'gc> {
    pub args: Vec<Option<Type>>,
    pub index: FnIndex,
    pub capture: Vec<Value<'gc>>,
}

pub trait VNativeFn {
    fn native(&self, args: Vec<Option<Value>>) -> Value;
}

pub struct VError<'gc> {
    pub value: Utf16String,
    pub info: Option<Value<'gc>>,
}

pub struct AttrElement<'gc> {
    pub name: Utf16String,
    pub value: Value<'gc>,
}
