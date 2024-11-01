use aiscript_engine_common::{AiScriptBasicError, Utf16String};

/// 中間表現
#[derive(Debug, PartialEq)]
pub struct Ir {
    pub data: Vec<DataItem>,
    pub functions: Vec<Procedure>,
    pub entry_point: FnIndex,
}

impl Default for Ir {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            functions: vec![Procedure::new()],
            entry_point: 0,
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
    User(Procedure),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Procedure {
    pub register_length: usize,
    pub instructions: Vec<Instruction>,
}

impl Procedure {
    pub fn new() -> Self {
        Procedure {
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
    LoadImmediate(Register, Register, usize),

    /// レジスタ0からレジスタ1[即値2]にコピー
    StoreImmediate(Register, Register, usize),
}
