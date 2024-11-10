use std::{cell::RefCell, rc::Rc};

use aiscript_engine::{Interpreter, InterpreterOpts, Parser, Result, Utf16String, Value};

struct TestOpts {
    result: Rc<RefCell<Value>>,
}

impl InterpreterOpts for TestOpts {
    fn out(&self, value: Value) {
        *self.result.borrow_mut() = value;
    }
}

pub(crate) fn exe(source: &str) -> Result<Value> {
    let mut parser = Parser::new();
    let ast = parser.parse(&Utf16String::from(source))?;
    let result = Rc::new(RefCell::new(Value::Uninitialized));
    let opts: Rc<dyn InterpreterOpts> = Rc::new(TestOpts {
        result: Rc::clone(&result),
    });
    let mut interpreter = Interpreter::new(Rc::clone(&opts));
    interpreter.run(&ast)?;
    return Ok(result.borrow().clone());
}
