use std::vec;

use aiscript_engine_ast::{
    self as ast, Expression, Identifier, Loc, NodeBase, Statement, StatementOrExpression,
};
use aiscript_engine_common::{AiScriptSyntaxError, NamePath, Result, Utf16Str};
use aiscript_engine_lexer::{ITokenStream, RawToken, TokenKind};
use utf16_literal::utf16;

use crate::syntaxes::expressions::parse_expr;

use super::common::{parse_block, parse_dest, parse_params, parse_type};

pub(super) fn parse_statement(s: &mut impl ITokenStream) -> Result<ast::StatementOrExpression> {
    fn statement(result: Result<impl Into<Statement>>) -> Result<StatementOrExpression> {
        return result.map(|value| StatementOrExpression::from_statement(value));
    }

    fn expr(result: Result<impl Into<Expression>>) -> Result<StatementOrExpression> {
        return result.map(|value| StatementOrExpression::from_expr(value));
    }

    let start_pos = s.get_pos().clone();

    match s.get_token_kind() {
        TokenKind::VarKeyword | TokenKind::LetKeyword => return statement(parse_var_def(s)),
        TokenKind::At => {
            if matches!(s.lookahead(1)?.kind, TokenKind::Identifier(_)) {
                return statement(parse_fn_def(s));
            }
        }
        TokenKind::Out => return expr(parse_out(s)),
        TokenKind::ReturnKeyword => return statement(parse_return(s)),
        TokenKind::OpenSharpBracket => return statement(parse_statement_with_attr(s)),
        TokenKind::EachKeyword => return statement(parse_each(s)),
        TokenKind::ForKeyword => return statement(parse_for(s)),
        TokenKind::LoopKeyword => return statement(parse_loop(s)),
        TokenKind::DoKeyword => return statement(parse_do_while(s)),
        TokenKind::WhileKeyword => return statement(parse_while(s)),
        TokenKind::BreakKeyword => {
            s.next()?;
            return Ok(StatementOrExpression::from_statement(ast::Break {
                loc: Loc {
                    start: start_pos,
                    end: s.get_pos().to_owned(),
                },
            }));
        }
        TokenKind::ContinueKeyword => {
            s.next()?;
            return Ok(StatementOrExpression::from_statement(ast::Continue {
                loc: Loc {
                    start: start_pos,
                    end: s.get_pos().to_owned(),
                },
            }));
        }
        _ => {}
    }
    let expr = parse_expr(s, false)?;
    return try_parse_assign(s, expr);
}

/// ```abnf
/// DefStatement = VarDef / FnDef
/// ```
pub(super) fn parse_def_statement(s: &mut impl ITokenStream) -> Result<ast::Definition> {
    match s.get_token_kind() {
        TokenKind::VarKeyword | TokenKind::LetKeyword => return parse_var_def(s),
        TokenKind::At => return parse_fn_def(s),
        kind => {
            return Err(Box::new(AiScriptSyntaxError::new(
                format!("unexpected token: {:?}", kind),
                s.get_pos().to_owned(),
            )))
        }
    }
}

/// ```abnf
/// BlockOrStatement = Block / Statement
/// ```
pub(super) fn parse_block_or_statement(
    s: &mut impl ITokenStream,
) -> Result<ast::StatementOrExpression> {
    if matches!(s.get_token_kind(), TokenKind::OpenBrace) {
        let start_pos = s.get_pos().clone();
        let statements = parse_block(s)?;
        return Ok(StatementOrExpression::from_expr(ast::Block {
            loc: Loc {
                start: start_pos,
                end: s.get_pos().to_owned(),
            },
            statements,
        }));
    } else {
        return parse_statement(s);
    }
}

