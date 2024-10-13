use crate::{
    ast::{self, Loc},
    error::{AiScriptSyntaxError, Result},
    expect_token_kind, is_token_kind,
    parser::{streams::ITokenStream, token::TokenKind},
};

use super::{expressions::parse_expr, statement::parse_statement};

/// ```abnf
/// Dest = IDENT / Expr
/// ```
pub(super) fn parse_dest(s: &mut impl ITokenStream) -> Result<ast::Expression> {
    if let TokenKind::Identifier(name) = s.get_token_kind() {
        let name = name.clone();
        let name_start_pos = s.get_pos().clone();
        s.next()?;
        return Ok(ast::Identifier {
            loc: Loc {
                start: name_start_pos,
                end: s.get_pos().to_owned(),
            },
            name,
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

    expect_token_kind!(s, TokenKind::OpenParen)?;
    s.next()?;

    while !is_token_kind!(s, TokenKind::CloseParen) {
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
        let ty = if is_token_kind!(s, TokenKind::Colon) {
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
                if is_token_kind!(s, TokenKind::NewLine) {
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

    expect_token_kind!(s, TokenKind::CloseParen)?;
    s.next()?;

    return Ok(items);
}

/// ```abnf
/// Block = "{" *Statement "}"
/// ```
pub(super) fn parse_block(s: &mut impl ITokenStream) -> Result<Vec<ast::StatementOrExpression>> {
    expect_token_kind!(s, TokenKind::OpenBrace)?;
    s.next()?;

    while is_token_kind!(s, TokenKind::NewLine) {
        s.next()?;
    }

    let mut steps: Vec<ast::StatementOrExpression> = Vec::new();
    while !is_token_kind!(s, TokenKind::CloseBrace) {
        steps.push(parse_statement(s)?);

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

    return Ok(steps);
}

pub(super) fn parse_type(s: &mut impl ITokenStream) -> Result<ast::TypeSource> {
    if is_token_kind!(s, TokenKind::At) {
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
    let start_pos = s.get_pos().clone();

    expect_token_kind!(s, TokenKind::At)?;
    s.next()?;
    expect_token_kind!(s, TokenKind::OpenParen)?;
    s.next()?;

    let mut params: Vec<ast::TypeSource> = Vec::new();
    while !is_token_kind!(s, TokenKind::CloseParen) {
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

    expect_token_kind!(s, TokenKind::CloseParen)?;
    s.next()?;
    expect_token_kind!(s, TokenKind::Arrow)?;
    s.next()?;

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
    let start_pos = s.get_pos().clone();

    let TokenKind::Identifier(name) = s.get_token_kind() else {
        return Err(s.unexpected_token());
    };
    let name = name.clone();
    s.next()?;

    // inner type
    let inner = if is_token_kind!(s, TokenKind::Lt) {
        s.next()?;
        let inner = parse_type(s)?;
        expect_token_kind!(s, TokenKind::Gt)?;
        s.next()?;
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
