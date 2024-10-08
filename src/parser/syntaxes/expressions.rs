use std::collections::HashMap;

use pratt::{parse_pratt, BindingPower};
use utf16_literal::utf16;

use crate::{
    ast::{self, Loc, Node},
    error::{AiScriptSyntaxError, Result},
    expect_token_kind, is_token_kind,
    parser::{
        streams::{ITokenStream, TokenStream},
        syntaxes::statement::parse_block_or_statement,
        token::TokenKind,
    },
    string::{Utf16Str, Utf16String},
};

use super::common::{parse_block, parse_params, parse_type};

mod pratt;

pub(super) fn parse_expr(s: &mut impl ITokenStream, is_static: bool) -> Result<ast::Expression> {
    if is_static {
        return parse_atom(s, true);
    } else {
        return parse_pratt(s, 0);
    }
}

fn parse_prefix(s: &mut impl ITokenStream, min_bp: BindingPower) -> Result<ast::Expression> {
    let start_pos = s.get_pos().clone();
    let op = s.get_token_kind().clone();
    s.next()?;

    // 改行のエスケープ
    if !is_token_kind!(s, TokenKind::BackSlash) {
        s.next()?;
        expect_token_kind!(s, TokenKind::NewLine)?;
        s.next()?;
    }

    let expr = parse_pratt(s, min_bp)?;

    let end_pos = expr.loc().end.clone();

    match op {
        TokenKind::Plus => {
            // 数値リテラル以外は非サポート
            if let ast::Expression::Num(num) = expr {
                return Ok(ast::Num {
                    loc: Loc {
                        start: start_pos,
                        end: end_pos,
                    },
                    value: num.value,
                }
                .into());
            } else {
                return Err(Box::new(AiScriptSyntaxError::new(
                    "currently, sign is only supported for number literal.",
                    start_pos,
                )));
            }
            // TODO: 将来的にサポートされる式を拡張
        }
        TokenKind::Minus => {
            // 数値リテラル以外は非サポート
            if let ast::Expression::Num(num) = expr {
                return Ok(ast::Num {
                    loc: Loc {
                        start: start_pos,
                        end: end_pos,
                    },
                    value: -num.value,
                }
                .into());
            } else {
                return Err(Box::new(AiScriptSyntaxError::new(
                    "currently, sign is only supported for number literal.",
                    start_pos,
                )));
            }
            // TODO: 将来的にサポートされる式を拡張
        }
        TokenKind::Not => {
            return Ok(ast::Not {
                loc: Loc {
                    start: start_pos,
                    end: end_pos,
                },
                expr: Box::new(expr),
            }
            .into());
        }
        _ => {
            return Err(Box::new(AiScriptSyntaxError::new(
                format!("unexpected token: {:?}", op),
                start_pos,
            )));
        }
    }
}

