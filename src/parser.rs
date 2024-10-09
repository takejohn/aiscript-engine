mod plugins;
mod scanner;
mod streams;
mod syntaxes;
mod token;

use crate::{ast, error::Result, string::Utf16Str};
use plugins::validate_keyword;
use scanner::Scanner;
use syntaxes::parse_top_level;

pub type ParserPlugin = dyn FnMut(&mut Vec<ast::NodeWrapper>) -> Result<()>;

pub enum PluginType {
    Validate,
    Transform,
}

pub struct Parser {
    validate_plugins: Vec<Box<ParserPlugin>>,
    transform_plugins: Vec<Box<ParserPlugin>>,
}

impl Parser {
    pub fn new() -> Self {
        return Parser {
            validate_plugins: vec![Box::new(validate_keyword)],
            transform_plugins: Vec::new(),
        };
    }

    pub fn add_plugin(&mut self, ty: PluginType, plugin: Box<ParserPlugin>) {
        match ty {
            PluginType::Validate => self.validate_plugins.push(plugin),
            PluginType::Transform => self.transform_plugins.push(plugin),
        }
    }

    pub fn parse(&mut self, input: &Utf16Str) -> Result<Vec<ast::NodeWrapper>> {
        let mut scanner = Scanner::new(input)?;
        let mut nodes: Vec<ast::NodeWrapper> = parse_top_level(&mut scanner)?;

        // validate the node tree
        for plugin in &mut self.validate_plugins {
            plugin(&mut nodes)?;
        }

        // transform the node tree
        for plugin in &mut self.transform_plugins {
            plugin(&mut nodes)?;
        }

        return Ok(nodes);
    }
}
