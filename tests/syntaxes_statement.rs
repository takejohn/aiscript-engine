extern crate aiscript_engine;

mod syntaxes_util;

use aiscript_engine::{ast, string::Utf16String};
use indoc::indoc;
use pretty_assertions::assert_eq;
use syntaxes_util::*;

#[test]
fn var_def() {
    assert_eq!(
        parse("let a =\n42"),
        vec![ast::Statement::Def(ast::Definition {
            loc: loc((1, 1), (2, 3)),
            dest: ast::Identifier {
                loc: loc((1, 5), (1, 7)),
                name: Utf16String::from("a"),
            }
            .into(),
            var_type: None,
            expr: ast::Num {
                loc: loc((2, 1), (2, 3)),
                value: 42.0,
            }
            .into(),
            is_mut: false,
            attr: Vec::new(),
        })
        .into()]
    );

    assert_eq!(
        parse("let a: num = 42"),
        vec![ast::Statement::Def(ast::Definition {
            loc: loc((1, 1), (1, 16)),
            dest: ast::Identifier {
                loc: loc((1, 5), (1, 6)),
                name: Utf16String::from("a"),
            }
            .into(),
            var_type: Some(
                ast::NamedTypeSource {
                    loc: loc((1, 8), (1, 12)),
                    name: Utf16String::from("num"),
                    inner: None,
                }
                .into()
            ),
            expr: ast::Num {
                loc: loc((1, 14), (1, 16)),
                value: 42.0,
            }
            .into(),
            is_mut: false,
            attr: Vec::new(),
        })
        .into()]
    );

    fails("let");
    fails("var");
}

#[test]
fn fn_def() {
    assert_eq!(
        parse(indoc! {"
        @f(x: num): num {
            x
        }
    "}),
        vec![ast::Statement::Def(ast::Definition {
            loc: loc((1, 1), (3, 2)),
            dest: ast::Identifier {
                loc: loc((1, 2), (1, 3)),
                name: Utf16String::from("f"),
            }
            .into(),
            var_type: None,
            expr: ast::Fn {
                loc: loc((1, 1), (3, 2)),
                args: vec![ast::FnArg {
                    dest: ast::Identifier {
                        loc: loc((1, 4), (1, 5)),
                        name: Utf16String::from("x"),
                    }
                    .into(),
                    value: ast::FnArgValue::Required { default: None },
                    arg_type: Some(
                        ast::NamedTypeSource {
                            loc: loc((1, 7), (1, 10)),
                            name: Utf16String::from("num"),
                            inner: None,
                        }
                        .into()
                    )
                }],
                ret_type: Some(
                    ast::NamedTypeSource {
                        loc: loc((1, 13), (1, 17)),
                        name: Utf16String::from("num"),
                        inner: None,
                    }
                    .into()
                ),
                children: vec![ast::Expression::Identifier(ast::Identifier {
                    loc: loc((2, 5), (2, 6)),
                    name: Utf16String::from("x"),
                })
                .into()],
            }
            .into(),
            is_mut: false,
            attr: Vec::new(),
        })
        .into()]
    );

    fails("@");
}
