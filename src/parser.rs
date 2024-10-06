mod scanner;
mod streams;
mod syntaxes;
mod token;

use crate::{ast, error::Result, string::Utf16Str};
use scanner::Scanner;
use syntaxes::parse_top_level;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        return Parser {};
    }

    pub fn parse(&self, input: &Utf16Str) -> Result<Vec<ast::NodeWrapper>> {
        let mut scanner = Scanner::new(input)?;
        let nodes: Vec<ast::NodeWrapper> = parse_top_level(&mut scanner)?;

        return Ok(nodes);
    }
}
