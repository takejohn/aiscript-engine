//! ASTノード

use std::collections::HashMap;

use derive_node::NodeBase;
use derive_wrapper::Wrapper;
use serde::{de::Visitor, ser::SerializeMap, Deserialize, Serialize};

use crate::string::{Utf16Str, Utf16String};

pub use crate::common::Position;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Loc {
    pub start: Position,
    pub end: Position,
}

pub trait NodeBase {
    /// コード位置
    fn loc(&self) -> &Loc;
}

/// 予約語でない名前を持つノード。
pub trait NamedNode: NodeBase {
    fn name(&self) -> &Utf16Str;
}

#[derive(Debug, PartialEq, Eq, NodeBase, Wrapper, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Node {
    /// 名前空間
    Ns(Namespace),

    /// メタデータ定義
    Meta(Meta),

    TypeSource(TypeSource),
    Attr(Attribute),

    #[serde(untagged)]
    Statement(Statement),

    #[serde(untagged)]
    Expr(Expression),
}

impl From<StatementOrExpression> for Node {
    fn from(value: StatementOrExpression) -> Self {
        match value {
            StatementOrExpression::Statement(statement) => statement.into(),
            StatementOrExpression::Expression(expression) => expression.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, NodeBase, Wrapper, Serialize, Deserialize)]
#[serde(untagged)]
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

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq, NodeBase, Wrapper, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum NamespaceMember {
    Ns(Namespace),

    Def(Definition),
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Meta {
    pub loc: Loc,

    /// 名
    pub name: Option<Utf16String>,

    /// 値
    pub value: Expression,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Wrapper, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
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
    #[serde(untagged)]
    Assign(Assign),
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Definition {
    pub loc: Loc,

    /// 宣言式
    pub dest: Expression,

    /// 変数の型
    #[serde(rename = "type")]
    pub var_type: Option<TypeSource>,

    /// 式
    pub expr: Expression,

    /// ミュータブルか否か
    #[serde(rename = "mut")]
    pub is_mut: bool,

    /// 付加された属性
    pub attr: Vec<Attribute>,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Return {
    pub loc: Loc,

    /// 式
    pub expr: Expression,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Each {
    pub loc: Loc,

    /// each文
    pub var: Expression,

    /// 配列
    pub items: Expression,

    /// 本体処理
    pub for_statement: Box<StatementOrExpression>,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct For {
    pub loc: Loc,

    pub iter: ForIterator,

    /// 本体処理
    pub for_statement: Box<StatementOrExpression>,
}

// TODO: json変換
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Loop {
    pub loc: Loc,

    /// 処理
    pub statements: Vec<StatementOrExpression>,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Break {
    pub loc: Loc,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Continue {
    pub loc: Loc,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Assign {
    pub loc: Loc,

    #[serde(rename = "type")]
    pub op: AssignOperator,

    /// 代入先
    pub dest: Expression,

    /// 式
    pub expr: Expression,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AssignOperator {
    /// 再代入文
    Assign,

    /// 加算代入文
    AddAssign,

    /// 減算代入文
    SubAssign,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Wrapper, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
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

    /// 変数などの識別子
    Identifier(Identifier),

    /// 関数呼び出し
    Call(Call),

    /// 配列要素アクセス
    Index(Index),

    /// プロパティアクセス
    Prop(Prop),

    /// 二項演算
    #[serde(untagged)]
    Binary(Binary),
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Binary {
    pub loc: Loc,

    #[serde(rename = "type")]
    pub op: BinaryOperator,

    pub left: Box<Expression>,
    pub right: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Not {
    pub loc: Loc,

    /// 式
    pub expr: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Elseif {
    /// elifの条件式
    pub cond: Expression,

    /// elif節
    pub then: StatementOrExpression,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Fn {
    pub loc: Loc,

    pub args: Vec<FnArg>,

    /// 戻り値の型
    #[serde(rename = "retType")]
    pub ret_type: Option<TypeSource>,

    /// 本体処理
    pub children: Vec<StatementOrExpression>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct FnArg {
    /// 引数名
    pub dest: Expression,

    /// 実引数が省略された場合の挙動
    #[serde(flatten)]
    pub value: FnArgValue,

    /// 引数の型
    #[serde(rename = "argType")]
    pub arg_type: Option<TypeSource>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FnArgValue {
    Optional,

    Required {
        /// 引数の初期値
        default: Option<Expression>,
    },
}

// booleanのtagが実装されたら自力実装しなくて済むはずだけど……
// https://github.com/serde-rs/serde/issues/745#issuecomment-294314786
//
// #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
// #[serde(tag = "optional")]
// pub enum FnArgValue {
//     #[serde(rename = "true")]
//     Optional,
//
//     #[serde(rename = "false")]
//     Required {
//         /// 引数の初期値
//         default: Option<Expression>,
//     },
// }
//
// 以下FnArgValueのSerialize, Deserialize実装

impl Serialize for FnArgValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            FnArgValue::Optional => {
                let mut result = serializer.serialize_map(Some(1))?;
                result.serialize_entry("optional", &true)?;
                return result.end();
            }
            FnArgValue::Required { default: None } => {
                let mut result = serializer.serialize_map(Some(1))?;
                result.serialize_entry("optional", &false)?;
                return result.end();
            }
            FnArgValue::Required {
                default: Some(default),
            } => {
                let mut result = serializer.serialize_map(Some(2))?;
                result.serialize_entry("optional", &false)?;
                result.serialize_entry("default", default)?;
                return result.end();
            }
        }
    }
}

enum FnArgValueKey {
    Optional,
    Default,
}

struct FnArgValueKeyVisitor;

impl<'de> Visitor<'de> for FnArgValueKeyVisitor {
    type Value = FnArgValueKey;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("\"optional\" or \"default\"")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "optional" => Ok(FnArgValueKey::Optional),
            "default" => Ok(FnArgValueKey::Default),
            _ => Err(serde::de::Error::unknown_field(v, &["optional", "default"])),
        }
    }
}

impl<'de> Deserialize<'de> for FnArgValueKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(FnArgValueKeyVisitor)
    }
}

struct FnArgValueVisitor;

impl<'de> Visitor<'de> for FnArgValueVisitor {
    type Value = FnArgValue;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("FnArgValue")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let mut optional: Option<bool> = None;
        let mut default: Option<Expression> = None;
        let mut received_default = false;

        while let Some(key) = map.next_key::<FnArgValueKey>()? {
            match key {
                FnArgValueKey::Optional => {
                    if optional.is_some() {
                        return Err(serde::de::Error::duplicate_field("optional"));
                    }
                    optional = Some(map.next_value::<bool>()?);
                }
                FnArgValueKey::Default => {
                    if received_default {
                        return Err(serde::de::Error::duplicate_field("default"));
                    }
                    default = map.next_value::<Option<Expression>>()?;
                    received_default = true;
                }
            }
        }

        let Some(optional) = optional else {
            return Err(serde::de::Error::missing_field("optional"));
        };

        if optional {
            if default.is_some() {
                return Err(serde::de::Error::custom(
                    "unexpected \"default\" value when \"optional\" is true",
                ));
            }
            return Ok(FnArgValue::Optional);
        } else {
            return Ok(FnArgValue::Required { default });
        }
    }
}

impl<'de> Deserialize<'de> for FnArgValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_map(FnArgValueVisitor)
    }
}

// 以上FnArgValueのSerialize, Deserialize実装

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Match {
    pub loc: Loc,

    /// 対象
    pub about: Box<Expression>,

    pub qs: Vec<MatchQ>,

    /// デフォルト値
    pub default: Option<Box<StatementOrExpression>>,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MatchQ {
    /// 条件
    pub q: Expression,

    /// 結果
    pub a: StatementOrExpression,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Block {
    pub loc: Loc,

    /// 処理
    pub statements: Vec<StatementOrExpression>,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Exists {
    pub loc: Loc,

    /// 変数名
    pub identifier: Identifier,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Tmpl {
    pub loc: Loc,

    /// 処理
    pub tmpl: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Str {
    pub loc: Loc,

    /// 文字列
    pub value: Utf16String,
}

#[derive(Debug, NodeBase, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Bool {
    pub loc: Loc,

    /// 真理値
    pub value: bool,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Null {
    pub loc: Loc,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Obj {
    pub loc: Loc,

    /// オブジェクト
    pub value: HashMap<Utf16String, Expression>,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Arr {
    pub loc: Loc,

    /// アイテム
    pub value: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Call {
    pub loc: Loc,

    /// 対象
    pub target: Box<Expression>,

    /// 引数
    pub args: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct Index {
    pub loc: Loc,

    /// 対象
    pub target: Box<Expression>,

    /// インデックス
    pub index: Box<Expression>,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
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

#[derive(Debug, PartialEq, Eq, NodeBase, Wrapper, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum TypeSource {
    /// 名前付き型
    #[serde(rename = "namedTypeSource")]
    NamedTypeSource(NamedTypeSource),

    /// 関数の型
    #[serde(rename = "fnTypeSource")]
    FnTypeSource(FnTypeSource),
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct NamedTypeSource {
    pub loc: Loc,

    /// 型名
    pub name: Utf16String,

    /// 内側の型
    pub inner: Option<Box<TypeSource>>,
}

#[derive(Debug, PartialEq, Eq, NodeBase, Serialize, Deserialize)]
pub struct FnTypeSource {
    pub loc: Loc,

    /// 引数の型
    pub args: Vec<TypeSource>,

    /// 戻り値の型
    pub result: Box<TypeSource>,
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    fn serialize<T: Serialize>(value: &T, expected: &str) {
        assert_eq!(serde_json::to_string(value).unwrap(), expected);
    }

    fn deserialize<'a, T: std::fmt::Debug + Eq + Deserialize<'a>>(s: &'a str, expected: &T) {
        assert_eq!(&serde_json::from_str::<T>(s).unwrap(), expected);
    }

    fn deserialize_fails<'a, T: Deserialize<'a>>(s: &'a str) {
        assert!(serde_json::from_str::<T>(s).is_err());
    }

    mod fn_arg_value {
        use super::*;

        #[test]
        fn test_serialize() {
            serialize(&FnArgValue::Optional, r#"{"optional":true}"#);
            serialize(
                &FnArgValue::Required { default: None },
                r#"{"optional":false}"#,
            );
            serialize(
                &FnArgValue::Required {
                    default: Some(
                        Null {
                            loc: Loc {
                                start: Position::At { line: 1, column: 8 },
                                end: Position::At { line: 1, column: 9 },
                            },
                        }
                        .into(),
                    ),
                },
                r#"{"optional":false,"default":{"type":"null","loc":{"start":{"line":1,"column":8},"end":{"line":1,"column":9}}}}"#,
            );
        }

        #[test]
        fn test_deserialize() {
            deserialize(r#"{"optional":true}"#, &FnArgValue::Optional);
            deserialize(
                r#"{"optional":false}"#,
                &FnArgValue::Required { default: None },
            );
            deserialize(
                r#"{"optional":false,"default":{"type":"null","loc":{"start":{"line":1,"column":8},"end":{"line":1,"column":9}}}}"#,
                &FnArgValue::Required {
                    default: Some(
                        Null {
                            loc: Loc {
                                start: Position::At { line: 1, column: 8 },
                                end: Position::At { line: 1, column: 9 },
                            },
                        }
                        .into(),
                    ),
                },
            );

            deserialize_fails::<FnArgValue>(r#"{}"#);
            deserialize_fails::<FnArgValue>(r#"{"default":{"type":"null","loc":{"start":{"line":1,"column":8},"end":{"line":1,"column":9}}}}"#);
            deserialize_fails::<FnArgValue>(r#"{"optional":true,"optional":true}"#);
            deserialize_fails::<FnArgValue>(r#"{"default":{"type":"null","loc":{"start":{"line":1,"column":8},"end":{"line":1,"column":9}}},"default":{"type":"null","loc":{"start":{"line":1,"column":8},"end":{"line":1,"column":9}}}}"#);
            deserialize_fails::<FnArgValue>(r#"{"optional":true,"default":{"type":"null","loc":{"start":{"line":1,"column":8},"end":{"line":1,"column":9}}}}"#);
        }
    }
}
