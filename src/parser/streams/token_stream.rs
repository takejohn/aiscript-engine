use crate::{
    common::Position,
    error::{AiScriptError, AiScriptSyntaxError, Result},
    parser::token::{Token, TokenKind, EOF},
};

/// カーソル位置にあるトークンの種類が指定したトークンの種類と一致するかどうかを示す値を取得します。
#[macro_export]
macro_rules! is_token_kind {
    ($stream: expr, $pattern: pat) => {
        match $crate::parser::streams::ITokenStream::get_token_kind($stream) {
            $pattern => true,
            _ => false,
        }
    };
}

/// カーソル位置にあるトークンが指定したトークンの種類と一致するかを確認します。
/// 一致しなかった場合には文法エラーを発生させます。
#[macro_export]
macro_rules! expect_token_kind {
    ($stream: expr, $pattern: pat) => {
        match $crate::parser::streams::ITokenStream::get_token_kind($stream) {
            $pattern => std::result::Result::Ok(()),
            _ => std::result::Result::Err($crate::parser::streams::ITokenStream::unexpected_token(
                $stream,
            )),
        }
    };
}

/// トークンの読み取りに関するトレイト
pub(in crate::parser) trait ITokenStream {
    /// カーソル位置にあるトークンを取得します。
    fn get_token(&self) -> &Token;

    /// カーソル位置にあるトークンの種類が指定したトークンの種類と一致するかどうかを示す値を取得します。
    fn is(&self, kind: &TokenKind) -> bool {
        self.get_token_kind() == kind
    }

    /// カーソル位置にあるトークンの種類を取得します。
    fn get_token_kind(&self) -> &TokenKind {
        &self.get_token().kind
    }

    /// カーソル位置にあるトークンの位置情報を取得します。
    fn get_pos(&self) -> &Position {
        &self.get_token().pos
    }

    /// カーソル位置を次のトークンへ進めます。
    fn next(&mut self) -> Result<()>;

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
pub(in crate::parser) struct TokenStream<'a> {
    source: &'a [Token],
    index: usize,
    token: &'a Token,
}

impl TokenStream<'_> {
    pub fn new(source: &[Token]) -> TokenStream {
        let result = TokenStream {
            source,
            index: 0,
            token: Self::load_token(source, 0),
        };
        return result;
    }

    pub fn eof(&self) -> bool {
        Self::eof_for_props(self.source, self.index)
    }

    fn eof_for_props(source: &[Token], index: usize) -> bool {
        index >= source.len()
    }

    fn load_token(source: &[Token], index: usize) -> &Token {
        source.get(index).unwrap_or(&EOF)
    }

    fn load(&mut self) {
        self.token = Self::load_token(self.source, self.index);
    }
}

impl ITokenStream for TokenStream<'_> {
    fn get_token(&self) -> &Token {
        if self.eof() {
            return &EOF;
        }
        return self.token;
    }

    fn next(&mut self) -> Result<()> {
        if !self.eof() {
            self.index += 1;
        }
        self.load();
        return Ok(());
    }

    fn lookahead(&mut self, offset: usize) -> Result<&Token> {
        Ok(self.source.get(self.index + offset).unwrap_or(&EOF))
    }
}
