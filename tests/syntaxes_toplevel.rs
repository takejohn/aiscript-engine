extern crate aiscript_engine;

mod syntaxes_util;

use std::collections::HashMap;

use pretty_assertions::assert_eq;

use aiscript_engine::{ast, string::Utf16String};

use syntaxes_util::*;

#[test]
fn empty() {
    assert_eq!(parse(""), Vec::new());
    assert_eq!(parse("\n\n\n"), Vec::new());
}

#[test]
fn namespace() {
    assert_eq!(
        parse(":: Ns {}"),
        vec![ast::Namespace {
            loc: loc((1, 1), (1, 9)),
            name: Utf16String::from("Ns"),
            members: Vec::new(),
        }
        .into()]
    );
    assert_eq!(
        parse(
            r#":: Ns {
    let x = 42
    :: Inner {}
}"#
        ),
        vec![ast::Namespace {
            loc: loc((1, 1), (4, 2)),
            name: Utf16String::from("Ns"),
            members: vec![
                ast::Definition {
                    loc: loc((2, 5), (2, 15)),
                    dest: ast::Identifier {
                        loc: loc((2, 9), (2, 11)),
                        name: Utf16String::from("x"),
                    }
                    .into(),
                    var_type: None,
                    expr: ast::Num {
                        loc: loc((2, 13), (2, 15)),
                        value: 42.0,
                    }
                    .into(),
                    is_mut: false,
                    attr: Vec::new(),
                }
                .into(),
                ast::Namespace {
                    loc: loc((3, 5), (3, 16)),
                    name: Utf16String::from("Inner"),
                    members: Vec::new(),
                }
                .into()
            ],
        }
        .into()]
    );

    fails(":: {}");
    fails(":: Ns {");
}

#[test]
fn meta() {
    assert_eq!(
        parse("### {}"),
        vec![ast::Meta {
            loc: loc((1, 1), (1, 7)),
            name: None,
            value: ast::Obj {
                loc: loc((1, 5), (1, 7)),
                value: HashMap::new(),
            }
            .into(),
        }
        .into()]
    );
    assert_eq!(
        parse("### Name 42"),
        vec![ast::Meta {
            loc: loc((1, 1), (1, 12)),
            name: Some(Utf16String::from("Name")),
            value: ast::Num {
                loc: loc((1, 10), (1, 12)),
                value: 42.0,
            }
            .into(),
        }
        .into()]
    );
    fails("###");
}