fn parse_infix(
    s: &mut impl ITokenStream,
    left: ast::Expression,
    min_bp: BindingPower,
) -> Result<ast::Expression> {
    let start_pos = s.get_pos().clone();
    let op = s.get_token_kind().clone();
    s.next()?;

    // 改行のエスケープ
    if is_token_kind!(s, TokenKind::BackSlash) {
        s.next()?;
        expect_token_kind!(s, TokenKind::NewLine)?;
        s.next()?;
    }

    if op == TokenKind::Dot {
        let TokenKind::Identifier(name) = s.get_token_kind() else {
            return Err(s.unexpected_token());
        };
        let name = name.clone();
        s.next()?;

        return Ok(ast::Prop {
            loc: Loc {
                start: start_pos,
                end: s.get_pos().clone(),
            },
            target: Box::new(left),
            name,
        }
        .into());
    } else {
        let right = parse_pratt(s, min_bp)?;
        let end_pos = s.get_pos().clone();

        let op = match op {
            TokenKind::Hat => ast::BinaryOperator::Pow,
            TokenKind::Asterisk => ast::BinaryOperator::Mul,
            TokenKind::Slash => ast::BinaryOperator::Div,
            TokenKind::Percent => ast::BinaryOperator::Rem,
            TokenKind::Plus => ast::BinaryOperator::Add,
            TokenKind::Minus => ast::BinaryOperator::Sub,
            TokenKind::Lt => ast::BinaryOperator::Lt,
            TokenKind::LtEq => ast::BinaryOperator::Lteq,
            TokenKind::Gt => ast::BinaryOperator::Gt,
            TokenKind::GtEq => ast::BinaryOperator::Gteq,
            TokenKind::Eq2 => ast::BinaryOperator::Eq,
            TokenKind::NotEq => ast::BinaryOperator::Neq,
            TokenKind::And2 => ast::BinaryOperator::And,
            TokenKind::Or2 => ast::BinaryOperator::Or,
            _ => {
                return Err(Box::new(AiScriptSyntaxError::new(
                    format!("unexpected token: {:?}", op),
                    start_pos,
                )))
            }
        };

        return Ok(ast::Binary {
            loc: Loc {
                start: start_pos,
                end: end_pos,
            },
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
        .into());
    }
}

fn parse_postfix(s: &mut impl ITokenStream, expr: ast::Expression) -> Result<ast::Expression> {
    let start_pos = s.get_pos().clone();
    let op = s.get_token_kind().clone();

    match op {
        TokenKind::OpenParen => return parse_call(s, expr),
        TokenKind::OpenBracket => {
            s.next()?;
            let index = parse_expr(s, false)?;
            expect_token_kind!(s, TokenKind::CloseBracket)?;
            s.next()?;

            return Ok(ast::Index {
                loc: Loc {
                    start: start_pos,
                    end: s.get_pos().to_owned(),
                },
                target: Box::new(expr),
                index: Box::new(index),
            }
            .into());
        }
        _ => {
            return Err(Box::new(AiScriptSyntaxError::new(
                format!("unexpected token: {:?}", op),
                start_pos,
            )));
        }
    }
}

fn parse_atom(s: &mut impl ITokenStream, is_static: bool) -> Result<ast::Expression> {
    fn map_into(res: Result<impl Into<ast::Expression>>) -> Result<ast::Expression> {
        res.map(|expr| expr.into())
    }

    let start_pos = s.get_pos().clone();

    match s.get_token_kind().clone() {
        TokenKind::IfKeyword => {
            if !is_static {
                return map_into(parse_if(s));
            }
        }
        TokenKind::At => {
            if !is_static {
                return map_into(parse_fn_expr(s));
            }
        }
        TokenKind::MatchKeyword => {
            if !is_static {
                return map_into(parse_match(s));
            }
        }
        TokenKind::EvalKeyword => {
            if !is_static {
                return map_into(parse_eval(s));
            }
        }
        TokenKind::Template(children) => {
            let mut values: Vec<ast::Expression> = Vec::new();

            if !is_static {
                let mut iter = children.iter().peekable();
                while let Some(ref element) = iter.next() {
                    match element.kind {
                        TokenKind::TemplateStringElement(ref value) => {
                            // トークンの終了位置を取得するために先読み
                            let next_token = iter
                                .peek()
                                .map_or_else(|| s.lookahead(1), |&token| Ok(token))?;
                            values.push(
                                ast::Str {
                                    loc: Loc {
                                        start: element.pos.to_owned(),
                                        end: next_token.pos.to_owned(),
                                    },
                                    value: value.clone(),
                                }
                                .into(),
                            );
                        }
                        TokenKind::TemplateExprElement(ref expr) => {
                            // スキャナで埋め込み式として事前に読み取っておいたトークン列をパースする
                            let mut expr_stream = TokenStream::new(&expr);
                            let expr = parse_expr(&mut expr_stream, false)?;
                            expect_token_kind!(&expr_stream, TokenKind::EOF)?;
                            values.push(expr);
                        }
                        _ => {
                            return Err(Box::new(AiScriptSyntaxError::new(
                                format!("unexpected token: {:?}", element.kind),
                                element.pos.to_owned(),
                            )));
                        }
                    }
                }

                s.next()?;
                return Ok(ast::Tmpl {
                    loc: Loc {
                        start: start_pos,
                        end: s.get_pos().clone(),
                    },
                    tmpl: values,
                }
                .into());
            }
        }
        TokenKind::StringLiteral(value) => {
            s.next()?;
            return Ok(ast::Str {
                loc: Loc {
                    start: start_pos,
                    end: s.get_pos().to_owned(),
                },
                value,
            }
            .into());
        }
        TokenKind::NumberLiteral(value) => {
            // TODO: validate number value
            let value: f64 = value.parse().unwrap();
            s.next()?;
            return Ok(ast::Num {
                loc: Loc {
                    start: start_pos,
                    end: s.get_pos().to_owned(),
                },
                value,
            }
            .into());
        }
        TokenKind::TrueKeyword => {
            s.next()?;
            return Ok(ast::Bool {
                loc: Loc {
                    start: start_pos,
                    end: s.get_pos().to_owned(),
                },
                value: true,
            }
            .into());
        }
        TokenKind::FalseKeyword => {
            s.next()?;
            return Ok(ast::Bool {
                loc: Loc {
                    start: start_pos,
                    end: s.get_pos().to_owned(),
                },
                value: false,
            }
            .into());
        }
        TokenKind::NullKeyword => {
            s.next()?;
            return Ok(ast::Null {
                loc: Loc {
                    start: start_pos,
                    end: s.get_pos().to_owned(),
                },
            }
            .into());
        }
        TokenKind::OpenBrace => {
            return map_into(parse_object(s, is_static));
        }
        TokenKind::OpenBracket => {
            return map_into(parse_array(s, is_static));
        }
        TokenKind::Identifier(_) => {
            if !is_static {
                return map_into(parse_reference(s));
            }
        }
        TokenKind::OpenParen => {
            s.next()?;
            let expr = parse_expr(s, is_static)?;
            expect_token_kind!(s, TokenKind::CloseParen)?;
            s.next()?;
            return Ok(expr);
        }
        _ => {}
    }
    return Err(Box::new(AiScriptSyntaxError::new(
        format!("unexpected token: {:?}", s.get_token_kind()),
        start_pos,
    )));
}

/// ```abnf
/// Call = "(" [Expr *(SEP Expr) [SEP]] ")"
/// ```
fn parse_call(s: &mut impl ITokenStream, target: ast::Expression) -> Result<ast::Expression> {
    let start_pos = s.get_pos().clone();
    let mut items: Vec<ast::Expression> = Vec::new();

    expect_token_kind!(s, TokenKind::OpenParen)?;
    s.next()?;

    if is_token_kind!(s, TokenKind::NewLine) {
        s.next()?;
    }

    while !is_token_kind!(s, TokenKind::CloseParen) {
        items.push(parse_expr(s, false)?);

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
                )));
            }
        }
    }

    expect_token_kind!(s, TokenKind::CloseParen)?;
    s.next()?;

    return Ok(ast::Call {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().to_owned(),
        },
        target: Box::new(target),
        args: items,
    }
    .into());
}

