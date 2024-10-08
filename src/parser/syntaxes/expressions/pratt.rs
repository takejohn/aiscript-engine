// NOTE: infix(中置演算子)ではlbpを大きくすると右結合、rbpを大きくすると左結合の演算子になります。
// この値は演算子が左と右に対してどのくらい結合力があるかを表わしています。詳細はpratt parsingの説明ページを参照してください。

use crate::{
    ast, error::Result, expect_token_kind, is_token_kind, parser::{streams::ITokenStream, syntaxes::expressions::{parse_atom, parse_infix, parse_postfix, parse_prefix}, token::TokenKind}
};

pub(super) type BindingPower = i32;

fn prefix_binding_power(op: &TokenKind) -> Option<BindingPower> {
    match op {
        TokenKind::Plus | TokenKind::Minus | TokenKind::Not => Some(14),
        _ => None,
    }
}

/// タプルは(lbp, rbp)の順
fn infix_binding_power(op: &TokenKind) -> Option<(BindingPower, BindingPower)> {
    match op {
        TokenKind::Dot => Some((18, 19)),
        TokenKind::Hat => Some((17, 16)),
        TokenKind::Asterisk | TokenKind::Slash | TokenKind::Percent => Some((12, 13)),
        TokenKind::Plus | TokenKind::Minus => Some((10, 11)),
        TokenKind::Lt | TokenKind::LtEq | TokenKind::Gt | TokenKind::GtEq => Some((8, 9)),
        TokenKind::Eq2 | TokenKind::NotEq => Some((6, 7)),
        TokenKind::And2 => Some((4, 5)),
        TokenKind::Or2 => Some((2, 3)),
        _ => None,
    }
}

fn postfix_binding_power(op: &TokenKind) -> Option<BindingPower> {
    match op {
        TokenKind::OpenParen | TokenKind::OpenBracket => Some(20),
        _ => None,
    }
}

pub(super) fn parse_pratt(
    s: &mut impl ITokenStream,
    min_bp: BindingPower,
) -> Result<ast::Expression> {
    // pratt parsing
    // https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

    let token_kind = s.get_token_kind();
    let prefix = prefix_binding_power(token_kind);
    let mut left: ast::Expression = if let Some(prefix) = prefix {
        parse_prefix(s, prefix)?
    } else {
        parse_atom(s, false)?
    };

    loop {
        // 改行のエスケープ
        if is_token_kind!(s, TokenKind::BackSlash) {
            s.next()?;
            expect_token_kind!(s, TokenKind::NewLine)?;
            s.next()?;
        }

        let token_kind = s.get_token_kind();

        let postfix = postfix_binding_power(token_kind);
        if let Some(postfix) = postfix {
            if postfix < min_bp {
                break;
            }

            if matches!(token_kind, TokenKind::OpenBracket | TokenKind::OpenParen) && s.get_token().has_left_spacing {
                // 前にスペースがある場合は後置演算子として処理しない
            } else {
                left = parse_postfix(s, left)?;
                continue;
            }
        }

        let infix = infix_binding_power(token_kind);
        if let Some((lbp, rbp)) = infix {
            if lbp < min_bp {
                break;
            }

            left = parse_infix(s, left, rbp)?;
            continue;
        }

        break;
    }

    return Ok(left);
}
