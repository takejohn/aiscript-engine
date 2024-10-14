use aiscript_engine_ast as ast;
use aiscript_engine_common::{AiScriptSyntaxError, Result, Utf16Str, Utf16String};
use derive_wrapper::Wrapper;
use utf16_literal::utf16;

#[derive(Wrapper)]
pub enum Type {
    Simple(TSimple),
    Generic(TGeneric),
    Fn(TFn),
}

pub enum TSimple {
    Null,
    Bool,
    Num,
    Str,
    Any,
    Void,
}

impl TSimple {
    pub fn name(&self) -> &Utf16Str {
        match self {
            TSimple::Null => Utf16Str::new(&utf16!("null")),
            TSimple::Bool => Utf16Str::new(&utf16!("bool")),
            TSimple::Num => Utf16Str::new(&utf16!("num")),
            TSimple::Str => Utf16Str::new(&utf16!("str")),
            TSimple::Any => Utf16Str::new(&utf16!("any")),
            TSimple::Void => Utf16Str::new(&utf16!("void")),
        }
    }

    pub fn for_name(name: &Utf16Str) -> Option<Self> {
        match name.as_u16s() {
            &utf16!("null") => Some(TSimple::Null),
            &utf16!("bool") => Some(TSimple::Bool),
            &utf16!("num") => Some(TSimple::Num),
            &utf16!("str") => Some(TSimple::Str),
            &utf16!("any") => Some(TSimple::Any),
            &utf16!("void") => Some(TSimple::Void),
            _ => None,
        }
    }
}

pub enum TGeneric {
    Arr(Box<Type>),
    Obj(Box<Type>),
}

pub struct TFn {
    args: Vec<Type>,
    result: Box<Type>,
}

fn get_type_name_by_source(type_source: &ast::TypeSource) -> Utf16String {
    match type_source {
        ast::TypeSource::NamedTypeSource(type_source) => {
            if let Some(inner) = &type_source.inner {
                let inner = get_type_name_by_source(&inner);
                return type_source.name.to_owned()
                    + utf16!('<')
                    + inner.as_utf16_str()
                    + utf16!('>');
            } else {
                return type_source.name.to_owned();
            }
        }
        ast::TypeSource::FnTypeSource(type_source) => {
            let mut args = type_source
                .args
                .iter()
                .map(|arg| get_type_name_by_source(arg));
            let mut name = Utf16String::from_iter(&utf16!("@("));
            if let Some(first_arg) = args.next() {
                name += first_arg.as_utf16_str();
            }
            while let Some(arg) = args.next() {
                name += Utf16Str::new(&utf16!(", "));
                name += arg.as_utf16_str();
            }
            name += Utf16Str::new(&utf16!(") { "));
            name += Utf16Str::new(&utf16!(" }"));
            return name;
        }
    }
}

pub fn get_type_by_source(type_source: &ast::TypeSource) -> Result<Type> {
    match type_source {
        ast::TypeSource::NamedTypeSource(named_type_source) => {
            let name = &named_type_source.name;
            if let Some(ty) = TSimple::for_name(name) {
                return Ok(ty.into());
            }
            match name.as_u16s() {
                &utf16!("arr") => {
                    return Ok(TGeneric::Arr(Box::new(get_inner_type(named_type_source)?)).into())
                }
                &utf16!("obj") => {
                    return Ok(TGeneric::Obj(Box::new(get_inner_type(named_type_source)?)).into())
                }
                _ => {}
            };
            return Err(Box::new(AiScriptSyntaxError::new(
                format!("Unknown type: '{}'", get_type_name_by_source(type_source)),
                named_type_source.loc.start.to_owned(),
            )));
        }
        ast::TypeSource::FnTypeSource(fn_type_source) => {
            let mut args: Vec<Type> = Vec::new();
            for arg in &fn_type_source.args {
                args.push(get_type_by_source(arg)?);
            }
            return Ok(TFn {
                args,
                result: Box::new(get_type_by_source(&fn_type_source.result)?),
            }
            .into());
        }
    }
}

fn get_inner_type(type_source: &ast::NamedTypeSource) -> Result<Type> {
    if let Some(ref inner) = type_source.inner {
        get_type_by_source(inner)
    } else {
        Ok(TSimple::Any.into())
    }
}