/// ```abnf
/// If = "if" Expr BlockOrStatement *("elif" Expr BlockOrStatement) ["else" BlockOrStatement]
/// ```
fn parse_if(s: &mut impl ITokenStream) -> Result<ast::If> {
    let start_pos = s.get_pos().clone();

    expect_token_kind!(s, TokenKind::IfKeyword);
    s.next()?;
    let cond = parse_expr(s, false)?;
    let then = parse_block_or_statement(s)?;

    if is_token_kind!(s, TokenKind::NewLine)
        && matches!(
            s.lookahead(1)?.kind,
            TokenKind::ElifKeyword | TokenKind::ElseKeyword
        )
    {
        s.next()?;
    }

    let mut elseif: Vec<ast::Elseif> = Vec::new();
    while is_token_kind!(s, TokenKind::ElifKeyword) {
        s.next()?;
        let elif_cond = parse_expr(s, false)?;
        let elif_then = parse_block_or_statement(s)?;
        if is_token_kind!(s, TokenKind::NewLine)
            && matches!(
                s.lookahead(1)?.kind,
                TokenKind::ElifKeyword | TokenKind::ElseKeyword
            )
        {
            s.next()?;
        }
        elseif.push(ast::Elseif {
            cond: elif_cond,
            then: elif_then,
        });
    }

    let else_statement = if is_token_kind!(s, TokenKind::ElseKeyword) {
        s.next()?;
        Some(Box::new(parse_block_or_statement(s)?))
    } else {
        None
    };

    return Ok(ast::If {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().to_owned(),
        },
        cond: Box::new(cond),
        then: Box::new(then),
        elseif,
        else_statement,
    });
}

