use aiscript_engine_ast::{self as ast, Loc};
use aiscript_engine_common::{AiScriptSyntaxError, Result};
use aiscript_engine_lexer::{ITokenStream, RawToken, TokenKind};

use super::{expressions::parse_expr, statement::parse_statement};

/// ```abnf
/// Dest = IDENT / Expr
/// ```
pub(super) fn parse_dest(s: &mut impl ITokenStream) -> Result<ast::Expression> {
    if let Some(token) = s.optional_identifer()? {
        let RawToken {
            raw: name,
            pos: name_start_pos,
            ..
        } = token;
        return Ok(ast::Identifier {
            loc: Loc {
                start: name_start_pos,
                end: s.get_pos().to_owned(),
            },
            name: name.into(),
        }
        .into());
    } else {
        return parse_expr(s, false);
    }
}

/// ```abnf
/// Params = "(" [Dest [":" Type] *(SEP Dest [":" Type])] ")"
/// ```
pub(super) fn parse_params(s: &mut impl ITokenStream) -> Result<Vec<ast::FnArg>> {
    let mut items: Vec<ast::FnArg> = Vec::new();

    s.expect_and_next(|token| matches!(token.kind, TokenKind::OpenParen))?;

    if matches!(s.get_token_kind(), TokenKind::NewLine) {
        s.next()?;
    }

    while !matches!(s.get_token_kind(), TokenKind::CloseParen) {
        let dest = parse_dest(s)?;

        let value = match s.get_token_kind() {
            TokenKind::Question => {
                s.next()?;
                ast::FnArgValue::Optional
            }
            TokenKind::Eq => {
                s.next()?;
                ast::FnArgValue::Required {
                    default: Some(parse_expr(s, false)?),
                }
            }
            _ => ast::FnArgValue::Required { default: None },
        };
        let ty = if matches!(s.get_token_kind(), TokenKind::Colon) {
            s.next()?;
            Some(parse_type(s)?)
        } else {
            None
        };

        items.push(ast::FnArg {
            dest,
            value,
            arg_type: ty,
        });

        // separator
        match s.get_token_kind() {
            TokenKind::NewLine => {
                s.next()?;
            }
            TokenKind::Comma => {
                s.next()?;
                if matches!(s.get_token_kind(), TokenKind::NewLine) {
                    s.next()?;
                }
            }
            TokenKind::CloseParen => {}
            _ => {
                return Err(Box::new(AiScriptSyntaxError::new(
                    "separator expected",
                    s.get_pos().to_owned(),
                )))
            }
        }
    }

    s.expect_and_next(|token| matches!(token.kind, TokenKind::CloseParen))?;

    return Ok(items);
}

/// ```abnf
/// Block = "{" *Statement "}"
/// ```
pub(super) fn parse_block(s: &mut impl ITokenStream) -> Result<Vec<ast::StatementOrExpression>> {
    s.expect_and_next(|token| matches!(token.kind, TokenKind::OpenBrace))?;

    while matches!(s.get_token_kind(), TokenKind::NewLine) {
        s.next()?;
    }

    let mut steps: Vec<ast::StatementOrExpression> = Vec::new();
    while !matches!(s.get_token_kind(), TokenKind::CloseBrace) {
        steps.push(parse_statement(s)?);

        // terminator
        match s.get_token_kind() {
            TokenKind::NewLine | TokenKind::SemiColon => {
                while matches!(
                    s.get_token_kind(),
                    TokenKind::NewLine | TokenKind::SemiColon
                ) {
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

    s.expect_and_next(|token| matches!(token.kind, TokenKind::CloseBrace))?;

    return Ok(steps);
}

pub(super) fn parse_type(s: &mut impl ITokenStream) -> Result<ast::TypeSource> {
    if matches!(s.get_token_kind(), TokenKind::At) {
        return parse_fn_type(s);
    } else {
        return parse_named_type(s);
    }
}

/// ```abnf
/// FnType = "@" "(" ParamTypes ")" "=>" Type
/// ParamTypes = [Type *(SEP Type)]
/// ```
fn parse_fn_type(s: &mut impl ITokenStream) -> Result<ast::TypeSource> {
    let start_pos = s
        .expect_and_next(|token| matches!(token.kind, TokenKind::At))?
        .pos;
    s.expect_and_next(|token| matches!(token.kind, TokenKind::OpenParen))?;

    let mut params: Vec<ast::TypeSource> = Vec::new();
    while !matches!(s.get_token_kind(), TokenKind::CloseParen) {
        if !params.is_empty() {
            match s.get_token_kind() {
                TokenKind::Comma => {
                    s.next()?;
                }
                _ => {
                    return Err(Box::new(AiScriptSyntaxError::new(
                        "separator expected",
                        s.get_pos().to_owned(),
                    )));
                }
            }
        }
        let ty = parse_type(s)?;
        params.push(ty);
    }

    s.expect_and_next(|token| matches!(token.kind, TokenKind::CloseParen))?;
    s.expect_and_next(|token| matches!(token.kind, TokenKind::Arrow))?;

    let result_type = parse_type(s)?;

    return Ok(ast::FnTypeSource {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().to_owned(),
        },
        args: params,
        result: Box::new(result_type),
    }
    .into());
}

/// ```abnf
/// NamedType = IDENT ["<" Type ">"]
/// ```
fn parse_named_type(s: &mut impl ITokenStream) -> Result<ast::TypeSource> {
    let RawToken {
        raw: name,
        pos: start_pos,
        ..
    } = s.expect_identifier_and_next()?;

    // inner type
    let inner = if matches!(s.get_token_kind(), TokenKind::Lt) {
        s.next()?;
        let inner = parse_type(s)?;
        s.expect_and_next(|token| matches!(token.kind, TokenKind::Gt))?;
        Some(Box::new(inner))
    } else {
        None
    };

    return Ok(ast::NamedTypeSource {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().to_owned(),
        },
        name,
        inner,
    }
    .into());
}
