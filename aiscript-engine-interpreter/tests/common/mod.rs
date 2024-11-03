use aiscript_engine_common::{Result, Utf16String};
use aiscript_engine_interpreter::Interpreter;
use aiscript_engine_parser::Parser;
use aiscript_engine_vm::Value;

pub(crate) fn exe(source: &str) -> Result<Value> {
    let mut parser = Parser::new();
    let ast = parser.parse(&Utf16String::from(source))?;
    let mut interpreter = Interpreter::new();
    return interpreter.run(&ast);
}
