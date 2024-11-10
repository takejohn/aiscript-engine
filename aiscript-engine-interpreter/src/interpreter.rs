//! AiScript interpreter

use std::collections::HashMap;
use std::rc::Rc;

use aiscript_engine_ast as ast;
use aiscript_engine_common::Result;
use utf16_literal::utf16;

use crate::ir::Translator;
use crate::library::{std_library, LibraryValue, NativeFn};
use crate::vm::{Value, Vm};

pub trait InterpreterOpts {
    fn out(&self, value: Value);
}

pub struct Interpreter {
    opts: Rc<dyn InterpreterOpts>,
}

impl Interpreter {
    pub fn new(opts: Rc<dyn InterpreterOpts>) -> Self {
        Interpreter { opts }
    }

    pub fn run(&mut self, program: &[ast::Node]) -> Result<Value> {
        let opts = Rc::clone(&self.opts);
        let lib = HashMap::from([(
            &utf16!("print") as &[u16],
            LibraryValue::Fn(NativeFn::Dynamic(Rc::new(
                move |args: Vec<Value>, vm: &mut Vm| {
                    let mut args = args.into_iter();
                    opts.out(args.next().unwrap()); // todo AiScriptエラーを返す
                    Ok(Value::Null)
                },
            ))),
        )]);
        let mut translator = Translator::new();
        translator.link_library(std_library());
        translator.link_library(lib);
        translator.translate(&program);
        let ir = translator.build();
        let mut vm = Vm::new();
        for native_fn in ir.native_functions {
            vm.register_native_fn(native_fn);
        }
        vm.exec(&ir.entry_point)?;
        return Ok(Value::Null);
    }
}