/// ```abnf
/// FnExpr = "@" Params [":" Type] Block
/// ```
fn parse_fn_expr(s: &mut impl ITokenStream) -> Result<ast::Fn> {
    let start_pos = s.get_pos().clone();

    expect_token_kind!(s, TokenKind::At)?;
    s.next()?;

    let params = parse_params(s)?;

    let ty = if !is_token_kind!(s, TokenKind::Colon) {
        s.next()?;
        Some(parse_type(s)?)
    } else {
        None
    };

    let body = parse_block(s)?;

    return Ok(ast::Fn {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().clone(),
        },
        args: params,
        ret_type: ty,
        children: body,
    });
}

/// ```abnf
/// Match = "match" Expr "{" [MatchCases] [defaultCase] "}"
/// MatchCases = "case" Expr "=>" BlockOrStatement *(SEP "case" Expr "=>" BlockOrStatement) [SEP]
/// DefaultCase = "default" "=>" BlockOrStatement [SEP]
/// ```
fn parse_match(s: &mut impl ITokenStream) -> Result<ast::Match> {
    let start_pos = s.get_pos().clone();

    expect_token_kind!(s, TokenKind::MatchKeyword)?;
    s.next()?;
    let about = parse_expr(s, false)?;

    expect_token_kind!(s, TokenKind::OpenBrace)?;
    s.next()?;

    if is_token_kind!(s, TokenKind::NewLine) {
        s.next()?;
    }

    let mut qs: Vec<ast::MatchQ> = Vec::new();
    while !is_token_kind!(s, TokenKind::DefaultKeyword | TokenKind::CloseBrace) {
        expect_token_kind!(s, TokenKind::CaseKeyword)?;
        s.next()?;
        let q = parse_expr(s, false)?;
        expect_token_kind!(s, TokenKind::Arrow)?;
        s.next()?;
        let a = parse_block_or_statement(s)?;
        qs.push(ast::MatchQ { q, a });

        // separator
        match s.get_token_kind() {
            TokenKind::NewLine => {
                s.next()?;
            }
            TokenKind::Comma => {
                s.next()?;
                if is_token_kind!(s, TokenKind::NewLine) {
                    s.next();
                }
                break;
            }
            TokenKind::DefaultKeyword | TokenKind::CloseBrace => {}
            _ => {
                return Err(Box::new(AiScriptSyntaxError::new(
                    "separator expected",
                    s.get_pos().to_owned(),
                )))
            }
        }
    }

    let default = if is_token_kind!(s, TokenKind::DefaultKeyword) {
        s.next()?;
        expect_token_kind!(s, TokenKind::Arrow)?;
        s.next()?;
        let default = parse_block_or_statement(s)?;

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
            TokenKind::CloseBrace => {}
            _ => {
                return Err(Box::new(AiScriptSyntaxError::new(
                    "separator expected",
                    s.get_pos().to_owned(),
                )));
            }
        }

        Some(Box::new(default))
    } else {
        None
    };

    expect_token_kind!(s, TokenKind::CloseBrace)?;
    s.next()?;

    return Ok(ast::Match {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().to_owned(),
        },
        about: Box::new(about),
        qs,
        default,
    });
}

/// ```abnf
/// Eval = "eval" Block
/// ```
fn parse_eval(s: &mut impl ITokenStream) -> Result<ast::Block> {
    let start_pos = s.get_pos().clone();

    expect_token_kind!(s, TokenKind::EvalKeyword)?;
    s.next()?;
    let statements = parse_block(s)?;

    return Ok(ast::Block {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().to_owned(),
        },
        statements,
    });
}

/// ```abnf
/// Exists = "exists" Reference
/// ```
fn parse_exists(s: &mut impl ITokenStream) -> Result<ast::Exists> {
    let start_pos = s.get_pos().clone();

    expect_token_kind!(s, TokenKind::ExistsKeyword)?;
    s.next()?;
    let identifier = parse_reference(s)?;

    return Ok(ast::Exists {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().clone(),
        },
        identifier,
    });
}

