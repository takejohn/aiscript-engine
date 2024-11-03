use aiscript_engine_common::{Result, Utf16String};
use aiscript_engine_interpreter::{Interpreter, InterpreterOpts};
use aiscript_engine_parser::Parser;
use aiscript_engine_vm::Value;

struct TestOpts {
    result: Value,
}

impl InterpreterOpts for TestOpts {
    fn out(&mut self, value: Value) {
        self.result = value;
    }
}

pub(crate) fn exe(source: &str) -> Result<Value> {
    let mut parser = Parser::new();
    let ast = parser.parse(&Utf16String::from(source))?;
    let mut opts = TestOpts {
        result: Value::Uninitialized,
    };
    let mut interpreter = Interpreter::new(&mut opts);
    interpreter.run(&ast)?;
    return Ok(opts.result);
}
