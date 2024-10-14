use aiscript_engine_common::{AiScriptSyntaxError, Result};
use aiscript_engine_lexer::{expect_token_kind, is_token_kind, ITokenStream, TokenKind};

use crate::ast::{self, Loc, Meta, Namespace, NodeBase};

use super::{
    expressions::parse_expr,
    statement::{parse_def_statement, parse_statement},
};

/// ```abnf
/// TopLevel = *(Namespace / Meta / Statement)
/// ```
pub fn parse_top_level(s: &mut impl ITokenStream) -> Result<Vec<ast::Node>> {
    let mut nodes: Vec<ast::Node> = Vec::new();

    while is_token_kind!(s, TokenKind::NewLine) {
        s.next()?;
    }

    while !is_token_kind!(s, TokenKind::EOF) {
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
                while is_token_kind!(s, TokenKind::NewLine | TokenKind::SemiColon) {
                    s.next()?;
                }
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
    let start_pos = s.get_pos().to_owned();

    expect_token_kind!(s, TokenKind::Colon2)?;
    s.next()?;

    let TokenKind::Identifier(name) = s.get_token_kind() else {
        return Err(s.unexpected_token());
    };
    let name = name.to_owned();
    s.next()?;

    let mut members: Vec<ast::NamespaceMember> = Vec::new();
    expect_token_kind!(s, TokenKind::OpenBrace)?;
    s.next()?;

    while is_token_kind!(s, TokenKind::NewLine) {
        s.next()?;
    }

    while !is_token_kind!(s, TokenKind::CloseBrace) {
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
                while is_token_kind!(s, TokenKind::NewLine | TokenKind::SemiColon) {
                    s.next()?;
                }
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
    expect_token_kind!(s, TokenKind::CloseBrace)?;
    s.next()?;

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
    let start_pos = s.get_pos().to_owned();

    expect_token_kind!(s, TokenKind::Sharp3)?;
    s.next()?;

    let name = if let TokenKind::Identifier(name) = s.get_token_kind() {
        let name = name.clone();
        s.next()?;
        Some(name)
    } else {
        None
    };

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
