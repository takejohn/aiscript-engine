use std::collections::VecDeque;

use utf16_literal::utf16;

use crate::{
    error::{AiScriptSyntaxError, Result},
    string::Utf16String,
};

use super::{
    streams::{CharStream, ITokenStream},
    token::{Token, TokenKind},
};

fn is_space_char(char: u16) -> bool {
    char == utf16!(' ') || char == utf16!('\t')
}

fn is_line_break_char(char: u16) -> bool {
    char == utf16!('\r') || char == utf16!('\n')
}

fn is_digit(char: u16) -> bool {
    char >= utf16!('0') && char <= utf16!('9')
}

fn is_word_char(char: u16) -> bool {
    (char >= utf16!('A') && char <= utf16!('Z'))
        || (char >= utf16!('a') && char <= utf16!('z'))
        || is_digit(char)
}

/// 入力文字列からトークンを読み取る構造体
pub(super) struct Scanner<'a> {
    stream: CharStream<'a>,
    tokens: VecDeque<Token>,
}

impl Scanner<'_> {
    pub fn new<'a>(stream: impl Into<CharStream<'a>>) -> Result<Scanner<'a>> {
        let mut scanner = Scanner {
            stream: stream.into(),
            tokens: VecDeque::new(),
        };
        let token = scanner.read_token()?;
        scanner.tokens.push_back(token);
        return Ok(scanner);
    }

    fn read_token(&mut self) -> Result<Token> {
        let mut has_left_spacing = false;

        loop {
            let Some(ch) = self.stream.char() else {
                return Ok(Token {
                    kind: TokenKind::EOF,
                    pos: self.stream.get_pos(),
                    has_left_spacing,
                });
            };
            // skip spasing
            if is_space_char(ch) {
                self.stream.next();
                has_left_spacing = true;
                continue;
            }

            // トークン位置を記憶
            let pos = self.stream.get_pos();

            if is_line_break_char(ch) {
                self.stream.next();
                return Ok(Token {
                    kind: TokenKind::NewLine,
                    pos,
                    has_left_spacing,
                });
            }

            return match ch {
                utf16!('!') => {
                    self.stream.next();
                    if self.stream.char().is_some_and(|char| char == utf16!('=')) {
                        self.stream.next();
                        Ok(Token {
                            kind: TokenKind::NotEq,
                            pos,
                            has_left_spacing,
                        })
                    } else {
                        Ok(Token {
                            kind: TokenKind::Not,
                            pos,
                            has_left_spacing,
                        })
                    }
                }
                utf16!('"') | utf16!('\'') => {
                    return self.read_string_literal(ch, has_left_spacing);
                }
                utf16!('#') => {
                    self.stream.next();
                    if self.stream.char().is_some_and(|ch| ch == utf16!('#')) {
                        self.stream.next();
                        if self.stream.char().is_some_and(|ch| ch == utf16!('#')) {
                            self.stream.next();
                            Ok(Token {
                                kind: TokenKind::Sharp3,
                                pos,
                                has_left_spacing,
                            })
                        } else {
                            Err(Box::new(AiScriptSyntaxError::new(
                                "invalid sequence of characters: \"##\"",
                                pos,
                            )))
                        }
                    } else if self.stream.char().is_some_and(|ch| ch == utf16!('[')) {
                        self.stream.next();
                        Ok(Token {
                            kind: TokenKind::OpenSharpBracket,
                            pos,
                            has_left_spacing,
                        })
                    } else {
                        Err(Box::new(AiScriptSyntaxError::new(
                            "invalid character: \"#\"",
                            pos,
                        )))
                    }
                }
                utf16!('%') => {
                    self.stream.next();
                    Ok(Token {
                        kind: TokenKind::Percent,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!('&') => {
                    self.stream.next();
                    if self.stream.char().is_some_and(|ch| ch == utf16!('&')) {
                        self.stream.next();
                        Ok(Token {
                            kind: TokenKind::And2,
                            pos,
                            has_left_spacing,
                        })
                    } else {
                        Err(Box::new(AiScriptSyntaxError::new(
                            "invalid character: \"&\"",
                            pos,
                        )))
                    }
                }
                utf16!('(') => {
                    self.stream.next();
                    Ok(Token {
                        kind: TokenKind::OpenParen,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!(')') => {
                    self.stream.next();
                    Ok(Token {
                        kind: TokenKind::CloseParen,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!('*') => {
                    self.stream.next();
                    Ok(Token {
                        kind: TokenKind::Asterisk,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!('+') => {
                    self.stream.next();
                    if self.stream.char().is_some_and(|ch| ch == utf16!('=')) {
                        self.stream.next();
                        Ok(Token {
                            kind: TokenKind::PlusEq,
                            pos,
                            has_left_spacing,
                        })
                    } else {
                        Ok(Token {
                            kind: TokenKind::Plus,
                            pos,
                            has_left_spacing,
                        })
                    }
                }
                utf16!(',') => {
                    self.stream.next();
                    Ok(Token {
                        kind: TokenKind::Comma,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!('-') => {
                    self.stream.next();
                    if self.stream.char().is_some_and(|ch| ch == utf16!('=')) {
                        self.stream.next();
                        Ok(Token {
                            kind: TokenKind::MinusEq,
                            pos,
                            has_left_spacing,
                        })
                    } else {
                        Ok(Token {
                            kind: TokenKind::Minus,
                            pos,
                            has_left_spacing,
                        })
                    }
                }
                utf16!('.') => {
                    self.stream.next();
                    Ok(Token {
                        kind: TokenKind::Dot,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!('/') => {
                    self.stream.next();
                    if let Some(ch) = self.stream.char() {
                        if ch == utf16!('*') {
                            self.stream.next();
                            self.skip_comment_range();
                            continue;
                        } else if ch == utf16!('/') {
                            self.stream.next();
                            self.skip_comment_line();
                            continue;
                        }
                    }
                    Ok(Token {
                        kind: TokenKind::Slash,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!(':') => {
                    self.stream.next();
                    if self.stream.char().is_some_and(|ch| ch == utf16!(':')) {
                        self.stream.next();
                        Ok(Token {
                            kind: TokenKind::Colon2,
                            pos,
                            has_left_spacing,
                        })
                    } else {
                        Ok(Token {
                            kind: TokenKind::Colon,
                            pos,
                            has_left_spacing,
                        })
                    }
                }
                utf16!(';') => {
                    self.stream.next();
                    Ok(Token {
                        kind: TokenKind::SemiColon,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!('<') => {
                    self.stream.next();
                    if let Some(ch) = self.stream.char() {
                        if ch == utf16!('=') {
                            self.stream.next();
                            return Ok(Token {
                                kind: TokenKind::LtEq,
                                pos,
                                has_left_spacing,
                            });
                        } else if ch == utf16!(':') {
                            self.stream.next();
                            return Ok(Token {
                                kind: TokenKind::Out,
                                pos,
                                has_left_spacing,
                            });
                        }
                    }
                    Ok(Token {
                        kind: TokenKind::Lt,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!('=') => {
                    self.stream.next();
                    if let Some(ch) = self.stream.char() {
                        if ch == utf16!('=') {
                            self.stream.next();
                            return Ok(Token {
                                kind: TokenKind::Eq2,
                                pos,
                                has_left_spacing,
                            });
                        } else if ch == utf16!('>') {
                            self.stream.next();
                            return Ok(Token {
                                kind: TokenKind::Arrow,
                                pos,
                                has_left_spacing,
                            });
                        } else {
                            return Ok(Token {
                                kind: TokenKind::Eq,
                                pos,
                                has_left_spacing,
                            });
                        }
                    }
                    Ok(Token {
                        kind: TokenKind::Eq,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!('>') => {
                    self.stream.next();
                    if self.stream.char().is_some_and(|ch| ch == utf16!('=')) {
                        self.stream.next();
                        Ok(Token {
                            kind: TokenKind::GtEq,
                            pos,
                            has_left_spacing,
                        })
                    } else {
                        Ok(Token {
                            kind: TokenKind::Gt,
                            pos,
                            has_left_spacing,
                        })
                    }
                }
                utf16!('?') => {
                    self.stream.next();
                    Ok(Token {
                        kind: TokenKind::Question,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!('@') => {
                    self.stream.next();
                    Ok(Token {
                        kind: TokenKind::At,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!('[') => {
                    self.stream.next();
                    Ok(Token {
                        kind: TokenKind::OpenBracket,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!('\\') => {
                    self.stream.next();
                    Ok(Token {
                        kind: TokenKind::BackSlash,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!(']') => {
                    self.stream.next();
                    Ok(Token {
                        kind: TokenKind::CloseBracket,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!('^') => {
                    self.stream.next();
                    Ok(Token {
                        kind: TokenKind::Hat,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!('`') => self.read_template(has_left_spacing),
                utf16!('{') => {
                    self.stream.next();
                    Ok(Token {
                        kind: TokenKind::OpenBrace,
                        pos,
                        has_left_spacing,
                    })
                }
                utf16!('|') => {
                    self.stream.next();
                    if self.stream.char().is_some_and(|ch| ch == utf16!('|')) {
                        self.stream.next();
                        Ok(Token {
                            kind: TokenKind::Or2,
                            pos,
                            has_left_spacing,
                        })
                    } else {
                        Err(Box::new(AiScriptSyntaxError::new(
                            "invalid character: \"|\"",
                            pos,
                        )))
                    }
                }
                utf16!('}') => {
                    self.stream.next();
                    Ok(Token {
                        kind: TokenKind::CloseBrace,
                        pos,
                        has_left_spacing,
                    })
                }
                _ => {
                    if let Some(token) = self.try_read_digits(has_left_spacing)? {
                        return Ok(token);
                    }

                    if let Some(token) = self.try_read_word(has_left_spacing) {
                        return Ok(token);
                    }

                    return Err(Box::new(AiScriptSyntaxError::new(
                        format!("invalid character: \"{}\"", Utf16String::from(ch)),
                        pos,
                    )));
                }
            };
        }
    }

    fn try_read_word(&mut self, has_left_spacing: bool) -> Option<Token> {
        // read a word
        let mut value = Utf16String::new();

        let pos = self.stream.get_pos();

        let mut char_of_eof = self.stream.char();
        while let Some(char) = char_of_eof {
            if !is_word_char(char) {
                break;
            }
            value.push(char);
            self.stream.next();
            char_of_eof = self.stream.char();
        }

        if value.is_empty() {
            return None;
        }

        return Some(Token {
            kind: TokenKind::for_word(&value),
            pos,
            has_left_spacing,
        });
    }

    fn try_read_digits(&mut self, has_left_spacing: bool) -> Result<Option<Token>> {
        let mut whole_number = Utf16String::new();
        let mut fractional = Utf16String::new();

        let pos = self.stream.get_pos();

        while let Some(ch) = self.stream.char() {
            if !is_digit(ch) {
                break;
            }
            whole_number.push(ch);
            self.stream.next();
        }
        if whole_number.is_empty() {
            return Ok(None);
        }
        if let Some(ch) = self.stream.char() {
            if ch == utf16!('.') {
                self.stream.next();
                while let Some(ch) = self.stream.char() {
                    if !is_digit(ch) {
                        break;
                    }
                    fractional.push(ch);
                    self.stream.next();
                }
                if fractional.is_empty() {
                    return Err(Box::new(AiScriptSyntaxError::new("digit expected", pos)));
                }
            }
        }
        let value = if !fractional.is_empty() {
            whole_number + utf16!('.') + fractional.as_utf16_str()
        } else {
            whole_number
        };
        return Ok(Some(Token {
            kind: TokenKind::NumberLiteral(value),
            pos,
            has_left_spacing,
        }));
    }

    fn read_string_literal(&mut self, literal_mark: u16, has_left_spacing: bool) -> Result<Token> {
        #[derive(PartialEq, Eq)]
        enum State {
            String,
            Escape,
        }

        let mut value = Utf16String::new();
        let mut state = State::String;

        let pos = self.stream.get_pos();
        self.stream.next();

        loop {
            match state {
                State::String => {
                    let Some(ch) = self.stream.char() else {
                        return Err(Box::new(AiScriptSyntaxError::new("unexpected EOF", pos)));
                    };
                    if ch == utf16!('\\') {
                        self.stream.next();
                        state = State::Escape;
                    } else if ch == literal_mark {
                        self.stream.next();
                        break;
                    } else {
                        value.push(ch);
                        self.stream.next();
                    }
                }
                State::Escape => {
                    let Some(ch) = self.stream.char() else {
                        return Err(Box::new(AiScriptSyntaxError::new("unexpected EOF", pos)));
                    };
                    value.push(ch);
                    self.stream.next();
                    state = State::String;
                }
            }
        }
        return Ok(Token {
            kind: TokenKind::StringLiteral(value),
            pos,
            has_left_spacing,
        });
    }

    fn read_template(&mut self, has_left_spacing: bool) -> Result<Token> {
        #[derive(PartialEq, Eq)]
        enum State {
            String,
            Escape,
            Expr,
        }

        let mut elements: Vec<Token> = Vec::new();
        let mut buf = Utf16String::new();
        let mut token_buf: Vec<Token> = Vec::new();
        let mut state = State::String;

        let pos = self.stream.get_pos();
        let mut element_pos = pos.clone();
        self.stream.next();

        loop {
            match state {
                State::String => {
                    let Some(ch) = self.stream.char() else {
                        // テンプレートの終了が無いままEOFに達した
                        return Err(Box::new(AiScriptSyntaxError::new("unexpected EOF", pos)));
                    };
                    if ch == utf16!('\\') {
                        // エスケープ
                        self.stream.next();
                        state = State::Escape;
                    } else if ch == utf16!('`') {
                        // テンプレートの終了
                        self.stream.next();
                        if !buf.is_empty() {
                            elements.push(Token {
                                kind: TokenKind::TemplateStringElement(buf),
                                pos: element_pos,
                                has_left_spacing,
                            });
                        }
                        break;
                    } else if ch == utf16!('{') {
                        // 埋め込み式の開始
                        self.stream.next();
                        if !buf.is_empty() {
                            elements.push(Token {
                                kind: TokenKind::TemplateStringElement(buf),
                                pos: element_pos,
                                has_left_spacing,
                            });
                            buf = Utf16String::new();
                        }
                        // ここから式エレメントになるので位置を更新
                        element_pos = self.stream.get_pos();
                        state = State::Expr;
                    } else {
                        buf.push(ch);
                        self.stream.next();
                    }
                }
                State::Escape => {
                    let Some(ch) = self.stream.char() else {
                        // エスケープ対象の文字が無いままEOFに達した
                        return Err(Box::new(AiScriptSyntaxError::new("unexpected EOF", pos)));
                    };
                    // 普通の文字列として取り込み
                    buf.push(ch);
                    self.stream.next();
                    // 通常の文字列に戻る
                    state = State::String;
                }
                State::Expr => {
                    let Some(ch) = self.stream.char() else {
                        // 埋め込み式の終端記号が無いままEOFに達した
                        return Err(Box::new(AiScriptSyntaxError::new("unexpected EOF", pos)));
                    };
                    // skip spacing
                    if is_space_char(ch) {
                        self.stream.next();
                        continue;
                    }
                    // 埋め込み式の終了
                    if ch == utf16!('}') {
                        let expr_element_pos = element_pos.clone();
                        // ここから文字列エレメントになるので位置を更新
                        element_pos = self.stream.get_pos();
                        // TemplateExprElementトークンの終了位置をTokenStreamが取得するためのEOFトークンを追加
                        token_buf.push(Token {
                            kind: TokenKind::EOF,
                            pos: element_pos.clone(),
                            has_left_spacing: false,
                        });
                        elements.push(Token {
                            kind: TokenKind::TemplateExprElement(token_buf),
                            pos: expr_element_pos,
                            has_left_spacing,
                        });
                        token_buf = Vec::new();
                        state = State::String;
                        self.stream.next();
                    } else {
                        let token = self.read_token()?;
                        token_buf.push(token);
                    }
                }
            }
        }

        return Ok(Token {
            kind: TokenKind::Template(elements),
            pos,
            has_left_spacing,
        });
    }

    fn skip_comment_line(&mut self) {
        while self.stream.char().is_some_and(|ch| ch != utf16!('\n')) {
            self.stream.next();
        }
    }

    fn skip_comment_range(&mut self) {
        loop {
            let Some(ch) = self.stream.char() else {
                break;
            };
            if ch == utf16!('*') {
                self.stream.next();
                if self.stream.char().is_some_and(|ch| ch == utf16!('/')) {
                    self.stream.next();
                    break;
                }
                continue;
            }
            self.stream.next();
        }
    }
}

impl ITokenStream for Scanner<'_> {
    fn get_token(&self) -> &Token {
        self.tokens.get(0).expect("no token found")
    }

    fn next(&mut self) -> Result<()> {
        // 現在のトークンがEOFだったら次のトークンに進まない
        if self.get_token().kind == TokenKind::EOF {
            return Ok(());
        }

        self.tokens.pop_front();

        if self.tokens.is_empty() {
            let token = self.read_token();
            self.tokens.push_back(token?);
        }

        return Ok(());
    }

    fn lookahead(&mut self, offset: usize) -> Result<&Token> {
        while self.tokens.len() <= offset {
            let token = self.read_token();
            self.tokens.push_back(token?);
        }

        return Ok(self.tokens.get(offset).expect("no token found"));
    }
}

#[cfg(test)]
mod tests {
    use std::borrow::Borrow;

    use crate::{common::Position, is_token_kind, string::Utf16Str};

    use super::*;

    fn init(source: &Utf16Str) -> Scanner {
        Scanner::new(source).unwrap()
    }

    fn next(stream: &mut Scanner, token: impl Borrow<Token>) {
        assert_eq!(stream.get_token(), token.borrow());
        stream.next().unwrap();
    }

    fn fails(source: &Utf16Str) {
        assert!(Scanner::new(source).is_err());
    }

    #[test]
    fn eof() {
        let source = Utf16String::from("");
        let mut stream = init(&source);
        next(
            &mut stream,
            &Token {
                kind: TokenKind::EOF,
                pos: Position::At { line: 1, column: 1 },
                has_left_spacing: false,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::EOF,
                pos: Position::At { line: 1, column: 1 },
                has_left_spacing: false,
            },
        );
    }

    #[test]
    fn keyword() {
        let source = Utf16String::from("if");
        let mut stream = init(&source);
        next(
            &mut stream,
            &Token {
                kind: TokenKind::IfKeyword,
                pos: Position::At { line: 1, column: 1 },
                has_left_spacing: false,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::EOF,
                pos: Position::At { line: 1, column: 3 },
                has_left_spacing: false,
            },
        );
    }

    #[test]
    fn identifier() {
        let source = Utf16String::from("xyz");
        let mut stream = init(&source);
        next(
            &mut stream,
            &Token {
                kind: TokenKind::Identifier(source.clone()),
                pos: Position::At { line: 1, column: 1 },
                has_left_spacing: false,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::EOF,
                pos: Position::At { line: 1, column: 4 },
                has_left_spacing: false,
            },
        );
    }

    #[test]
    fn invalid_token() {
        let source = Utf16String::from("$");
        assert!(Scanner::new(source.as_utf16_str()).is_err());
    }

    #[test]
    fn words() {
        let source = Utf16String::from("abc xyz");
        let mut stream = init(&source);
        next(
            &mut stream,
            &Token {
                kind: TokenKind::Identifier(Utf16String::from("abc")),
                pos: Position::At { line: 1, column: 1 },
                has_left_spacing: false,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::Identifier(Utf16String::from("xyz")),
                pos: Position::At { line: 1, column: 5 },
                has_left_spacing: true,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::EOF,
                pos: Position::At { line: 1, column: 8 },
                has_left_spacing: false,
            },
        );
    }

    #[test]
    fn stream() {
        let source = Utf16String::from("@abc() { }");
        let mut stream = init(&source);
        next(
            &mut stream,
            &Token {
                kind: TokenKind::At,
                pos: Position::At { line: 1, column: 1 },
                has_left_spacing: false,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::Identifier(Utf16String::from("abc")),
                pos: Position::At { line: 1, column: 2 },
                has_left_spacing: false,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::OpenParen,
                pos: Position::At { line: 1, column: 5 },
                has_left_spacing: false,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::CloseParen,
                pos: Position::At { line: 1, column: 6 },
                has_left_spacing: false,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::OpenBrace,
                pos: Position::At { line: 1, column: 8 },
                has_left_spacing: true,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::CloseBrace,
                pos: Position::At {
                    line: 1,
                    column: 10,
                },
                has_left_spacing: true,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::EOF,
                pos: Position::At {
                    line: 1,
                    column: 11,
                },
                has_left_spacing: false,
            },
        );
    }

    #[test]
    fn multi_lines() {
        let source = Utf16String::from("aaa\nbbb");
        let mut stream = init(&source);
        next(
            &mut stream,
            &Token {
                kind: TokenKind::Identifier(Utf16String::from("aaa")),
                pos: Position::At { line: 1, column: 1 },
                has_left_spacing: false,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::NewLine,
                pos: Position::At { line: 1, column: 4 },
                has_left_spacing: false,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::Identifier(Utf16String::from("bbb")),
                pos: Position::At { line: 2, column: 1 },
                has_left_spacing: false,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::EOF,
                pos: Position::At { line: 2, column: 4 },
                has_left_spacing: false,
            },
        );
    }

    #[test]
    fn lookahead() {
        let source = Utf16String::from("@abc() { }");
        let mut stream = init(&source);
        assert_eq!(
            stream.lookahead(1).unwrap(),
            &Token {
                kind: TokenKind::Identifier(Utf16String::from("abc")),
                pos: Position::At { line: 1, column: 2 },
                has_left_spacing: false
            }
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::At,
                pos: Position::At { line: 1, column: 1 },
                has_left_spacing: false,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::Identifier(Utf16String::from("abc")),
                pos: Position::At { line: 1, column: 2 },
                has_left_spacing: false,
            },
        );
        next(
            &mut stream,
            &Token {
                kind: TokenKind::OpenParen,
                pos: Position::At { line: 1, column: 5 },
                has_left_spacing: false,
            },
        );
    }

    #[test]
    fn symbols() {
        let source = Utf16Str::new(&utf16!("="));
        let mut stream = init(source);
        next(
            &mut stream,
            Token {
                kind: TokenKind::Eq,
                pos: Position::At { line: 1, column: 1 },
                has_left_spacing: false,
            },
        );

        fails(Utf16Str::new(&utf16!("##")));
        fails(Utf16Str::new(&utf16!("#")));
        fails(Utf16Str::new(&utf16!("&")));
        fails(Utf16Str::new(&utf16!("|")));
    }

    #[test]
    fn numbers() {
        let source = Utf16Str::new(&utf16!("1.23\n4.56"));
        let mut stream = init(source);
        next(
            &mut stream,
            Token {
                kind: TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("1.23"))),
                pos: Position::At { line: 1, column: 1 },
                has_left_spacing: false,
            },
        );
        next(
            &mut stream,
            Token {
                kind: TokenKind::NewLine,
                pos: Position::At { line: 1, column: 5 },
                has_left_spacing: false,
            },
        );
        next(
            &mut stream,
            Token {
                kind: TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("4.56"))),
                pos: Position::At { line: 2, column: 1 },
                has_left_spacing: false,
            },
        );

        fails(Utf16Str::new(&utf16!("1.")));
    }

    #[test]
    fn strings() {
        let source = Utf16Str::new(&utf16!(r#""a\\b""#));
        let mut stream = init(source);
        next(
            &mut stream,
            Token {
                kind: TokenKind::StringLiteral(Utf16String::from_iter(&utf16!("a\\b"))),
                pos: Position::At { line: 1, column: 1 },
                has_left_spacing: false,
            },
        );

        fails(Utf16Str::new(&utf16!(r#"""#)));
        fails(Utf16Str::new(&utf16!(r#""\"#)));
    }

    #[test]
    fn comments() {
        let source = Utf16Str::new(&utf16!("/**"));
        let mut stream = init(source);
        next(
            &mut stream,
            Token {
                kind: TokenKind::EOF,
                pos: Position::At { line: 1, column: 4 },
                has_left_spacing: false,
            },
        );
    }

    #[test]
    fn templates() {
        let source = Utf16Str::new(&utf16!(r#"`{true}&{false }\\`"#));
        let mut stream = init(source);
        next(
            &mut stream,
            Token {
                kind: TokenKind::Template(vec![
                    Token {
                        kind: TokenKind::TemplateExprElement(vec![
                            Token {
                                kind: TokenKind::TrueKeyword,
                                pos: Position::At { line: 1, column: 3 },
                                has_left_spacing: false,
                            },
                            Token {
                                kind: TokenKind::EOF,
                                pos: Position::At { line: 1, column: 7 },
                                has_left_spacing: false,
                            },
                        ]),
                        pos: Position::At { line: 1, column: 3 },
                        has_left_spacing: false,
                    },
                    Token {
                        kind: TokenKind::TemplateStringElement(Utf16String::from_iter(&utf16!(
                            "&"
                        ))),
                        pos: Position::At { line: 1, column: 7 },
                        has_left_spacing: false,
                    },
                    Token {
                        kind: TokenKind::TemplateExprElement(vec![
                            Token {
                                kind: TokenKind::FalseKeyword,
                                pos: Position::At {
                                    line: 1,
                                    column: 10,
                                },
                                has_left_spacing: false,
                            },
                            Token {
                                kind: TokenKind::EOF,
                                pos: Position::At {
                                    line: 1,
                                    column: 16,
                                },
                                has_left_spacing: false,
                            },
                        ]),
                        pos: Position::At {
                            line: 1,
                            column: 10,
                        },
                        has_left_spacing: false,
                    },
                    Token {
                        kind: TokenKind::TemplateStringElement(Utf16String::from_iter(&utf16!(
                            "\\"
                        ))),
                        pos: Position::At {
                            line: 1,
                            column: 16,
                        },
                        has_left_spacing: false,
                    },
                ]),
                pos: Position::At { line: 1, column: 1 },
                has_left_spacing: false,
            },
        );

        fails(Utf16Str::new(&utf16!(r#"`"#)));
        fails(Utf16Str::new(&utf16!(r#"`\"#)));
        fails(Utf16Str::new(&utf16!(r#"`{"#)));
    }

    #[test]
    fn tokens() {
        let source = Utf16Str::new(&utf16!(
            r#"
/// line comment
/*
block comment
*/
### {}
:: Ns {}
#[test]
let o = {
    f: @(x?: str) {
        if x != null <: x
    }
    a: [2 * (3 / 4) \
        % 5 ^ 6,
        ``]
}
!true && false || 1 < 2
o.f("abc")
1 + 1 == 2; 1 <= 3; +3 > -5; 4 >= 2
var x = 0
while x < 3 x += 2
do x -= 1 while x > 0
for 5 {
    <: 'Hello'
}
each let item, o.a {
    <: item
}
match x {default => 42}
            "#
        ));
        let mut stream = init(source);
        let expected = [
            TokenKind::NewLine,
            TokenKind::NewLine,
            TokenKind::NewLine,
            TokenKind::Sharp3,
            TokenKind::OpenBrace,
            TokenKind::CloseBrace,
            TokenKind::NewLine,
            TokenKind::Colon2,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("Ns"))),
            TokenKind::OpenBrace,
            TokenKind::CloseBrace,
            TokenKind::NewLine,
            TokenKind::OpenSharpBracket,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("test"))),
            TokenKind::CloseBracket,
            TokenKind::NewLine,
            TokenKind::LetKeyword,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("o"))),
            TokenKind::Eq,
            TokenKind::OpenBrace,
            TokenKind::NewLine,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("f"))),
            TokenKind::Colon,
            TokenKind::At,
            TokenKind::OpenParen,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("x"))),
            TokenKind::Question,
            TokenKind::Colon,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("str"))),
            TokenKind::CloseParen,
            TokenKind::OpenBrace,
            TokenKind::NewLine,
            TokenKind::IfKeyword,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("x"))),
            TokenKind::NotEq,
            TokenKind::NullKeyword,
            TokenKind::Out,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("x"))),
            TokenKind::NewLine,
            TokenKind::CloseBrace,
            TokenKind::NewLine,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("a"))),
            TokenKind::Colon,
            TokenKind::OpenBracket,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("2"))),
            TokenKind::Asterisk,
            TokenKind::OpenParen,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("3"))),
            TokenKind::Slash,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("4"))),
            TokenKind::CloseParen,
            TokenKind::BackSlash,
            TokenKind::NewLine,
            TokenKind::Percent,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("5"))),
            TokenKind::Hat,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("6"))),
            TokenKind::Comma,
            TokenKind::NewLine,
            TokenKind::Template(Vec::new()),
            TokenKind::CloseBracket,
            TokenKind::NewLine,
            TokenKind::CloseBrace,
            TokenKind::NewLine,
            TokenKind::Not,
            TokenKind::TrueKeyword,
            TokenKind::And2,
            TokenKind::FalseKeyword,
            TokenKind::Or2,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("1"))),
            TokenKind::Lt,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("2"))),
            TokenKind::NewLine,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("o"))),
            TokenKind::Dot,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("f"))),
            TokenKind::OpenParen,
            TokenKind::StringLiteral(Utf16String::from_iter(&utf16!("abc"))),
            TokenKind::CloseParen,
            TokenKind::NewLine,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("1"))),
            TokenKind::Plus,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("1"))),
            TokenKind::Eq2,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("2"))),
            TokenKind::SemiColon,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("1"))),
            TokenKind::LtEq,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("3"))),
            TokenKind::SemiColon,
            TokenKind::Plus,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("3"))),
            TokenKind::Gt,
            TokenKind::Minus,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("5"))),
            TokenKind::SemiColon,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("4"))),
            TokenKind::GtEq,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("2"))),
            TokenKind::NewLine,
            TokenKind::VarKeyword,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("x"))),
            TokenKind::Eq,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("0"))),
            TokenKind::NewLine,
            TokenKind::WhileKeyword,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("x"))),
            TokenKind::Lt,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("3"))),
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("x"))),
            TokenKind::PlusEq,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("2"))),
            TokenKind::NewLine,
            TokenKind::DoKeyword,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("x"))),
            TokenKind::MinusEq,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("1"))),
            TokenKind::WhileKeyword,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("x"))),
            TokenKind::Gt,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("0"))),
            TokenKind::NewLine,
            TokenKind::ForKeyword,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("5"))),
            TokenKind::OpenBrace,
            TokenKind::NewLine,
            TokenKind::Out,
            TokenKind::StringLiteral(Utf16String::from_iter(&utf16!("Hello"))),
            TokenKind::NewLine,
            TokenKind::CloseBrace,
            TokenKind::NewLine,
            TokenKind::EachKeyword,
            TokenKind::LetKeyword,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("item"))),
            TokenKind::Comma,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("o"))),
            TokenKind::Dot,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("a"))),
            TokenKind::OpenBrace,
            TokenKind::NewLine,
            TokenKind::Out,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("item"))),
            TokenKind::NewLine,
            TokenKind::CloseBrace,
            TokenKind::NewLine,
            TokenKind::MatchKeyword,
            TokenKind::Identifier(Utf16String::from_iter(&utf16!("x"))),
            TokenKind::OpenBrace,
            TokenKind::DefaultKeyword,
            TokenKind::Arrow,
            TokenKind::NumberLiteral(Utf16String::from_iter(&utf16!("42"))),
            TokenKind::CloseBrace,
            TokenKind::NewLine,
        ];
        let mut expected_iter = expected.iter();
        while !is_token_kind!(&stream, TokenKind::EOF) {
            let kind = stream.get_token_kind();
            assert_eq!(kind, expected_iter.next().unwrap());
            stream.next().unwrap();
        }
        assert!(expected_iter.next().is_none());
    }
}
