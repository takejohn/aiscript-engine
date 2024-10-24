use std::collections::VecDeque;

use utf16_literal::utf16;

use aiscript_engine_common::{Position, Utf16Str, Utf16String};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TokenKind {
    EOF,
    NewLine,
    Identifier(Utf16String),

    // literal
    NumberLiteral(Utf16String),
    StringLiteral(Utf16String),

    /* template string
    TODO: TemplateStringElement, TemplateExprElementを列挙型TemplateElementとし、
    TokenをトレイトとしてTemplateElementにTokenを実装する

    enum TemplateElement {
        String(Utf16String),
        Expr(Vec<Token>),
    }

    impl Token for TemplateElement {
        ...
    }
    */
    Template(Vec<Token>),
    TemplateStringElement(Utf16String),
    TemplateExprElement(VecDeque<Token>),

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
        match word.as_u16s() {
            &utf16!("null") => TokenKind::NullKeyword,
            &utf16!("true") => TokenKind::TrueKeyword,
            &utf16!("false") => TokenKind::FalseKeyword,
            &utf16!("each") => TokenKind::EachKeyword,
            &utf16!("for") => TokenKind::ForKeyword,
            &utf16!("loop") => TokenKind::LoopKeyword,
            &utf16!("do") => TokenKind::DoKeyword,
            &utf16!("while") => TokenKind::WhileKeyword,
            &utf16!("break") => TokenKind::BreakKeyword,
            &utf16!("continue") => TokenKind::ContinueKeyword,
            &utf16!("match") => TokenKind::MatchKeyword,
            &utf16!("case") => TokenKind::CaseKeyword,
            &utf16!("default") => TokenKind::DefaultKeyword,
            &utf16!("if") => TokenKind::IfKeyword,
            &utf16!("elif") => TokenKind::ElifKeyword,
            &utf16!("else") => TokenKind::ElseKeyword,
            &utf16!("return") => TokenKind::ReturnKeyword,
            &utf16!("eval") => TokenKind::EvalKeyword,
            &utf16!("var") => TokenKind::VarKeyword,
            &utf16!("let") => TokenKind::LetKeyword,
            &utf16!("exists") => TokenKind::ExistsKeyword,
            _ => TokenKind::Identifier(Utf16String::from(word)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: Position,
    pub has_left_spacing: bool,
}

pub struct RawToken {
    pub raw: Utf16String,
    pub pos: Position,
    pub has_left_spacing: bool,
}

pub const EOF: Token = Token {
    kind: TokenKind::EOF,
    pos: Position::EOF,
    has_left_spacing: false,
};
