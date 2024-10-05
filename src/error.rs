use std::fmt::Debug;

use crate::common::Location;

pub type Result<T> = core::result::Result<T, Box<dyn AiScriptError>>;

pub trait AiScriptError: Debug {
    fn name(&self) -> &'static str;

    fn message(&self) -> &str;

    fn loc(&self) -> Option<Location>;

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.loc() {
            Some(loc) => write!(f, "{}: {} ({})", self.name(), self.message(), loc),
            None => write!(f, "{}: {}", self.name(), self.message()),
        }
    }
}

/// Parse-time errors.
pub struct AiScriptSyntaxError {
    message: String,

    loc: Location,
}

impl AiScriptSyntaxError {
    pub fn new(message: impl Into<String>, loc: Location) -> AiScriptSyntaxError {
        AiScriptSyntaxError {
            message: message.into(),
            loc,
        }
    }
}

impl Debug for AiScriptSyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        AiScriptError::fmt(self, f)
    }
}

impl AiScriptError for AiScriptSyntaxError {
    fn name(&self) -> &'static str {
        "Syntax"
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn loc(&self) -> Option<Location> {
        Some(self.loc.clone())
    }
}
