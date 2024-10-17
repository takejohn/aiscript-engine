use aiscript_engine_common::Utf16String;

pub struct Ir {
    pub data: Vec<DataItem>,
    pub functions: Vec<IrFn>,
}

pub enum DataItem {
    Str(Utf16String),
}

pub enum IrFn {
    User {
        instructions: Vec<Instruction>,
    },
}

pub type DataIndex = usize;

pub type FnIndex = usize;

pub type Register = usize;

pub type Argument = usize;

pub type InstructionAddress = usize;

pub enum Instruction {
    /// 何もしない
    Nop,

    /// 引数を指定
    Arg(Argument, Register),

    /// 関数呼び出し
    Call(FnIndex),

    /// trueなら分岐
    Branch(Register, InstructionAddress),

    /// falseなら分岐
    BranchNot(Register, InstructionAddress),

    /// レジスタ1からレジスタ0への値のコピー
    Move(Register, Register),

    /// 加算: レジスタ0 <- レジスタ1 + レジスタ2
    Add(Register, Register, Register),

    /// 減算
    Sub(Register, Register, Register),

    /// null値を格納
    Null(Register),

    /// 真理値を格納
    Bool(Register, bool),

    /// 数値を格納
    Num(Register, f64),

    /// dataから値をコピー
    Load(Register, DataIndex),

    /// 容量を指定してarrを初期化
    Arr(Register, usize),

    /// 容量を指定してobjを初期化
    Obj(Register, usize),

    /// レジスタ0[レジスタ1] = レジスタ2
    Insert(Register, Register, Register),

    /// レジスタ0 = レジスタ1[レジスタ2]
    Get(Register, Register, Register),

    /// 論理否定: レジスタ0 = !レジスタ1
    Not(Register, Register),

    /// return型の値を格納: レジスタ0 = return レジスタ1
    Return(Register, Register),

    /// break値を格納
    Break(Register),

    /// continue値を格納
    Continue(Register),

    /// 論理積: レジスタ0 = レジスタ1 && レジスタ2
    And(Register, Register, Register),

    /// 論理和: レジスタ0 = レジスタ1 && レジスタ2
    Or(Register, Register, Register),
}
