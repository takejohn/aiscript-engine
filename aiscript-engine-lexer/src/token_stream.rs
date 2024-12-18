use std::collections::VecDeque;

use crate::token::{RawToken, Token, TokenKind, EOF};
use aiscript_engine_common::{AiScriptError, AiScriptSyntaxError, Position, Result};

/// トークンの読み取りに関するトレイト
pub trait ITokenStream {
    /// カーソル位置にあるトークンの参照を取得します。
    fn get_token(&self) -> &Token;

    /// カーソル位置にあるトークンの種類を取得します。
    fn get_token_kind(&self) -> &TokenKind {
        &self.get_token().kind
    }

    /// カーソル位置にあるトークンの位置情報を取得します。
    fn get_pos(&self) -> &Position {
        &self.get_token().pos
    }

    /// 現在のカーソル位置のトークンを取得し、カーソル位置を次のトークンへ進めます。
    fn next(&mut self) -> Result<Token>;

    /// トークンの先読みを行います。カーソル位置は移動されません。
    fn lookahead(&mut self, offset: usize) -> Result<&Token>;

    /// カーソル位置にあるトークンが与えられたクロージャの条件を満たす間、[`ITokenStream::next`]を繰り返し呼び出します。
    fn skip_while(&mut self, mut predicate: impl FnMut(&Token) -> bool) -> Result<()> {
        while predicate(self.get_token()) {
            self.next()?;
        }
        return Ok(());
    }

    /// カーソル位置にあるトークンが条件を満たすことを確認し、カーソル位置を次のトークンへ進めます。  
    /// 与えられたクロージャが`true`を返す場合、カーソル位置のトークンを返し、
    /// `false`を返す場合、文法エラーを発生させます。
    fn expect_and_next(&mut self, predicate: impl FnOnce(&Token) -> bool) -> Result<Token> {
        if predicate(self.get_token()) {
            Ok(self.next()?)
        } else {
            Err(self.unexpected_token())
        }
    }

    /// カーソル位置にあるトークンが識別子であることを確認し、そのトークンを取得して、カーソル位置を次のトークンへ進めます。
    /// 識別子でなかった場合は文法エラーを発生させます。
    fn expect_identifier_and_next(&mut self) -> Result<RawToken> {
        self.optional_identifer()?
            .ok_or_else(|| self.unexpected_token())
    }

    /// カーソル位置に識別子トークンがある場合、それを取得し、カーソル位置を次のトークンへ進めます。
    fn optional_identifer(&mut self) -> Result<Option<RawToken>> {
        if let TokenKind::Identifier(_) = self.get_token_kind() {
            let Token {
                kind,
                pos,
                has_left_spacing,
            } = self.next()?;
            if let TokenKind::Identifier(name) = kind {
                return Ok(Some(RawToken {
                    raw: name,
                    pos,
                    has_left_spacing,
                }));
            } else {
                panic!("not an identifer")
            }
        } else {
            return Ok(None);
        }
    }

    fn expect_eof(&self) -> Result<()> {
        if matches!(self.get_token_kind(), TokenKind::EOF) {
            Ok(())
        } else {
            Err(self.unexpected_token())
        }
    }

    /// トークンの種類が予期しない場合のエラーを生成します。
    fn unexpected_token(&self) -> Box<dyn AiScriptError> {
        Box::new(AiScriptSyntaxError::new(
            format!("unexpected token: {:?}", self.get_token_kind()),
            self.get_token().pos.clone(),
        ))
    }
}

/// トークン列からトークンを読み取る構造体
pub struct TokenStream {
    source: VecDeque<Token>,
}

impl TokenStream {
    pub fn new(source: VecDeque<Token>) -> TokenStream {
        let result = TokenStream { source };
        return result;
    }

    pub fn eof(&self) -> bool {
        return self.source.is_empty();
    }
}

impl ITokenStream for TokenStream {
    fn get_token(&self) -> &Token {
        self.source.front().unwrap_or(&EOF)
    }

    fn next(&mut self) -> Result<Token> {
        return Ok(self.source.pop_front().unwrap_or(EOF.clone()));
    }

    fn lookahead(&mut self, offset: usize) -> Result<&Token> {
        Ok(self.source.get(offset).unwrap_or(&EOF))
    }
}
