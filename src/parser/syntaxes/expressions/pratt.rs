// NOTE: infix(中置演算子)ではlbpを大きくすると右結合、rbpを大きくすると左結合の演算子になります。
// この値は演算子が左と右に対してどのくらい結合力があるかを表わしています。詳細はpratt parsingの説明ページを参照してください。
// https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

use crate::{
    ast,
    error::Result,
    parser::{streams::ITokenStream, token::TokenKind},
};

pub(super) type BindingPower = i32;

fn prefix_binding_power(op: TokenKind) -> Option<BindingPower> {
    match op {
        TokenKind::Plus | TokenKind::Minus | TokenKind::Not => Some(14),
        _ => None,
    }
}

/// タプルは(lbp, rbp)の順
fn infix_binding_power(op: TokenKind) -> Option<(BindingPower, BindingPower)> {
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

fn postfix_binding_power(op: TokenKind) -> Option<BindingPower> {
    match op {
        TokenKind::OpenParen | TokenKind::OpenBracket => Some(20),
        _ => None,
    }
}

pub(super) fn parse_pratt(
    s: &mut impl ITokenStream,
    min_bp: BindingPower,
) -> Result<ast::Expression> {
    todo!()
}
