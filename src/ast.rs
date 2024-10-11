//! ASTノード

use std::collections::HashMap;

use derive_node::Node;
use derive_wrapper::Wrapper;

use crate::string::{Utf16Str, Utf16String};

pub use crate::common::Position;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Loc {
    pub start: Position,
    pub end: Position,
}

pub trait Node {
    /// コード位置
    fn loc(&self) -> &Loc;
}

/// 予約語でない名前を持つノード。
pub trait NamedNode: Node {
    fn name(&self) -> &Utf16Str;
}

#[derive(Debug, PartialEq, Eq, Node, Wrapper)]
pub enum NodeWrapper {
    /// 名前空間
    Ns(Namespace),

    /// メタデータ定義
    Meta(Meta),

    Statement(Statement),
    Expr(Expression),
    TypeSource(TypeSource),
    Attr(Attribute),
}

impl From<StatementOrExpression> for NodeWrapper {
    fn from(value: StatementOrExpression) -> Self {
        match value {
            StatementOrExpression::Statement(statement) => statement.into(),
            StatementOrExpression::Expression(expression) => expression.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Node, Wrapper)]
pub enum StatementOrExpression {
    Statement(Statement),
    Expression(Expression),
}

impl StatementOrExpression {
    pub fn from_statement(statement: impl Into<Statement>) -> Self {
        StatementOrExpression::Statement(statement.into())
    }

    pub fn from_expr(expr: impl Into<Expression>) -> Self {
        StatementOrExpression::Expression(expr.into())
    }
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Namespace {
    pub loc: Loc,

    /// 空間名
    pub name: Utf16String,

    /// メンバー
    pub members: Vec<NamespaceMember>,
}

impl NamedNode for Namespace {
    fn name(&self) -> &Utf16Str {
        &self.name
    }
}

#[derive(Debug, PartialEq, Eq, Node, Wrapper)]
pub enum NamespaceMember {
    Namespace(Namespace),

