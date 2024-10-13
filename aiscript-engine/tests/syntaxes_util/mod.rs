extern crate aiscript_engine;

use aiscript_engine::{ast, string::Utf16String, Parser};

pub(super) fn parse(source: &str) -> Vec<ast::Node> {
    let source = Utf16String::from(source);
    let mut parser = Parser::new();
    return parser.parse(&source).unwrap();
}

pub(super) fn fails(source: &str) {
    let source = Utf16String::from(source);
    let mut parser = Parser::new();
    assert!(parser.parse(&source).is_err());
}

pub(super) fn loc(start: (usize, usize), end: (usize, usize)) -> ast::Loc {
    ast::Loc {
        start: ast::Position::At {
            line: start.0,
            column: start.1,
        },
        end: ast::Position::At {
            line: end.0,
            column: end.1,
        },
    }
}
