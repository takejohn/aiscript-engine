use crate::{
    plugins::{validate_keyword, validate_type},
    syntaxes::toplevel::parse_top_level,
};
use aiscript_engine_ast::{self as ast};
use aiscript_engine_common::{Result, Utf16Str};
use aiscript_engine_lexer::Scanner;

pub type ParserPlugin = dyn FnMut(&mut Vec<ast::Node>) -> Result<()>;

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
            validate_plugins: vec![Box::new(validate_keyword), Box::new(validate_type)],
            transform_plugins: Vec::new(),
        };
    }

    pub fn add_plugin(&mut self, ty: PluginType, plugin: Box<ParserPlugin>) {
        match ty {
            PluginType::Validate => self.validate_plugins.push(plugin),
            PluginType::Transform => self.transform_plugins.push(plugin),
        }
    }

    pub fn parse(&mut self, input: &Utf16Str) -> Result<Vec<ast::Node>> {
        let mut scanner = Scanner::new(input)?;
        let mut nodes: Vec<ast::Node> = parse_top_level(&mut scanner)?;

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
