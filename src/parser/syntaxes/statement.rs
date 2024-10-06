use crate::{ast, error::Result, parser::streams::ITokenStream};

pub(super) fn parse_statement(s: &mut impl ITokenStream) -> Result<ast::Statement> {
    todo!()
}

/// ```abnf
/// DefStatement = VarDef / FnDef
/// ```
pub(super) fn parse_def_statement(s: &mut impl ITokenStream) -> Result<ast::Definition> {
    todo!()
}

/// ```abnf
/// BlockOrStatement = Block / Statement
/// ```
pub(super) fn parse_block_or_statement(
    s: &mut impl ITokenStream,
) -> Result<ast::StatementOrExpression> {
    todo!()
}

/// ```abnf
/// VerDef = ("let" / "var") Dest [":" Type] "=" Expr
/// ```
fn parse_var_def(s: &mut impl ITokenStream) -> Result<ast::Definition> {
    todo!()
}

/// ```abnf
/// FnDef = "@" IDENT Params [":" Type] Block
/// ```
fn parse_fn_def(s: &mut impl ITokenStream) -> Result<ast::Definition> {
    todo!()
}

/// ```abnf
/// Out = "<:" Expr
/// ```
fn parse_out(s: &mut impl ITokenStream) -> Result<ast::Call> {
    todo!()
}

/// ```abnf
/// Each = "each" "(" "let" Dest "," Expr ")" BlockOrStatement
///      / "each"     "let" Dest "," Expr     BlockOrStatement
/// ```
fn parse_each(s: &mut impl ITokenStream) -> Result<ast::Each> {
    todo!()
}

/// ```abnf
/// For = ForRange / ForTimes
/// ForRange = "for" "(" "let" IDENT ["=" Expr] "," Expr ")" BlockOrStatement
///          / "for"     "let" IDENT ["=" Expr] "," Expr     BlockOrStatement
/// ForTimes = "for" "(" Expr ")" BlockOrStatement
///          / "for"     Expr     BlockOrStatement
/// ```
fn parse_for(s: &mut impl ITokenStream) -> Result<ast::For> {
    todo!()
}

/// ```abnf
/// Return = "return" Expr
/// ```
fn parse_return(s: &mut impl ITokenStream) -> Result<ast::Return> {
    todo!()
}

/// ```abnf
/// StatementWithAttr = *Attr Statement
/// ```
fn parse_statement_with_attr(s: &mut impl ITokenStream) -> Result<ast::Definition> {
    todo!()
}

/// ```abnf
/// Attr = "#[" IDENT [StaticExpr] "]"
/// ```
fn parse_attr(s: &mut impl ITokenStream) -> Result<ast::Attribute> {
    todo!()
}

/// ```abnf
/// Loop = "loop" Block
/// ```
fn parse_loop(s: &mut impl ITokenStream) -> Result<ast::Loop> {
    todo!()
}

/// ```abnf
/// Loop = "do" BlockOrStatement "while" Expr
/// ```
fn parse_do_while(s: &mut impl ITokenStream) -> Result<ast::Loop> {
    todo!()
}

/// ```abnf
/// Loop = "while" Expr BlockOrStatement
/// ```
fn parse_while(s: &mut impl ITokenStream) -> Result<ast::Loop> {
    todo!()
}

/// ```abnf
/// Assign = Expr ("=" / "+=" / "-=") Expr
/// ```
fn try_parse_assign(s: &mut impl ITokenStream, dest: ast::Expression) -> Option<ast::Statement> {
    todo!()
}
