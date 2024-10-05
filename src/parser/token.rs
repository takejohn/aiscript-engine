use crate::{
    common::Position,
    string::{Utf16Str, Utf16String},
    utf16_str,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum TokenKind {
    EOF,
    NewLine,
    Identifier(Utf16String),

    // literal
    NumberLiteral(Utf16String),
    StringLiteral(Utf16String),

    // template string
    Template(Vec<Token>),
    TemplateStringElement(Utf16String),
    TemplateExprElement(Vec<Token>),

    // keyword
    NullKeyword,
    TrueKeyword,
    FalseKeyword,
    EachKeyword,
    ForKeyword,
    LoopKeyword,
    DoKeyword,
    WhileKeyword,
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

    /// "?"
    Question,

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

impl TokenKind {
    pub fn for_word(word: &Utf16Str) -> TokenKind {
        match word {
            v if v == utf16_str!('n', 'u', 'l', 'l') => TokenKind::NullKeyword,
            v if v == utf16_str!('t', 'r', 'u', 'e') => TokenKind::TrueKeyword,
            v if v == utf16_str!('f', 'a', 'l', 's', 'e') => TokenKind::FalseKeyword,
            v if v == utf16_str!('e', 'a', 'c', 'h') => TokenKind::EachKeyword,
            v if v == utf16_str!('f', 'o', 'r') => TokenKind::ForKeyword,
            v if v == utf16_str!('l', 'o', 'o', 'p') => TokenKind::LoopKeyword,
            v if v == utf16_str!('d', 'o') => TokenKind::DoKeyword,
            v if v == utf16_str!('w', 'h', 'i', 'l', 'e') => TokenKind::WhileKeyword,
            v if v == utf16_str!('b', 'r', 'e', 'a', 'k') => TokenKind::BreakKeyword,
            v if v == utf16_str!('c', 'o', 'n', 't', 'i', 'n', 'u', 'e') => {
                TokenKind::ContinueKeyword
            }
            v if v == utf16_str!('m', 'a', 't', 'c', 'h') => TokenKind::MatchKeyword,
            v if v == utf16_str!('c', 'a', 's', 'e') => TokenKind::CaseKeyword,
            v if v == utf16_str!('d', 'e', 'f', 'a', 'u', 'l', 't') => TokenKind::DefaultKeyword,
            v if v == utf16_str!('i', 'f') => TokenKind::IfKeyword,
            v if v == utf16_str!('e', 'l', 'i', 'f') => TokenKind::ElifKeyword,
            v if v == utf16_str!('e', 'l', 's', 'e') => TokenKind::ElseKeyword,
            v if v == utf16_str!('r', 'e', 't', 'u', 'r', 'n') => TokenKind::ReturnKeyword,
            v if v == utf16_str!('e', 'v', 'a', 'l') => TokenKind::EvalKeyword,
            v if v == utf16_str!('v', 'a', 'r') => TokenKind::VarKeyword,
            v if v == utf16_str!('l', 'e', 't') => TokenKind::LetKeyword,
            v if v == utf16_str!('e', 'x', 'i', 's', 't', 's') => TokenKind::ExistsKeyword,
            _ => TokenKind::Identifier(Utf16String::from(word)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Token {
    pub kind: TokenKind,
    pub pos: Position,
    pub has_left_spacing: bool,
}

pub const EOF: Token = Token {
    kind: TokenKind::EOF,
    pos: Position::EOF,
    has_left_spacing: false,
};
