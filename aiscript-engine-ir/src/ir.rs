use std::fmt::Debug;

use aiscript_engine_common::{AiScriptBasicError, Utf16String};
use aiscript_engine_library::NativeFn;

/// 中間表現
#[derive(Debug, PartialEq)]
pub struct Ir<'lib> {
    pub data: Vec<DataItem>,
    pub functions: Vec<Function<'lib>>,
    pub entry_point: FnIndex,
}

impl Default for Ir<'_> {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            functions: vec![Function::User(UserFn::new())],
            entry_point: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DataItem {
    Str(Utf16String),
}

pub enum Function<'lib> {
    User(UserFn),
    Native(&'lib NativeFn),
}

impl Debug for Function<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::User(proc) => Debug::fmt(proc, f),
            Self::Native(_) => write!(f, "<native code>"),
        }
    }
}

impl PartialEq for Function<'_> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::User(a), Self::User(b)) => a == b,
            (Self::Native(a), Self::Native(b)) => std::ptr::eq(a, b),
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UserFn {
    pub register_length: usize,
    pub instructions: Vec<Instruction>,
}

impl UserFn {
    pub fn new() -> Self {
        UserFn {
            register_length: 0,
            instructions: Vec::new(),
        }
    }
}

pub type DataIndex = usize;

pub type FnIndex = usize;

pub type Register = usize;

pub type Argument = usize;

pub type InstructionAddress = usize;

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {
    /// 何もしない
    Nop,

    /// エラーによる強制終了
    Panic(AiScriptBasicError),

    /// レジスタの値が真なら前のコード、偽なら後のコードを実行
    If(Register, Vec<Instruction>, Vec<Instruction>),

    /// nullを格納
    Null(Register),

    /// numを格納
    Num(Register, f64),

    /// boolを格納
    Bool(Register, bool),

    /// [`DataItem`]の参照を格納
    Data(Register, DataIndex),

    /// 指定された長さの未初期化のarrの参照を格納
    Arr(Register, usize),

    /// 指定された初期容量をもつobjの参照を格納
    Obj(Register, usize),

    /// クロージャを格納
    Fn(Register, FnIndex),

    /// レジスタ0にレジスタ1の値をコピー
    Move(Register, Register),

    /// レジスタ0にレジスタ1の値を加える
    Add(Register, Register),

    /// レジスタ0からレジスタ1の値を減じる
    Sub(Register, Register),

    /// レジスタ0にレジスタ1の論理否定を代入
    Not(Register, Register),

    /// レジスタ1[レジスタ2]からレジスタ0にコピー
    Load(Register, Register, Register),

    /// レジスタ1[即値2]からレジスタ0にコピー
    LoadIndex(Register, Register, usize),

    /// レジスタ1.即値2からレジスタ0にコピー
    LoadProp(Register, Register, DataIndex),

    /// レジスタ0からレジスタ1[レジスタ2]にコピー
    Store(Register, Register, Register),

    /// レジスタ0からレジスタ1[即値2]にコピー
    StoreIndex(Register, Register, usize),

    /// レジスタ0からレジスタ1.即値2にコピー
    StoreProp(Register, Register, DataIndex),

    /// レジスタ1の関数をレジスタ2の配列の引数で呼び出し、返値をレジスタ0に格納
    Call(Register, Register, Register),
}