/// ```abnf
/// VerDef = ("let" / "var") Dest [":" Type] "=" Expr
/// ```
fn parse_var_def(s: &mut impl ITokenStream) -> Result<ast::Definition> {
    let start_pos = s.get_pos().clone();

    let is_mut = match s.get_token_kind() {
        TokenKind::LetKeyword => false,
        TokenKind::VarKeyword => true,
        _ => {
            return Err(Box::new(AiScriptSyntaxError::new(
                format!("unexpected token: {:?}", s.get_token_kind()),
                s.get_pos().to_owned(),
            )))
        }
    };
    s.next()?;

    let dest = parse_dest(s)?;

    let ty = if matches!(s.get_token_kind(), TokenKind::Colon) {
        s.next()?;
        Some(parse_type(s)?)
    } else {
        None
    };

    s.expect_and_next(|token| matches!(token.kind, TokenKind::Eq))?;

    s.skip_while(|token| matches!(token.kind, TokenKind::NewLine))?;

    let expr = parse_expr(s, false)?;

    return Ok(ast::Definition {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().to_owned(),
        },
        dest,
        var_type: ty,
        expr,
        is_mut,
        attr: Vec::new(),
    });
}

/// ```abnf
/// FnDef = "@" IDENT Params [":" Type] Block
/// ```
fn parse_fn_def(s: &mut impl ITokenStream) -> Result<ast::Definition> {
    let start_pos = s
        .expect_and_next(|token| matches!(token.kind, TokenKind::At))?
        .pos;

    let RawToken {
        raw: name,
        pos: name_start_pos,
        ..
    } = s.expect_identifier_and_next()?;
    let dest = ast::Identifier {
        loc: Loc {
            start: name_start_pos,
            end: s.get_pos().to_owned(),
        },
        name: name.into(),
    };

    let params = parse_params(s)?;

    let ty = if matches!(s.get_token_kind(), TokenKind::Colon) {
        s.next()?;
        Some(parse_type(s)?)
    } else {
        None
    };

    let body = parse_block(s)?;

    let end_pos = s.get_pos().clone();

    return Ok(ast::Definition {
        loc: Loc {
            start: start_pos.clone(),
            end: end_pos.clone(),
        },
        dest: dest.into(),
        expr: ast::Fn {
            loc: Loc {
                start: start_pos,
                end: end_pos,
            },
            args: params,
            ret_type: ty,
            children: body,
        }
        .into(),
        var_type: None,
        is_mut: false,
        attr: Vec::new(),
    });
}

/// ```abnf
/// Out = "<:" Expr
/// ```
fn parse_out(s: &mut impl ITokenStream) -> Result<ast::Call> {
    let start_pos = s
        .expect_and_next(|token| matches!(token.kind, TokenKind::Out))?
        .pos;
    let expr = parse_expr(s, false)?;

    return Ok(ast::Call {
        loc: Loc {
            start: start_pos.clone(),
            end: s.get_pos().to_owned(),
        },
        target: Box::new(
            Identifier {
                loc: Loc {
                    start: start_pos.clone(),
                    end: start_pos,
                },
                name: NamePath::from(Utf16Str::new(&utf16!("print"))),
            }
            .into(),
        ),
        args: vec![expr],
    });
}

/// ```abnf
/// Each = "each" "(" "let" Dest "," Expr ")" BlockOrStatement
///      / "each"     "let" Dest "," Expr     BlockOrStatement
/// ```
fn parse_each(s: &mut impl ITokenStream) -> Result<ast::Each> {
    let start_pos = s
        .expect_and_next(|token| matches!(token.kind, TokenKind::EachKeyword))?
        .pos;

    let has_paren = matches!(s.get_token_kind(), TokenKind::OpenParen);
    if has_paren {
        s.next()?;
    }

    s.expect_and_next(|token| matches!(token.kind, TokenKind::LetKeyword))?;

    let dest = parse_dest(s)?;

    if matches!(s.get_token_kind(), TokenKind::Comma) {
        s.next()?;
    } else {
        return Err(Box::new(AiScriptSyntaxError::new(
            "separator expected",
            s.get_pos().to_owned(),
        )));
    }

    let items = parse_expr(s, false)?;

    if has_paren {
        s.expect_and_next(|token| matches!(token.kind, TokenKind::CloseParen))?;
    }

    let body = parse_block_or_statement(s)?;

    return Ok(ast::Each {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().to_owned(),
        },
        var: dest,
        items,
        for_statement: Box::new(body),
    });
}

