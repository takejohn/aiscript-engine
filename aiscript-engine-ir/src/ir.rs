use aiscript_engine_common::{AiScriptBasicError, Utf16String};

/// 中間表現
#[derive(Debug, PartialEq)]
pub struct Ir {
    pub data: Vec<DataItem>,
    pub functions: Vec<Block>,
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

    pub(crate) fn new_register(&mut self) -> Register {
        let index = self.register_length;
        self.register_length += 1;
        return index;
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

    /// nullを格納
    Null(Register),

    /// numを格納
    Num(Register, f64),

    /// boolを格納
    Bool(Register, bool),

    /// [`DataItem`]の参照を格納
    Data(Register, DataIndex),

    /// レジスタ0にレジスタ1の値をコピー
    Move(Register, Register),

    /// レジスタ0にレジスタ1の値を加える
    Add(Register, Register),

    /// レジスタ0からレジスタ1の値を減じる
    Sub(Register, Register),
}
