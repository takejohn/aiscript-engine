//! AiScript interpreter

use std::collections::HashMap;

use aiscript_engine_ast as ast;
use aiscript_engine_common::Result;
use aiscript_engine_ir::Translator;
use aiscript_engine_library::{std_library, LibraryValue, NativeFn};
use aiscript_engine_vm::{Value, Vm, VmState};
use utf16_literal::utf16;

pub trait InterpreterOpts {
    fn out(&mut self, value: Value);
}

pub struct Interpreter<'opts> {
    opts: &'opts mut dyn InterpreterOpts,
}

impl<'opts> Interpreter<'opts> {
    pub fn new(opts: &'opts mut dyn InterpreterOpts) -> Self {
        Interpreter { opts }
    }

    pub fn run(&mut self, program: &[ast::Node]) -> Result<Value> {
        let mut out = |args: Vec<Value>, _| {
            let mut args = args.into_iter();
            self.opts.out(args.next().unwrap()); // todo AiScriptエラーを返す
            Ok(Value::Null)
        };
        let lib = HashMap::from([(
            &utf16!("print") as &[u16],
            LibraryValue::Fn(NativeFn::Dynamic(&mut out)),
        )]);
        let mut translator = Translator::new();
        translator.link_library(std_library());
        translator.link_library(lib);
        translator.translate(&program);
        let mut ir = translator.build();
        let mut vm = Vm::new(&mut ir);
        while let VmState::Continue = vm.step()? {
            // nop
        }
        return Ok(Value::Null);
    }
}
