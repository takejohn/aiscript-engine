use std::{borrow::Cow, collections::HashMap};

use aiscript_engine_common::{NamePath, Utf16Str};
use utf16_literal::utf16;

pub(crate) use variable::Variable;

mod variable;

/// グローバル変数が登録されたスコープ。
struct RootScope<'ast> {
    variables: HashMap<Cow<'ast, NamePath>, Variable>,
}

/// 名前空間内のスコープ。
/// 変数の実体は[`RootScope`]にある。
struct NamespaceScope<'ast> {
    name: &'ast Utf16Str,
}

/// 関数や制御構文内のスコープ。
struct BlockScope<'ast> {
    variables: HashMap<&'ast NamePath, Variable>,
}

pub(crate) struct Scopes<'ast> {
    root: RootScope<'ast>,
    namespaces: Vec<NamespaceScope<'ast>>,
    blocks: Vec<BlockScope<'ast>>,
}

impl<'ast> Scopes<'ast> {
    pub(crate) fn new() -> Self {
        Scopes {
            root: RootScope {
                variables: HashMap::new(),
            },
            namespaces: Vec::new(),
            blocks: Vec::new(),
        }
    }

    fn is_root(&self) -> bool {
        self.namespaces.is_empty() && self.blocks.is_empty()
    }

    fn is_namespace(&self) -> bool {
        !self.namespaces.is_empty()
    }

    fn current_scope_name(&self) -> &Utf16Str {
        if self.is_root() {
            Utf16Str::new(&utf16!("<root>"))
        } else {
            Utf16Str::new(&utf16!("<anonymous>"))
        }
    }

    pub(crate) fn push_namespace_scope(&mut self, name: &'ast Utf16Str) {
        self.namespaces.push(NamespaceScope { name });
    }

    pub(crate) fn push_block_scope(&mut self) {
        self.blocks.push(BlockScope {
            variables: HashMap::new(),
        });
    }

    /// スタックのトップからローカルスコープを1つ破棄します。
    /// ローカルスコープがない場合はパニックします。
    pub(crate) fn drop_local_scope(&mut self) {
        if let None = self.blocks.pop() {
            if let None = self.namespaces.pop() {
                panic!("No local scopes");
            }
        }
    }

    /// 名前空間の変数名をルートにおける名前に解決します。
    fn resolve(&self, name: &NamePath) -> NamePath {
        let prefix_len = (self.namespaces.len() + 1).saturating_sub(name.segment_count());
        let mut result = NamePath::new();
        for namespace in self.namespaces.iter().take(prefix_len) {
            result.append(namespace.name);
        }
        result.append_path(name);
        return result;
    }

    fn get(&self, name: &NamePath) -> Option<&Variable> {
        for block in &self.blocks {
            if let Some(variable) = block.variables.get(name) {
                return Some(variable);
            }
        }

        if self.is_namespace() {
            if let Some(variable) = self.root.variables.get(&self.resolve(name)) {
                return Some(variable);
            }
        }

        return self.root.variables.get(name);
    }

    fn exists(&self, name: &NamePath) -> bool {
        self.get(name).is_some()
    }

    pub(crate) fn add(&mut self, name: &'ast NamePath, variable: Variable) {
        if let Some(block) = self.blocks.last_mut() {
            // ブロック
            block.variables.insert(name, variable);
        } else {
            if self.namespaces.is_empty() {
                // ルート
                self.root.variables.insert(Cow::Borrowed(name), variable);
            } else {
                // 名前空間
                self.root
                    .variables
                    .insert(Cow::Owned(self.resolve(name)), variable);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use aiscript_engine_common::Utf16String;

    use super::*;

    fn name_path(s: &str) -> NamePath {
        NamePath::from(Utf16String::from(s))
    }

    #[test]
    fn scope_name() {
        let mut scopes = Scopes::new();
        assert!(scopes.is_root());
        assert_eq!(
            scopes.current_scope_name(),
            Utf16Str::new(&utf16!("<root>"))
        );

        scopes.push_namespace_scope(Utf16Str::new(&utf16!("Ns")));
        assert!(!scopes.is_root());
        assert_eq!(
            scopes.current_scope_name(),
            Utf16Str::new(&utf16!("<anonymous>"))
        );

        scopes.drop_local_scope();
        assert!(scopes.is_root());
        assert_eq!(
            scopes.current_scope_name(),
            Utf16Str::new(&utf16!("<root>"))
        );

        scopes.push_block_scope();
        assert!(!scopes.is_root());
        assert_eq!(
            scopes.current_scope_name(),
            Utf16Str::new(&utf16!("<anonymous>"))
        );
    }

    #[test]
    #[should_panic]
    fn drop_no_local_scope() {
        let mut scopes = Scopes::new();
        scopes.drop_local_scope();
    }

    #[test]
    fn resolve() {
        let mut scopes = Scopes::new();
        scopes.push_namespace_scope(Utf16Str::new(&utf16!("Ns")));
        assert_eq!(scopes.resolve(&name_path("a")), name_path("Ns:a"));
        assert_eq!(scopes.resolve(&name_path("Ns:b")), name_path("Ns:b"));
        assert_eq!(scopes.resolve(&name_path("Ns:Ns:c")), name_path("Ns:Ns:c"));
        scopes.drop_local_scope();
        assert!(scopes.is_root());
    }

    #[test]
    fn add_to_root() {
        let mut scopes = Scopes::new();
        let variable_name = name_path("a");
        scopes.add(&variable_name, Default::default());
        assert!(scopes.exists(&name_path("a")));
        assert!(scopes.is_root());
    }

    #[test]
    fn add_to_namespace() {
        let mut scopes = Scopes::new();
        let variable_name = name_path("a");
        scopes.push_namespace_scope(Utf16Str::new(&utf16!("Ns")));
        scopes.add(&variable_name, Default::default());
        assert!(scopes.exists(&name_path("a")));
        assert!(scopes.exists(&name_path("Ns:a")));
        scopes.drop_local_scope();
        assert!(scopes.is_root());
    }

    #[test]
    fn add_to_block() {
        let mut scopes = Scopes::new();
        let variable_name = name_path("a");
        scopes.push_block_scope();
        scopes.add(&variable_name, Default::default());
        assert!(scopes.exists(&name_path("a")));
        scopes.drop_local_scope();
        assert!(scopes.is_root());
    }
}
