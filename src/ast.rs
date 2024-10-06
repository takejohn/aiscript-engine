//! ASTノード

use std::collections::HashMap;

use derive_node::Node;
use derive_wrapper::Wrapper;

use crate::{common::Position, string::Utf16String};

#[derive(Clone)]
pub struct Loc {
    pub start: Position,
    pub end: Position,
}

pub trait Node {
    /// コード位置
    fn loc(&self) -> &Loc;
}

#[derive(Node, Wrapper)]
pub enum NodeWrapper {
    /// 名前空間
    Ns(Namespace),

    /// メタデータ定義
    Meta(Meta),

    Statement(Statement),
    Expression(Expression),
    TypeSource(TypeSource),
    Attribute(Attribute),
}

#[derive(Node, Wrapper)]
pub enum StatementOrExpression {
    Statement(Statement),
    Expression(Expression),
}

#[derive(Node)]
pub struct Namespace {
    pub loc: Loc,

    /// 空間名
    pub name: Utf16String,

    /// メンバー
    pub members: Vec<NamespaceMember>,
}

#[derive(Node, Wrapper)]
pub enum NamespaceMember {
    Namespace(Namespace),

    Def(Definition),
}

#[derive(Node)]
pub struct Meta {
    pub loc: Loc,

    /// 名
    pub name: Option<Utf16String>,

    /// 値
    pub value: Expression,
}

#[derive(Node, Wrapper)]
pub enum Statement {
    /// 変数宣言文
    Def(Definition),

    /// return文
    Return(Return),

    /// each文
    Each(Each),

    /// for文
    For(For),

    /// loop文
    Loop(Loop),

    /// break文
    Break(Break),

    /// continue文
    Continue(Continue),

    /// 代入文
    Assign(Assign),

    /// 加算代入文
    AddAssign(AddAssign),

    /// 減算代入文
    SubAssign(SubAssign),
}

#[derive(Node)]
pub struct Definition {
    pub loc: Loc,

    /// 宣言式
    pub dest: Expression,

    /// 変数の型
    pub var_type: Option<TypeSource>,

    /// 式
    pub expr: Expression,

    /// ミュータブルか否か
    pub is_mut: bool,

    /// 付加された属性
    pub attr: Vec<Attribute>,
}

#[derive(Node)]
pub struct Attribute {
    pub loc: Loc,

    /// 属性名
    pub name: Utf16String,

    /// 値
    pub value: Expression,
}

#[derive(Node)]
pub struct Return {
    pub loc: Loc,

    /// 式
    pub expr: Expression,
}

#[derive(Node)]
pub struct Each {
    pub loc: Loc,

    /// each文
    pub var: Expression,

    /// 配列
    pub items: Expression,

    /// 本体処理
    pub for_statement: Box<Statement>,
}

#[derive(Node)]
pub struct For {
    pub loc: Loc,

    /// イテレータ変数名
    pub var: Option<Utf16String>,

    /// 開始値
    pub from: Option<Expression>,

    /// 終値
    pub to: Option<Expression>,

    /// 回数
    pub times: Option<Expression>,

    /// 本体処理
    pub for_statement: Box<Statement>,
}

#[derive(Node)]
pub struct Loop {
    pub loc: Loc,

    /// 処理
    pub statements: Vec<Statement>,
}

#[derive(Node)]
pub struct Break {
    pub loc: Loc,
}

#[derive(Node)]
pub struct Continue {
    pub loc: Loc,
}

#[derive(Node)]
pub struct AddAssign {
    pub loc: Loc,

    /// 代入先
    pub dest: Expression,

    /// 式
    pub expr: Expression,
}

#[derive(Node)]
pub struct SubAssign {
    pub loc: Loc,

    /// 代入先
    pub dest: Expression,

    /// 式
    pub expr: Expression,
}

#[derive(Node)]
pub struct Assign {
    pub loc: Loc,

    /// 代入先
    pub dest: Expression,

    /// 式
    pub expr: Expression,
}

#[derive(Node, Wrapper)]
pub enum Expression {
    /// if式
    If(If),

    /// 関数
    Fn(Fn),

    /// パターンマッチ
    Match(Match),

    /// ブロックまたはeval式
    Block(Block),

    /// 変数の存在判定
    Exists(Exists),

    /// テンプレート
    Tmpl(Tmpl),

    /// 文字列リテラル
    Str(Str),

    /// 数値リテラル
    Num(Num),

    /// 真理値リテラル
    Bool(Bool),

    /// nullリテラル
    Null(Null),

    /// オブジェクト
    Obj(Obj),

    /// 配列
    Arr(Arr),

    /// 否定
    Not(Not),

    Pow(Pow),
    Mul(Mul),
    Div(Div),
    Rem(Rem),
    Add(Add),
    Sub(Sub),
    Lt(Lt),
    Lteq(Lteq),
    Gt(Gt),
    Gteq(Gteq),
    Eq(Eq),
    Neq(Neq),
    And(And),
    Or(Or),

    /// 変数などの識別子
    Identifier(Identifier),

    /// 関数呼び出し
    Call(Call),

