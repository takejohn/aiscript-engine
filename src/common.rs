use std::fmt::{Debug, Display};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Position {
    At { line: usize, column: usize },
    EOF,
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Position::At { line, column } => write!(f, "{line}:{column}"),
            Position::EOF => write!(f, "EOF"),
        }
    }
}
