//! ASTノード

use std::collections::HashMap;

use crate::common::Position;

#[derive(Clone)]
pub struct Loc {
    pub start: Position,
    pub end: Position,
}

pub struct Node {
    /// コード位置
    pub loc: Loc,

    pub kind: NodeKind,
}

pub enum NodeKind {
    /// 名前空間
    Ns(Namespace),

    /// メタデータ定義
    Meta(Meta),

    Statement(Statement),
    Expression(Expression),
    TypeSource(TypeSource),
    Attribute(Attribute),
}

enum StatementOrExpression {
    Statement(Statement),
    Expression(Expression),
}

pub struct Namespace {
    /// 空間名
    pub name: String,

    /// メンバー
    pub members: Vec<NamespaceMember>,
}

enum NamespaceMember {
    Namespace(Namespace),

    Def(Definition),
}

pub struct Meta {
    /// 名
    pub name: Option<String>,

    /// 値
    pub value: Expression,
}

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

pub struct Definition {
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

pub struct Attribute {
    /// 属性名
    pub name: String,

    /// 値
    pub value: Expression,
}

pub struct Return {
    /// 式
    pub expr: Expression,
}

pub struct Each {
    /// each文
    pub var: Expression,

    /// 配列
    pub items: Expression,

    /// 本体処理
    pub for_statement: Box<Statement>,
}

pub struct For {
    /// イテレータ変数名
    pub var: Option<String>,

    /// 開始値
    pub from: Option<Expression>,

    /// 終値
    pub to: Option<Expression>,

    /// 回数
    pub times: Option<Expression>,

    /// 本体処理
    pub for_statement: Box<Statement>,
}

pub struct Loop {
    /// 処理
    pub statements: Vec<Statement>,
}

pub struct Break;

pub struct Continue;

pub struct AddAssign {
    /// 代入先
    pub dest: Expression,

    /// 式
    pub expr: Expression,
}

pub struct SubAssign {
    /// 代入先
    pub dest: Expression,

    /// 式
    pub expr: Expression,
}

pub struct Assign {
    /// 代入先
    pub dest: Expression,

    /// 式
    pub expr: Expression,
}

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

pub struct Not {
    /// 式
    pub expr: Box<Expression>,
}

pub struct Pow {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub struct Mul {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub struct Div {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub struct Rem {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub struct Add {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub struct Sub {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub struct Lt {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub struct Lteq {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub struct Gt {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub struct Gteq {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub struct Eq {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub struct Neq {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub struct And {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub struct Or {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

pub struct If {
    /// 条件式
    pub cond: Box<Expression>,

    /// then節
    pub then: Box<StatementOrExpression>,

    pub elseif: Vec<Elseif>,

    /// else節
    pub else_process: Box<StatementOrExpression>,
}

pub struct Elseif {
    /// elifの条件式
    pub cond: Expression,

    /// elif節
    pub then: StatementOrExpression,
}

pub struct Fn {
    pub args: Vec<FnArg>,

    /// 戻り値の型
    pub ret_type: TypeSource,

    /// 本体処理
    pub children: Vec<StatementOrExpression>,
}

struct FnArg {
    /// 引数名
    pub dest: Expression,

    pub optional: bool,

    /// 引数の初期値
    pub default: Option<Expression>,

    /// 引数の型
    pub arg_type: Option<TypeSource>,
}

pub struct Match {
    /// 対象
    pub about: Box<Expression>,

    pub qs: Vec<MatchQs>,

    /// デフォルト値
    pub default: Option<Box<StatementOrExpression>>,
}

struct MatchQs {
    /// 条件
    pub q: Expression,

    /// 結果
    pub a: StatementOrExpression,
}

pub struct Block {
    /// 処理
    pub statements: Vec<StatementOrExpression>,
}

pub struct Exists {
    /// 変数名
    pub identifier: Identifier,
}

pub struct Tmpl {
    /// 処理
    pub tmpl: Vec<Expression>,
}

pub struct Str {
    /// 文字列
    pub value: String,
}

pub struct Num {
    /// 数値
    pub value: f64,
}

pub struct Bool {
    /// 真理値
    pub value: bool,
}

pub struct Null;

pub struct Obj {
    /// オブジェクト
    pub value: HashMap<String, Expression>,
}

pub struct Arr {
    /// アイテム
    pub value: Vec<Expression>,
}

pub struct Identifier {
    /// 変数名
    pub name: String,
}

pub struct Call {
    /// 対象
    pub target: Box<Expression>,

    /// 引数
    pub args: Vec<Expression>,
}

pub struct Index {
    /// 対象
    pub target: Box<Expression>,

    /// インデックス
    pub name: Box<Expression>,
}

pub struct Prop {
    /// 対象
    pub target: Box<Expression>,

    /// プロパティ名
    pub name: String,
}

pub enum TypeSource {
    /// 名前付き型
    NamedTypeSource(NamedTypeSource),

    /// 関数の型
    FnTypeSource(FnTypeSource),
}

pub struct NamedTypeSource {
    /// 型名
    pub name: String,

    /// 内側の型
    pub inner: Option<Box<TypeSource>>,
}

pub struct FnTypeSource {
    /// 引数の型
    pub args: Vec<TypeSource>,

    /// 戻り値の型
    pub result: Box<TypeSource>,
}
