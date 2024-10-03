use crate::{ast::Loc, common::Location};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum TokenKind {
    EOF,
    NewLine,
    Identifier,

    // literal
    NumberLiteral(String),
    StringLiteral(String),

    // template string
    Template(Vec<Token>),
    TemplateStringElement,
    TemplateExprElement(Vec<Token>),

    // keyword
    NullKeyword,
    TrueKeyword,
    FalseKeyword,
    EachKeyword,
    ForKeyword,
    LoopKeyword,
    BreakKeyword,
    ContinueKeyword,
    MatchKeyword,
    CaseKeyword,
    DefaultKeyword,
    IfKeyword,
    ElifKeyword,
    ElseKeyword,
    ReturnKeyword,
    EvalKeyword,
    VarKeyword,
    LetKeyword,
    ExistsKeyword,

    /// "!"
    Not,

    /// "!="
    NotEq,

    /// "#["
    OpenSharpBracket,

    /// "###"
    Sharp3,

    /// "%"
    Percent,

    /// "&&"
    And2,

    /// "("
    OpenParen,

    /// ")"
    CloseParen,

    /// "*"
    Asterisk,

    /// "+"
    Plus,

    /// "+="
    PlusEq,

    /// ","
    Comma,

    /// "-"
    Minus,

    /// "-="
    MinusEq,

    /// "."
    Dot,

    /// "/"
    Slash,

    /// ":"
    Colon,

    /// "::"
    Colon2,

    /// ";"
    SemiColon,

    /// "<"
    Lt,

    /// "<="
    LtEq,

    /// "<:"
    Out,

    /// "="
    Eq,

    /// "=="
    Eq2,

    /// "=>"
    Arrow,

    /// ">"
    Gt,

    /// ">="
    GtEq,

    /// "@"
    At,

    /// "["
    OpenBracket,

    /// "\\"
    BackSlash,

    /// "]"
    CloseBracket,

    /// "^"
    Hat,

    /// "{"
    OpenBrace,

    /// "||"
    Or2,

    /// "}"
    CloseBrace,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct Token {
    pub kind: TokenKind,
    pub loc: Location,
    pub has_left_spacing: bool,
}

pub const EOF: Token = Token {
    kind: TokenKind::EOF,
    loc: Location::EOF,
    has_left_spacing: false,
};
