use std::{borrow::Cow, fmt::Debug};

use crate::position::Position;

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

pub enum AiScriptBasicErrorKind {
    /// Parse-time errors.
    Syntax,

    /// Namespace collection errors.
    Namespace,
}

impl AiScriptBasicErrorKind {
    pub fn name(&self) -> &'static str {
        match self {
            AiScriptBasicErrorKind::Syntax => "Syntax",
            AiScriptBasicErrorKind::Namespace => "Namespace",
        }
    }
}

pub struct AiScriptBasicError {
    kind: AiScriptBasicErrorKind,

    message: Cow<'static, str>,

    pos: Option<Position>,
}

impl AiScriptBasicError {
    pub fn new(
        kind: AiScriptBasicErrorKind,
        message: impl Into<Cow<'static, str>>,
        pos: Option<Position>,
    ) -> Self {
        AiScriptBasicError {
            kind,
            message: message.into(),
            pos,
        }
    }
}

impl AiScriptError for AiScriptBasicError {
    fn name(&self) -> &'static str {
        self.kind.name()
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn pos(&self) -> Option<Position> {
        self.pos.clone()
    }
}

impl Debug for AiScriptBasicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        AiScriptError::fmt(self, f)
    }
}

/// Parse-time errors.
/// TODO: このラッパーを廃止
pub struct AiScriptSyntaxError(AiScriptBasicError);

impl AiScriptSyntaxError {
    pub fn new(message: impl Into<Cow<'static, str>>, pos: Position) -> AiScriptSyntaxError {
        AiScriptSyntaxError(AiScriptBasicError::new(
            AiScriptBasicErrorKind::Syntax,
            message,
            Some(pos),
        ))
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
        self.0.message()
    }

    fn pos(&self) -> Option<Position> {
        self.0.pos()
    }
}