    /// 配列要素アクセス
    Index(Index),

    /// プロパティアクセス
    Prop(Prop),
}

#[derive(Node)]
pub struct Not {
    pub loc: Loc,

    /// 式
    pub expr: Box<Expression>,
}

#[derive(Node)]
pub struct Pow {
    pub loc: Loc,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Node)]
pub struct Mul {
    pub loc: Loc,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Node)]
pub struct Div {
    pub loc: Loc,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Node)]
pub struct Rem {
    pub loc: Loc,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Node)]
pub struct Add {
    pub loc: Loc,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Node)]
pub struct Sub {
    pub loc: Loc,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Node)]
pub struct Lt {
    pub loc: Loc,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Node)]
pub struct Lteq {
    pub loc: Loc,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Node)]
pub struct Gt {
    pub loc: Loc,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Node)]
pub struct Gteq {
    pub loc: Loc,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Node)]
pub struct Eq {
    pub loc: Loc,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Node)]
pub struct Neq {
    pub loc: Loc,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Node)]
pub struct And {
    pub loc: Loc,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Node)]
pub struct Or {
    pub loc: Loc,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Node)]
pub struct If {
    pub loc: Loc,

    /// 条件式
    pub cond: Box<Expression>,

    /// then節
    pub then: Box<StatementOrExpression>,

    pub elseif: Vec<Elseif>,

    /// else節
    pub else_process: Box<StatementOrExpression>,
}

#[derive(Node)]
pub struct Elseif {
    pub loc: Loc,

    /// elifの条件式
    pub cond: Expression,

    /// elif節
    pub then: StatementOrExpression,
}

#[derive(Node)]
pub struct Fn {
    pub loc: Loc,

    pub args: Vec<FnArg>,

    /// 戻り値の型
    pub ret_type: TypeSource,

    /// 本体処理
    pub children: Vec<StatementOrExpression>,
}

#[derive(Node)]
pub struct FnArg {
    pub loc: Loc,

    /// 引数名
    pub dest: Expression,

    pub optional: bool,

    /// 引数の初期値
    pub default: Option<Expression>,

    /// 引数の型
    pub arg_type: Option<TypeSource>,
}

#[derive(Node)]
pub struct Match {
    pub loc: Loc,

    /// 対象
    pub about: Box<Expression>,

    pub qs: Vec<MatchQs>,

    /// デフォルト値
    pub default: Option<Box<StatementOrExpression>>,
}

#[derive(Node)]
pub struct MatchQs {
    pub loc: Loc,

    /// 条件
    pub q: Expression,

    /// 結果
    pub a: StatementOrExpression,
}

#[derive(Node)]
pub struct Block {
    pub loc: Loc,

    /// 処理
    pub statements: Vec<StatementOrExpression>,
}

#[derive(Node)]
pub struct Exists {
    pub loc: Loc,

    /// 変数名
    pub identifier: Identifier,
}

#[derive(Node)]
pub struct Tmpl {
    pub loc: Loc,

    /// 処理
    pub tmpl: Vec<Expression>,
}

#[derive(Node)]
pub struct Str {
    pub loc: Loc,

    /// 文字列
    pub value: Utf16String,
}

#[derive(Node)]
pub struct Num {
    pub loc: Loc,

    /// 数値
    pub value: f64,
}

#[derive(Node)]
pub struct Bool {
    pub loc: Loc,

    /// 真理値
    pub value: bool,
}

#[derive(Node)]
pub struct Null {
    pub loc: Loc,
}

#[derive(Node)]
pub struct Obj {
    pub loc: Loc,

    /// オブジェクト
    pub value: HashMap<Utf16String, Expression>,
}

#[derive(Node)]
pub struct Arr {
    pub loc: Loc,

    /// アイテム
    pub value: Vec<Expression>,
}

#[derive(Node)]
pub struct Identifier {
    pub loc: Loc,

    /// 変数名
    pub name: Utf16String,
}

#[derive(Node)]
pub struct Call {
    pub loc: Loc,

    /// 対象
    pub target: Box<Expression>,

    /// 引数
    pub args: Vec<Expression>,
}

#[derive(Node)]
pub struct Index {
    pub loc: Loc,

    /// 対象
    pub target: Box<Expression>,

    /// インデックス
    pub name: Box<Expression>,
}

#[derive(Node)]
pub struct Prop {
    pub loc: Loc,

    /// 対象
    pub target: Box<Expression>,

    /// プロパティ名
    pub name: Utf16String,
}

#[derive(Node, Wrapper)]
pub enum TypeSource {
    /// 名前付き型
    NamedTypeSource(NamedTypeSource),

    /// 関数の型
    FnTypeSource(FnTypeSource),
}

#[derive(Node)]
pub struct NamedTypeSource {
    pub loc: Loc,

    /// 型名
    pub name: Utf16String,

    /// 内側の型
    pub inner: Option<Box<TypeSource>>,
}

#[derive(Node)]
pub struct FnTypeSource {
    pub loc: Loc,

    /// 引数の型
    pub args: Vec<TypeSource>,

    /// 戻り値の型
    pub result: Box<TypeSource>,
}