/// ```abnf
/// Reference = IDENT *(":" IDENT)
/// ```
fn parse_reference(s: &mut impl ITokenStream) -> Result<ast::Identifier> {
    let start_pos = s.get_pos().clone();

    let mut segs: Vec<Utf16String> = Vec::new();
    loop {
        if !segs.is_empty() {
            if is_token_kind!(s, TokenKind::Colon) {
                if s.get_token().has_left_spacing {
                    return Err(Box::new(AiScriptSyntaxError::new(
                        "Cannot use spaces in a reference.",
                        s.get_pos().to_owned(),
                    )));
                }
                s.next()?;
                if s.get_token().has_left_spacing {
                    return Err(Box::new(AiScriptSyntaxError::new(
                        "Cannot use spaces in a reference.",
                        s.get_pos().to_owned(),
                    )));
                }
            } else {
                break;
            }
        }
        let TokenKind::Identifier(ident) = s.get_token_kind() else {
            return Err(s.unexpected_token());
        };
        segs.push(ident.clone());
        s.next()?;
    }
    return Ok(ast::Identifier {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().to_owned(),
        },
        name: Utf16Str::new(&utf16!(":")).join(segs.iter()),
    });
}

/// ```abnf
/// Object = "{" [IDENT ":" Expr *(SEP IDENT ":" Expr) [SEP]] "}"
/// ```
fn parse_object(s: &mut impl ITokenStream, is_static: bool) -> Result<ast::Obj> {
    let start_pos = s.get_pos().clone();

    expect_token_kind!(s, TokenKind::OpenBrace)?;
    s.next()?;

    while is_token_kind!(s, TokenKind::NewLine) {
        s.next()?;
    }

    let mut map: HashMap<Utf16String, ast::Expression> = HashMap::new();
    while !is_token_kind!(s, TokenKind::CloseBrace) {
        let TokenKind::Identifier(k) = s.get_token_kind() else {
            return Err(s.unexpected_token());
        };
        let k = k.clone();

        expect_token_kind!(s, TokenKind::Colon)?;
        s.next()?;

        let v = parse_expr(s, is_static)?;

        map.insert(k, v);

        // separator
        match s.get_token_kind() {
            TokenKind::NewLine | TokenKind::Comma => {
                s.next()?;
                while is_token_kind!(s, TokenKind::NewLine) {
                    s.next()?;
                }
            }
            TokenKind::CloseBrace => {}
            _ => {
                return Err(Box::new(AiScriptSyntaxError::new(
                    "separator expected",
                    s.get_pos().to_owned(),
                )));
            }
        }
    }

    expect_token_kind!(s, TokenKind::CloseBrace);
    s.next()?;

    return Ok(ast::Obj {
        loc: Loc {
            start: start_pos,
            end: s.get_pos().to_owned(),
        },
        value: map,
    });
}

/// ```abnf
/// Array = "[" [Expr *(SEP Expr) [SEP]] "]"
/// ```
fn parse_array(s: &mut impl ITokenStream, is_static: bool) -> Result<ast::Arr> {
    let start_pos = s.get_pos().clone();

    expect_token_kind!(s, TokenKind::OpenBracket)?;
    s.next()?;

    while is_token_kind!(s, TokenKind::NewLine) {
        s.next()?;
    }

    let mut value: Vec<ast::Expression> = Vec::new();
    while is_token_kind!(s, TokenKind::CloseBracket) {
        value.push(parse_expr(s, is_static)?);

        // separator
        match s.get_token_kind() {
            TokenKind::NewLine | TokenKind::Comma => {
                s.next()?;
                while is_token_kind!(s, TokenKind::NewLine) {
                    s.next()?;
                }
            }
            TokenKind::CloseBracket => {}
            _ => {
                return Err(Box::new(AiScriptSyntaxError::new(
                    "separator expected",
                    s.get_pos().to_owned(),
                )));
            }
        }
    }

    expect_token_kind!(s, TokenKind::CloseBracket);
    s.next()?;

    return Ok(ast::Arr {
        loc: Loc { start: start_pos, end: s.get_pos().to_owned() },
        value,
    });
}
