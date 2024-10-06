use crate::{ast, error::Result, parser::streams::ITokenStream};

mod pratt;

pub(super) fn parse_expr(s: &mut impl ITokenStream, is_static: bool) -> Result<ast::Expression> {
    todo!()
}
