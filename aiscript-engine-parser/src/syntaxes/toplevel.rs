use aiscript_engine_ast::{self as ast, Loc, Meta, Namespace, NodeBase};
use aiscript_engine_common::{AiScriptSyntaxError, Result};
use aiscript_engine_lexer::{ITokenStream, TokenKind};

use super::{
    expressions::parse_expr,
    statement::{parse_def_statement, parse_statement},
};

/// ```abnf
/// TopLevel = *(Namespace / Meta / Statement)
/// ```
pub fn parse_top_level(s: &mut impl ITokenStream) -> Result<Vec<ast::Node>> {
    let mut nodes: Vec<ast::Node> = Vec::new();

    s.skip_while(|token| matches!(token.kind, TokenKind::NewLine))?;

    while !matches!(s.get_token_kind(), TokenKind::EOF) {
        match s.get_token_kind() {
            TokenKind::Colon2 => {
                nodes.push(parse_namespace(s)?.into());
            }
            TokenKind::Sharp3 => {
                nodes.push(parse_meta(s)?.into());
            }
            _ => {
                nodes.push(parse_statement(s)?.into());
            }
        }

        // terminator
        match s.get_token_kind() {
            TokenKind::NewLine | TokenKind::SemiColon => {
                s.skip_while(|token| {
                    matches!(token.kind, TokenKind::NewLine | TokenKind::SemiColon)
                })?;
            }
            TokenKind::EOF => {}
            _ => {
                return Err(Box::new(AiScriptSyntaxError::new(
                    "Multiple statements cannot be placed on a single line.",
                    s.get_pos().to_owned(),
                )));
            }
        }
    }

    return Ok(nodes);
}

/// ```abnf
/// Namespace = "::" IDENT "{" *(VarDef / FnDef / Namespace) "}"
/// ```
pub(super) fn parse_namespace(s: &mut impl ITokenStream) -> Result<ast::Namespace> {
    let start_pos = s
        .expect_and_next(|token| matches!(token.kind, TokenKind::Colon2))?
        .pos;

    let name = s.expect_identifier_and_next()?.raw;

    let mut members: Vec<ast::NamespaceMember> = Vec::new();
    s.expect_and_next(|token| matches!(token.kind, TokenKind::OpenBrace))?;

    s.skip_while(|token| matches!(token.kind, TokenKind::NewLine))?;

    while !matches!(s.get_token_kind(), TokenKind::CloseBrace) {
        match s.get_token_kind() {
            TokenKind::VarKeyword | TokenKind::LetKeyword | TokenKind::At => {
                members.push(parse_def_statement(s)?.into());
            }
            TokenKind::Colon2 => {
                members.push(parse_namespace(s)?.into());
            }
            _ => {}
        }

        // terminator
        match s.get_token_kind() {
            TokenKind::NewLine | TokenKind::SemiColon => {
                s.skip_while(|token| {
                    matches!(token.kind, TokenKind::NewLine | TokenKind::SemiColon)
                })?;
            }
            TokenKind::CloseBrace => {}
            _ => {
                return Err(Box::new(AiScriptSyntaxError::new(
                    "Multiple statements cannot be placed on a single line.",
                    s.get_pos().to_owned(),
                )));
            }
        }
    }
    s.expect_and_next(|token| matches!(token.kind, TokenKind::CloseBrace))?;

    return Ok(Namespace {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().to_owned(),
        },
        name,
        members,
    });
}

/// ```abnf
/// Meta = "###" [IDENT] StaticExpr
/// ```
pub(super) fn parse_meta(s: &mut impl ITokenStream) -> Result<ast::Meta> {
    let start_pos = s
        .expect_and_next(|token| matches!(token.kind, TokenKind::Sharp3))?
        .pos;

    let name = s.optional_identifer()?.map(|token| token.raw);

    let value = parse_expr(s, true)?;

    return Ok(Meta {
        loc: Loc {
            start: start_pos,
            end: value.loc().end.to_owned(),
        },
        name,
        value,
    });
}
