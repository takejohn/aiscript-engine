use std::collections::HashMap;

use aiscript_engine_common::Utf16String;
use utf16_literal::utf16;

use super::{Library, LibraryValue};

macro_rules! str {
    ($name: expr , $value: expr) => {
        (
            &$name as &'static [u16],
            $crate::library::LibraryValue::Str(::aiscript_engine_common::Utf16String::from(
                &$value as &'static [u16],
            )),
        )
    };
}

macro_rules! func {
    ($name: expr , $value: expr) => {
        (
            &$name as &'static [u16],
            $crate::library::LibraryValue::Fn($crate::library::NativeFn::Static($value)),
        )
    };
}

pub(crate) fn std_library() -> Library {
    HashMap::from([
        (
            &utf16!("Core:v") as &'static [u16],
            LibraryValue::Str(version()),
        ),
        str!(utf16!("Core:ai"), utf16!("kawaii")),
        func!(utf16!("Core:not"), core::not),
        func!(utf16!("Core:eq"), core::eq),
        func!(utf16!("Core:neq"), core::neq),
        func!(utf16!("Core:and"), core::and),
        func!(utf16!("Core:or"), core::or),
        func!(utf16!("Core:add"), core::add),
        func!(utf16!("Core:sub"), core::sub),
        func!(utf16!("Core:mul"), core::mul),
        func!(utf16!("Core:pow"), core::pow),
        func!(utf16!("Core:div"), core::div),
        func!(utf16!("Core:mod"), core::modulo),
        func!(utf16!("Core:gt"), core::gt),
        func!(utf16!("Core:lt"), core::lt),
        func!(utf16!("Core:gteq"), core::gteq),
        func!(utf16!("Core:lteq"), core::lteq),
    ])
}

fn version() -> Utf16String {
    let version_str = env!("CARGO_PKG_VERSION");
    let separator = version_str
        .find('+')
        .expect("no plus sign in package version");
    return Utf16String::from(&version_str[separator + 1..]);
}

mod core {
    use aiscript_engine_common::Result;
    use aiscript_engine_values::Value;

    use crate::{arguments::Arguments, vm::Vm};

    pub(super) fn not(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        Ok(Value::Bool(!args.expect_boolean()?))
    }

    pub(super) fn eq(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        let left = args.expect_any()?;
        let right = args.expect_any()?;
        Ok(Value::Bool(left == right))
    }

    pub(super) fn neq(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        let left = args.expect_any()?;
        let right = args.expect_any()?;
        Ok(Value::Bool(left != right))
    }

    pub(super) fn and(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        let left = args.expect_boolean()?;
        let right = args.expect_boolean()?;
        Ok(Value::Bool(left && right))
    }

    pub(super) fn or(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        let left = args.expect_boolean()?;
        let right = args.expect_boolean()?;
        Ok(Value::Bool(left || right))
    }

    pub(super) fn add(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        let left = args.expect_number()?;
        let right = args.expect_number()?;
        Ok(Value::Num(left + right))
    }

    pub(super) fn sub(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        let left = args.expect_number()?;
        let right = args.expect_number()?;
        Ok(Value::Num(left - right))
    }

    pub(super) fn mul(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        let left = args.expect_number()?;
        let right = args.expect_number()?;
        Ok(Value::Num(left * right))
    }

    pub(super) fn pow(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        let left = args.expect_number()?;
        let right = args.expect_number()?;
        Ok(Value::Num(left.powf(right)))
    }

    pub(super) fn div(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        let left = args.expect_number()?;
        let right = args.expect_number()?;
        Ok(Value::Num(left / right))
    }

    pub(super) fn modulo(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        let left = args.expect_number()?;
        let right = args.expect_number()?;
        Ok(Value::Num(left % right))
    }

    pub(super) fn gt(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        let left = args.expect_number()?;
        let right = args.expect_number()?;
        Ok(Value::Bool(left > right))
    }

    pub(super) fn lt(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        let left = args.expect_number()?;
        let right = args.expect_number()?;
        Ok(Value::Bool(left < right))
    }

    pub(super) fn gteq(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        let left = args.expect_number()?;
        let right = args.expect_number()?;
        Ok(Value::Bool(left >= right))
    }

    pub(super) fn lteq(args: Vec<Value>, _: &mut Vm) -> Result<Value> {
        let mut args = Arguments::from(args);
        let left = args.expect_number()?;
        let right = args.expect_number()?;
        Ok(Value::Bool(left <= right))
    }
}
