use crate::common::Location;

pub type Result<T> = core::result::Result<T, Box<dyn AiScriptError>>;

pub trait AiScriptError {
    fn name(&self) -> &'static str;

    fn message(&self) -> &str;

    fn loc(&self) -> Option<Location>;
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