/// ```abnf
/// For = ForRange / ForTimes
/// ForRange = "for" "(" "let" IDENT ["=" Expr] "," Expr ")" BlockOrStatement
///          / "for"     "let" IDENT ["=" Expr] "," Expr     BlockOrStatement
/// ForTimes = "for" "(" Expr ")" BlockOrStatement
///          / "for"     Expr     BlockOrStatement
/// ```
fn parse_for(s: &mut impl ITokenStream) -> Result<ast::For> {
    let start_pos = s
        .expect_and_next(|token| matches!(token.kind, TokenKind::ForKeyword))?
        .pos;

    let has_paren = matches!(s.get_token_kind(), TokenKind::OpenParen);
    if has_paren {
        s.next()?;
    }

    if matches!(s.get_token_kind(), TokenKind::LetKeyword) {
        // range syntax
        s.next()?;

        let RawToken {
            raw: name,
            pos: ident_pos,
            ..
        } = s.expect_identifier_and_next()?;

        let from: Expression = if matches!(s.get_token_kind(), TokenKind::Eq) {
            s.next()?;
            parse_expr(s, false)?
        } else {
            ast::Num {
                loc: Loc {
                    start: ident_pos.clone(),
                    end: ident_pos,
                },
                value: 0.0,
            }
            .into()
        };

        if matches!(s.get_token_kind(), TokenKind::Comma) {
            s.next()?;
        } else {
            return Err(Box::new(AiScriptSyntaxError::new(
                "separator expected",
                s.get_pos().to_owned(),
            )));
        }

        let to = parse_expr(s, false)?;

        if has_paren {
            s.expect_and_next(|token| matches!(token.kind, TokenKind::CloseParen))?;
        }

        let body = parse_block_or_statement(s)?;

        return Ok(ast::For {
            loc: Loc {
                start: start_pos,
                end: s.get_pos().to_owned(),
            },
            iter: ast::ForIterator::Range {
                var: name,
                from,
                to,
            },
            for_statement: Box::new(body.into()),
        });
    } else {
        // times syntax

        let times = parse_expr(s, false)?;

        if has_paren {
            s.expect_and_next(|token| matches!(token.kind, TokenKind::CloseParen))?;
        }

        let body = parse_block_or_statement(s)?;

        return Ok(ast::For {
            loc: Loc {
                start: start_pos,
                end: s.get_pos().to_owned(),
            },
            iter: ast::ForIterator::Times { times },
            for_statement: Box::new(body.into()),
        });
    }
}

/// ```abnf
/// Return = "return" Expr
/// ```
fn parse_return(s: &mut impl ITokenStream) -> Result<ast::Return> {
    let start_pos = s
        .expect_and_next(|token| matches!(token.kind, TokenKind::ReturnKeyword))?
        .pos;
    let expr = parse_expr(s, false)?;

    return Ok(ast::Return {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().clone(),
        },
        expr,
    });
}

/// ```abnf
/// StatementWithAttr = *Attr Statement
/// ```
fn parse_statement_with_attr(s: &mut impl ITokenStream) -> Result<ast::Definition> {
    let mut attrs: Vec<ast::Attribute> = Vec::new();
    while matches!(s.get_token_kind(), TokenKind::OpenSharpBracket) {
        attrs.push(parse_attr(s)?);
        s.expect_and_next(|token| matches!(token.kind, TokenKind::NewLine))?;
    }

    let statement = parse_statement(s)?;
    let loc = statement.loc().start.to_owned();

    if let StatementOrExpression::Statement(statement) = statement {
        if let Statement::Def(mut statement) = statement {
            statement.attr.extend(attrs);
            return Ok(statement);
        }
    }

    return Err(Box::new(AiScriptSyntaxError::new(
        "invalid attribute.",
        loc,
    )));
}

/// ```abnf
/// Attr = "#[" IDENT [StaticExpr] "]"
/// ```
fn parse_attr(s: &mut impl ITokenStream) -> Result<ast::Attribute> {
    let start_pos = s
        .expect_and_next(|token| matches!(token.kind, TokenKind::OpenSharpBracket))?
        .pos;

    let name = s.expect_identifier_and_next()?.raw;

    let value = if !matches!(s.get_token_kind(), TokenKind::CloseBracket) {
        parse_expr(s, true)?
    } else {
        let close_pos = s.get_pos().clone();
        ast::Bool {
            loc: Loc {
                start: close_pos.clone(),
                end: close_pos,
            },
            value: true,
        }
        .into()
    };

    s.expect_and_next(|token| matches!(token.kind, TokenKind::CloseBracket))?;

    return Ok(ast::Attribute {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().to_owned(),
        },
        name,
        value,
    });
}

