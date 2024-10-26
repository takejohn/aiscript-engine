use aiscript_engine_common::{AiScriptBasicError, Utf16String};

/// スタックマシン?の中間表現
#[derive(Debug, PartialEq)]
pub struct Ir {
    pub data: Vec<DataItem>,
    pub functions: Vec<IrFn>,
    pub entry_point: Block,
}

impl Default for Ir {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            functions: Vec::new(),
            entry_point: Block::new(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DataItem {
    Str(Utf16String),
}

#[derive(Debug, PartialEq)]
pub struct IrFn {
    pub code: IrFnCode,
}

#[derive(Debug, PartialEq)]
pub enum IrFnCode {
    User(Block),
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub register_length: usize,
    pub instructions: Vec<Instruction>,
}

impl Block {
    pub fn new() -> Self {
        Block {
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

#[derive(Debug, PartialEq)]
pub enum Instruction {
    /// 何もしない
    Nop,

    /// エラーによる強制終了
    Panic(AiScriptBasicError),

    /// スタックトップを削除
    /// ```text
    /// [..., x] -> [...]
    /// ```
    Drop,

    /// スタックトップを複製
    /// ```text
    /// [..., x] -> [..., x, x]
    /// ```
    Dup,

    /// 引数を指定
    /// ```text
    /// [..., arg] -> [...]
    /// ```
    ArgSet,

    /// 引数を取得
    /// ```text
    /// [...] -> [..., arg]
    /// ```
    ArgGet,

    /// 関数呼び出し
    /// ```text
    /// [..., f] -> [..., result]
    /// ```
    Call,

    /// falseなら分岐
    /// ```text
    /// [..., cond] -> [...]
    /// ```
    Branch(InstructionAddress),

    /// trueなら分岐
    /// ```text
    /// [..., cond] -> [...]
    /// ```
    BranchNot(InstructionAddress),

    /// レジスタ1からレジスタ0への値のコピー
    Move(Register, Register),

    /// 加算
    /// ```text
    /// [..., a, b] -> [..., a + b]
    /// ```
    Add,

    /// 減算
    /// ```text
    /// [..., a, b] -> [..., a - b]
    /// ```
    Sub,

    /// 論理積
    /// ```text
    /// [..., a, b] -> [..., a && b]
    /// ```
    And,

    /// 論理和
    /// ```text
    /// [..., a, b] -> [..., a || b]
    /// ```
    Or,

    /// null値をプッシュ
    /// ```text
    /// [...] -> [..., null]
    /// ```
    Null,

    /// 真理値をプッシュ
    /// ```text
    /// [...] -> [..., b]
    /// ```
    Bool(bool),

    /// 数値をプッシュ
    /// ```text
    /// [...] -> [..., n]
    /// ```
    Num(f64),

    /// 関数(クロージャ)をプッシュ
    /// ```text
    /// [...] -> [..., n]
    /// ```
    Fn(FnIndex),

    /// dataから値をコピー
    /// ```text
    /// [...] -> [..., x]
    /// ```
    Load(DataIndex),

    /// arrを初期化
    /// ```text
    /// [...] -> [..., a]
    /// ```
    Arr,

    /// objを初期化
    /// ```text
    /// [...] -> [..., o]
    /// ```
    Obj,

    /// objの値を取得
    /// ```text
    /// [..., o] -> [..., o.prop]
    /// ```
    Prop(DataIndex),

    /// arrやobjの値を取得
    /// ```text
    /// [..., o, i] -> [..., o[i]]
    /// ```
    Index,

    /// 論理否定
    /// ```text
    /// [..., b] -> [..., !b]
    /// ```
    Not,

    /// return値
    /// ```text
    /// [..., x] -> [..., return x]
    /// ```
    Return,

    /// break値
    /// ```text
    /// [...] -> [..., break]
    /// ```
    Break,

    /// continue値
    /// ```text
    /// [...] -> [..., continue]
    /// ```
    Continue,
}
