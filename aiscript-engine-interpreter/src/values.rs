use std::borrow::Cow;

use aiscript_engine_ast::{Expression, Node};
use aiscript_engine_common::Utf16String;
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
    args: VFnArg,
    statements: &'gc Vec<Node>,
    scope: (), // TODO
}

pub trait VNativeFn {
    fn native(&self, args: Vec<Option<Value>>) -> Value;
}

pub struct VFnArg {
    dest: Expression,
    ty: Option<Type>,
}

pub struct VError<'gc> {
    value: Utf16String,
    info: Option<Value<'gc>>,
}

pub struct AttrElement<'gc> {
    name: Utf16String,
    value: Value<'gc>,
}
