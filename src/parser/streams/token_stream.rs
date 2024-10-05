use crate::{
    error::{AiScriptSyntaxError, Result},
    parser::token::{Token, TokenKind, EOF},
};

/// トークンの読み取りに関するトレイト
pub(in crate::parser) trait ITokenStream {
    /// カーソル位置にあるトークンを取得します。
    fn get_token(&self) -> &Token;

    /// カーソル位置にあるトークンの種類を取得します。
    fn get_kind(&self) -> &TokenKind;

    /// カーソル位置を次のトークンへ進めます。
    fn next(&mut self) -> Result<()>;

    /// トークンの先読みを行います。カーソル位置は移動されません。
    fn lookahead(&mut self, offset: usize) -> Result<&Token>;

    /// カーソル位置にあるトークンが指定したトークンの種類と一致するかを確認します。
    /// 一致しなかった場合には文法エラーを発生させます。
    fn expect(&self, kind: TokenKind) -> Result<()>;

    /// カーソル位置にあるトークンが指定したトークンの種類と一致することを確認し、
    /// カーソル位置を次のトークンへ進めます。
    fn next_with(&mut self, kind: TokenKind) -> Result<()>;
}

/// トークン列からトークンを読み取るトレイト
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

    fn get_kind(&self) -> &TokenKind {
        &self.get_token().kind
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

    fn expect(&self, kind: TokenKind) -> Result<()> {
        if *self.get_kind() == kind {
            return Err(Box::new(AiScriptSyntaxError::new(
                format!("unexpected token: {:?}", self.get_kind()),
                self.token.loc.clone(),
            )));
        }
        return Ok(());
    }

    fn next_with(&mut self, kind: TokenKind) -> Result<()> {
        self.expect(kind)?;
        self.next();
        return Ok(());
    }
}
