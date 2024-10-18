use std::borrow::Cow;

use aiscript_engine_common::{NamePath, Utf16Str};
use utf16_literal::utf16;

const ROOT: &Utf16Str = Utf16Str::new(&utf16!("<root>"));
pub const ANONYMOUS: &Utf16Str = Utf16Str::new(&utf16!("<anonymous>"));

pub(crate) struct Variable<'src> {
    name: Cow<'src, NamePath>,
    is_mutable: bool,
}

/// 静的に解析されたスコープ。
/// * `'sc` - 親スコープのライフタイム
/// * `'ast` - ASTのライフタイム
pub(crate) struct Scope<'sc, 'ast: 'sc> {
    name: &'ast Utf16Str,
    parent: Option<&'sc mut Scope<'sc, 'ast>>,
    ns_name: Option<&'ast Utf16Str>,
    variables: Vec<Variable<'ast>>,
}

impl<'p, 'ast: 'p> Default for Scope<'p, 'ast> {
    fn default() -> Self {
        Self {
            name: ANONYMOUS,
            parent: None,
            ns_name: None,
            variables: Vec::new(),
        }
    }
}

impl<'sc, 'ast: 'sc> Scope<'sc, 'ast> {
    pub(crate) fn new() -> Self {
        Scope {
            name: ROOT,
            ..Default::default()
        }
    }

    pub(crate) fn create_child_scope(&'ast mut self, name: &'ast Utf16Str) -> Self {
        Scope {
            name,
            parent: Some(self),
            ..Default::default()
        }
    }

    pub(crate) fn create_child_namespace_scope(
        &'sc mut self,
        ns_name: &'ast Utf16Str,
        name: &'ast Utf16Str,
    ) -> Self {
        Scope {
            name,
            parent: Some(self),
            ns_name: Some(ns_name),
            ..Default::default()
        }
    }

    pub(crate) fn get(&'ast self, name: &NamePath) -> Option<&'ast Variable> {
        if let Some(result) = self.variables.iter().find(|var| *var.name == *name) {
            return Some(result);
        }
        if let Some(parent) = &self.parent {
            return parent.get(name);
        }
        return None;
    }

    pub(crate) fn exists(&self, name: &NamePath) -> bool {
        if self
            .variables
            .iter()
            .find(|var| *var.name == *name)
            .is_some()
        {
            return true;
        }
        if let Some(parent) = &self.parent {
            return parent.exists(name);
        }
        return false;
    }

    /// 指定した名前の変数を現在のスコープに追加します。名前空間である場合は接頭辞を付して親のスコープにも追加します
    pub(crate) fn add(&mut self, name: Cow<'ast, NamePath>, is_mutable: bool) -> bool {
        if self.exists(&name) {
            return false;
        }
        self.variables.push(Variable {
            name: name.clone(),
            is_mutable,
        });
        if let Some(ns_name) = self.ns_name {
            if let Some(parent) = &mut self.parent {
                let mut path = NamePath::new(ns_name);
                path.append_path(&name);
                return parent.add(Cow::Owned(path), is_mutable);
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let scope = Scope::default();
        assert_eq!(scope.name, ANONYMOUS);
        assert!(scope.parent.is_none());
        assert!(scope.ns_name.is_none());
        assert!(scope.variables.is_empty());
    }

    #[test]
    fn new() {
        let scope = Scope::new();
        assert_eq!(scope.name, ROOT);
        assert!(scope.parent.is_none());
        assert!(scope.ns_name.is_none());
        assert!(scope.variables.is_empty());
    }

    #[test]
    fn create_child_scope() {
        let mut parent = Scope::new();
        let parent_ptr: *const Scope = &parent;
        let child = parent.create_child_scope(ANONYMOUS);
        assert_eq!(child.name, ANONYMOUS);
        assert!(std::ptr::eq(child.parent.unwrap(), parent_ptr));
        assert!(child.ns_name.is_none());
        assert!(child.variables.is_empty());
    }

    #[test]
    fn create_child_namespace_scope() {
        let mut parent = Scope::new();
        let parent_ptr: *const Scope = &parent;
        let child = parent.create_child_namespace_scope(Utf16Str::new(&utf16!("Ns")), ANONYMOUS);
        assert_eq!(child.name, ANONYMOUS);
        assert!(std::ptr::eq(child.parent.unwrap(), parent_ptr));
        assert_eq!(child.ns_name.unwrap(), Utf16Str::new(&utf16!("Ns")));
        assert!(child.variables.is_empty());
    }
}
