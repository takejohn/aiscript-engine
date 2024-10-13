use std::fmt::Debug;

use crate::common::Position;

pub type Result<T> = core::result::Result<T, Box<dyn AiScriptError>>;

pub trait AiScriptError: Debug {
    fn name(&self) -> &'static str;

    fn message(&self) -> &str;

    fn pos(&self) -> Option<Position>;

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.pos() {
            Some(pos) => write!(f, "{}: {} ({})", self.name(), self.message(), pos),
            None => write!(f, "{}: {}", self.name(), self.message()),
        }
    }
}

/// Parse-time errors.
pub struct AiScriptSyntaxError {
    message: String,

    pos: Position,
}

impl AiScriptSyntaxError {
    pub fn new(message: impl Into<String>, pos: Position) -> AiScriptSyntaxError {
        AiScriptSyntaxError {
            message: message.into(),
            pos,
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

    fn pos(&self) -> Option<Position> {
        Some(self.pos.clone())
    }
}
