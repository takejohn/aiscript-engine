use aiscript_engine_common::Utf16String;
use aiscript_engine_ir::{translate, Ir};
use aiscript_engine_parser::Parser;

pub fn to_ir(source: &str) -> Ir {
    let mut parser = Parser::new();
    let ast = parser.parse(&Utf16String::from(source)).unwrap();
    return translate(&ast);
}
