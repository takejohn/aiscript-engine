#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Location {
    At { line: usize, column: usize },
    EOF,
}
