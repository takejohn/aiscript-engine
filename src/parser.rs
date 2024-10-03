use crate::ast;

mod scanner;
mod streams;
mod token;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        return Parser {};
    }

    pub fn parse(&self, input: &str) {
        let mut nodes: Vec<ast::Node> = Vec::new();
    }
}
