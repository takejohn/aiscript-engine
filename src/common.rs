use std::fmt::{Debug, Display};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Location {
    At { line: usize, column: usize },
    EOF,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Location::At { line, column } => write!(f, "{line}:{column}"),
            Location::EOF => write!(f, "EOF"),
        }
    }
}