/// ```abnf
/// Loop = "loop" Block
/// ```
fn parse_loop(s: &mut impl ITokenStream) -> Result<ast::Loop> {
    let start_pos = s
        .expect_and_next(|token| matches!(token.kind, TokenKind::LoopKeyword))?
        .pos;
    let statements = parse_block(s)?;

    return Ok(ast::Loop {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().to_owned(),
        },
        statements,
    });
}

/// ```abnf
/// Loop = "do" BlockOrStatement "while" Expr
/// ```
fn parse_do_while(s: &mut impl ITokenStream) -> Result<ast::Loop> {
    let do_start_pos = s
        .expect_and_next(|token| matches!(token.kind, TokenKind::DoKeyword))?
        .pos;
    let body = parse_block_or_statement(s)?;
    let while_pos = s
        .expect_and_next(|token| matches!(token.kind, TokenKind::WhileKeyword))?
        .pos;
    let cond = parse_expr(s, false)?;
    let end_pos = s.get_pos().clone();

    return Ok(ast::Loop {
        loc: Loc {
            start: do_start_pos,
            end: end_pos.clone(),
        },
        statements: vec![
            body,
            StatementOrExpression::from_expr(ast::If {
                loc: Loc {
                    start: while_pos.clone(),
                    end: end_pos.clone(),
                },
                cond: Box::new(
                    ast::Not {
                        loc: Loc {
                            start: while_pos,
                            end: end_pos.clone(),
                        },
                        expr: Box::new(cond),
                    }
                    .into(),
                ),
                then: Box::new(StatementOrExpression::from_statement(ast::Break {
                    loc: Loc {
                        start: end_pos.clone(),
                        end: end_pos,
                    },
                })),
                elseif: Vec::new(),
                else_statement: None,
            }),
        ],
    });
}

/// ```abnf
/// Loop = "while" Expr BlockOrStatement
/// ```
fn parse_while(s: &mut impl ITokenStream) -> Result<ast::Loop> {
    let start_pos = s
        .expect_and_next(|token| matches!(token.kind, TokenKind::WhileKeyword))?
        .pos;
    let cond = parse_expr(s, false)?;
    let cond_end_pos = s.get_pos().clone();
    let body = parse_block_or_statement(s)?;

    return Ok(ast::Loop {
        loc: Loc {
            start: start_pos.clone(),
            end: s.get_pos().to_owned(),
        },
        statements: vec![
            StatementOrExpression::from_expr(ast::If {
                loc: Loc {
                    start: start_pos.clone(),
                    end: cond_end_pos.clone(),
                },
                cond: Box::new(
                    ast::Not {
                        loc: Loc {
                            start: start_pos.clone(),
                            end: cond_end_pos.clone(),
                        },
                        expr: Box::new(cond),
                    }
                    .into(),
                ),
                then: Box::new(StatementOrExpression::from_statement(ast::Break {
                    loc: Loc {
                        start: cond_end_pos.clone(),
                        end: cond_end_pos,
                    },
                })),
                elseif: Vec::new(),
                else_statement: None,
            }),
            body,
        ],
    });
}

/// ```abnf
/// Assign = Expr ("=" / "+=" / "-=") Expr
/// ```
fn try_parse_assign(
    s: &mut impl ITokenStream,
    dest: ast::Expression,
) -> Result<ast::StatementOrExpression> {
    let op = match s.get_token_kind() {
        TokenKind::Eq => ast::AssignOperator::Assign,
        TokenKind::PlusEq => ast::AssignOperator::AddAssign,
        TokenKind::MinusEq => ast::AssignOperator::SubAssign,
        _ => {
            return Ok(dest.into());
        }
    };

    // Assign
    let start_pos = s.next()?.pos;
    let expr = parse_expr(s, false)?;
    return Ok(StatementOrExpression::from_statement(ast::Assign {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().to_owned(),
        },
        op,
        dest,
        expr,
    }));
}
