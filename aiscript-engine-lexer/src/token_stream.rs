use std::collections::VecDeque;

use crate::token::{Token, TokenKind, EOF};
use aiscript_engine_common::{AiScriptError, AiScriptSyntaxError, Position, Result};

/// カーソル位置にあるトークンの種類が指定したトークンの種類と一致するかどうかを示す値を取得します。
#[macro_export]
macro_rules! is_token_kind {
    ($stream: expr, $pattern: pat) => {
        match $crate::ITokenStream::get_token_kind($stream) {
            $pattern => true,
            _ => false,
        }
    };
}

/// カーソル位置にあるトークンが指定したトークンの種類と一致するかを確認します。
/// 一致しなかった場合には文法エラーを発生させます。
#[macro_export]
macro_rules! expect_token_kind {
    ($stream: expr, $pattern: pat) => {{
        let s = &$stream;
        match $crate::ITokenStream::get_token_kind(*s) {
            $pattern => ::std::result::Result::Ok(()),
            _ => ::std::result::Result::Err($crate::ITokenStream::unexpected_token(*s)),
        }
    }};
}

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
