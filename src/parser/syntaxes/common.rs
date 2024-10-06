use crate::{ast, error::Result, parser::streams::ITokenStream};

/// ```abnf
/// Dest = IDENT / Expr
/// ```
pub(super) fn parse_dest(s: &mut impl ITokenStream) -> Result<ast::Expression> {
    todo!()
}

/// ```abnf
/// Params = "(" [Dest [":" Type] *(SEP Dest [":" Type])] ")"
/// ```
pub(super) fn parse_params(s: &mut impl ITokenStream) -> Result<Vec<ast::FnArg>> {
    todo!()
}

/// ```abnf
/// Block = "{" *Statement "}"
/// ```
pub(super) fn parse_block(s: &mut impl ITokenStream) -> Result<Vec<ast::StatementOrExpression>> {
    todo!()
}

pub(super) fn parse_type(s: &mut impl ITokenStream) -> Result<ast::TypeSource> {
    todo!()
}