    Def(Definition),
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Meta {
    pub loc: Loc,

    /// 名
    pub name: Option<Utf16String>,

    /// 値
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq, Node, Wrapper)]
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
}

#[derive(Debug, PartialEq, Eq, Node)]
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

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Attribute {
    pub loc: Loc,

    /// 属性名
    pub name: Utf16String,

    /// 値
    pub value: Expression,
}

impl NamedNode for Attribute {
    fn name(&self) -> &Utf16Str {
        &self.name
    }
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Return {
    pub loc: Loc,

    /// 式
    pub expr: Expression,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Each {
    pub loc: Loc,

    /// each文
    pub var: Expression,

    /// 配列
    pub items: Expression,

    /// 本体処理
    pub for_statement: Box<StatementOrExpression>,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct For {
    pub loc: Loc,

    pub iter: ForIterator,

    /// 本体処理
    pub for_statement: Box<StatementOrExpression>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ForIterator {
    Range {
        /// イテレータ変数名
        var: Utf16String,

        /// 開始値
        from: Expression,

        /// 終値
        to: Expression,
    },
    Number {
        /// 回数
        times: Expression,
    },
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Loop {
    pub loc: Loc,

    /// 処理
    pub statements: Vec<StatementOrExpression>,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Break {
    pub loc: Loc,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Continue {
    pub loc: Loc,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Assign {
    pub loc: Loc,

    pub op: AssignOperator,

    /// 代入先
    pub dest: Expression,

    /// 式
    pub expr: Expression,
}

#[derive(Debug, PartialEq, Eq)]
pub enum AssignOperator {
    /// 再代入文
    Reassign,

    /// 加算代入文
    AddAssign,

    /// 減算代入文
    SubAssign,
}

#[derive(Debug, PartialEq, Eq, Node, Wrapper)]
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

    /// 二項演算
    Binary(Binary),

    /// 変数などの識別子
    Identifier(Identifier),

    /// 関数呼び出し
    Call(Call),

    /// 配列要素アクセス
    Index(Index),

    /// プロパティアクセス
    Prop(Prop),
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Binary {
    pub loc: Loc,

    pub op: BinaryOperator,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    Pow,
    Mul,
    Div,
    Rem,
    Add,
    Sub,
    Lt,
    Lteq,
    Gt,
    Gteq,
    Eq,
    Neq,
    And,
    Or,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Not {
    pub loc: Loc,

    /// 式
    pub expr: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct If {
    pub loc: Loc,

    /// 条件式
    pub cond: Box<Expression>,

    /// then節
    pub then: Box<StatementOrExpression>,

    pub elseif: Vec<Elseif>,

    /// else節
    pub else_statement: Option<Box<StatementOrExpression>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Elseif {
    /// elifの条件式
    pub cond: Expression,

    /// elif節
    pub then: StatementOrExpression,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Fn {
    pub loc: Loc,

    pub args: Vec<FnArg>,

    /// 戻り値の型
    pub ret_type: Option<TypeSource>,

    /// 本体処理
    pub children: Vec<StatementOrExpression>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct FnArg {
    /// 引数名
    pub dest: Expression,

    /// 実引数が省略された場合の挙動
    pub value: FnArgValue,

    /// 引数の型
    pub arg_type: Option<TypeSource>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FnArgValue {
    Optional,

    /// 引数の初期値
    Default(Expression),

    Required,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Match {
    pub loc: Loc,

    /// 対象
    pub about: Box<Expression>,

    pub qs: Vec<MatchQ>,

    /// デフォルト値
    pub default: Option<Box<StatementOrExpression>>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MatchQ {
    /// 条件
    pub q: Expression,

    /// 結果
    pub a: StatementOrExpression,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Block {
    pub loc: Loc,

    /// 処理
    pub statements: Vec<StatementOrExpression>,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Exists {
    pub loc: Loc,

    /// 変数名
    pub identifier: Identifier,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Tmpl {
    pub loc: Loc,

    /// 処理
    pub tmpl: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Str {
    pub loc: Loc,

    /// 文字列
    pub value: Utf16String,
}

#[derive(Debug, Node)]
pub struct Num {
    pub loc: Loc,

    /// 数値
    pub value: f64,
}

impl PartialEq for Num {
    fn eq(&self, other: &Self) -> bool {
        self.loc == other.loc && self.value.to_bits() == other.value.to_bits()
    }
}

impl Eq for Num {}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Bool {
    pub loc: Loc,

    /// 真理値
    pub value: bool,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Null {
    pub loc: Loc,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Obj {
    pub loc: Loc,

    /// オブジェクト
    pub value: HashMap<Utf16String, Expression>,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Arr {
    pub loc: Loc,

    /// アイテム
    pub value: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Identifier {
    pub loc: Loc,

    /// 変数名
    pub name: Utf16String,
}

impl NamedNode for Identifier {
    fn name(&self) -> &Utf16Str {
        &self.name
    }
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Call {
    pub loc: Loc,

    /// 対象
    pub target: Box<Expression>,

    /// 引数
    pub args: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Index {
    pub loc: Loc,

    /// 対象
    pub target: Box<Expression>,

    /// インデックス
    pub index: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct Prop {
    pub loc: Loc,

    /// 対象
    pub target: Box<Expression>,

    /// プロパティ名
    pub name: Utf16String,
}

impl NamedNode for Prop {
    fn name(&self) -> &Utf16Str {
        &self.name
    }
}

#[derive(Debug, PartialEq, Eq, Node, Wrapper)]
pub enum TypeSource {
    /// 名前付き型
    NamedTypeSource(NamedTypeSource),

    /// 関数の型
    FnTypeSource(FnTypeSource),
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct NamedTypeSource {
    pub loc: Loc,

    /// 型名
    pub name: Utf16String,

    /// 内側の型
    pub inner: Option<Box<TypeSource>>,
}

#[derive(Debug, PartialEq, Eq, Node)]
pub struct FnTypeSource {
    pub loc: Loc,

    /// 引数の型
    pub args: Vec<TypeSource>,

    /// 戻り値の型
    pub result: Box<TypeSource>,
}
