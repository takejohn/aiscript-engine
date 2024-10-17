use std::collections::HashMap;

use aiscript_engine_common::Utf16Str;
use utf16_literal::utf16;

use crate::{path::Path, Register};

const ROOT: &Utf16Str = Utf16Str::new(&utf16!("<root>"));
pub const ANONYMOUS: &Utf16Str = Utf16Str::new(&utf16!("<anonymous>"));

pub(crate) struct Variable<'src> {
    path: Path<'src>,
    is_mutable: bool,
}

pub(crate) struct Scope<'src> {
    name: &'src Utf16Str,
    parent: Option<&'src mut Scope<'src>>,
    ns_name: Option<&'src Utf16Str>,
    variables: Vec<Variable<'src>>,
}

impl<'src> Default for Scope<'src> {
    fn default() -> Self {
        Self {
            name: ANONYMOUS,
            parent: None,
            ns_name: None,
            variables: Vec::new(),
        }
    }
}

impl<'src> Scope<'src> {
    pub(crate) fn new() -> Self {
        Scope {
            name: ROOT,
            ..Default::default()
        }
    }

    pub(crate) fn create_child_scope(&'src mut self, name: &'src Utf16Str) -> Self {
        Scope {
            name,
            parent: Some(self),
            ..Default::default()
        }
    }

    pub(crate) fn create_child_namespace_scope(&'src mut self, ns_name: &'src Utf16Str, name: &'src Utf16Str) -> Self {
        Scope {
            name,
            parent: Some(self),
            ns_name: Some(ns_name),
            ..Default::default()
        }
    }

    pub(crate) fn get(&self, name: &Utf16Str) -> Option<&'src Variable> {
        if let Some(result) = self.variables.iter().find(|var| var.path.eq_to_ref(name)) {
            return Some(result);
        }
        if let Some(parent) = &self.parent {
            return parent.get(name);
        }
        return None;
    }

    pub(crate) fn exists(&self, name: &Utf16Str) -> bool {
        if self.variables.iter().find(|var| var.path.eq_to_ref(name)).is_some() {
            return true;
        }
        if let Some(parent) = &self.parent {
            return parent.exists(name);
        }
        return false;
    }

    /// 指定した名前の変数を現在のスコープに追加します。名前空間である場合は接頭辞を付して親のスコープにも追加します
    pub(crate) fn add(&mut self, variable: Variable<'src>) -> bool {
        let name = variable.path;
        todo!()
        // if self.exists(name) {
        //     return false;
        // }
        // self.variables.push(variable);
        // if let Some(ns_name) = self.ns_name {
        //     if let Some(parent) = self.parent {
        //         return parent.add(Variable {
        //             path: ns_name.to_owned() + Utf16Str::new(&utf16!(":")) + name,
        //             ..variable
        //         })
        //     }
        // }
        // return true;
    }
}
