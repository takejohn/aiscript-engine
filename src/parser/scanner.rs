use std::collections::VecDeque;

use crate::error::AiScriptSyntaxError;

use super::{
    streams::{CharStream, ITokenStream},
    token::{Token, TokenKind, EOF},
};

fn is_space_char(char: u16) -> bool {
    char == ' ' as u16 || char == '\t' as u16
}

fn is_line_break_char(char: u16) -> bool {
    char == '\r' as u16 || char == '\n' as u16
}

fn is_digit(char: u16) -> bool {
    char >= '0' as u16 && char <= '9' as u16
}

fn is_word_char(char: u16) -> bool {
    (char >= 'A' as u16 && char <= 'Z' as u16)
        || (char >= 'a' as u16 && char <= 'z' as u16)
        || is_digit(char)
}

/// 入力文字列からトークンを読み取るトレイト
pub(super) struct Scanner {
    stream: CharStream,
    tokens: VecDeque<Token>,
}

impl Scanner {
    pub fn new(stream: impl Into<CharStream>) -> Scanner {
        Scanner {
            stream: stream.into(),
            tokens: VecDeque::new(),
        }
        // TODO: read token
    }

    fn read_token(&mut self) -> Token {
        let mut has_left_spacing = false;

        loop {
            let char = self.stream.char();
            match char {
                Err(_) => return EOF,
                Ok(char) => {
                    // skip spasing
                    if char == ' ' as u16 || char == '\t' as u16 {
                        self.stream.next();
                        has_left_spacing = true;
                        continue;
                    }

                    // トークン位置を記憶
                    let loc = self.stream.get_pos();

                    if is_line_break_char(char) {
                        self.stream.next();
                        return Token {
                            kind: TokenKind::NewLine,
                            loc,
                            has_left_spacing,
                        }
                    }
                    match char {
                        '!' => {},
                        _ => {}
                    }
                }
            }
        }
    }
}

impl ITokenStream for Scanner {
    fn token(&self) -> &Token {
        self.tokens.get(0).expect("no token found")
    }

    fn get_kind(&self) -> &super::token::TokenKind {
        &self.token().kind
    }

    fn next(&mut self) {
        // 現在のトークンがEOFだったら次のトークンに進まない
        if self.token().kind == TokenKind::EOF {
            return;
        }

        self.tokens.pop_front();

        if self.tokens.is_empty() {
            let token = self.read_token();
            self.tokens.push_back(token);
        }
    }

    fn lookahead(&mut self, offset: usize) -> &Token {
        while self.tokens.len() <= offset {
            let token = self.read_token();
            self.tokens.push_back(token);
        }

        return self.tokens.get(offset).expect("no token found");
    }

    fn expect(&self, kind: super::token::TokenKind) -> crate::error::Result<()> {
        if *self.get_kind() == kind {
            return Err(Box::new(AiScriptSyntaxError::new(
                format!("unexpected token: {:?}", self.get_kind()),
                self.token().loc.clone(),
            )));
        }
        return Ok(());
    }

    fn next_with(&mut self, kind: super::token::TokenKind) -> crate::error::Result<()> {
        self.expect(kind)?;
        self.next();
        return Ok(());
    }
}
