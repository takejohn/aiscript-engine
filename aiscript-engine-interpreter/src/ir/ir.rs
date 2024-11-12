use std::{fmt::Debug, rc::Rc};

use crate::library::NativeFn;
use aiscript_engine_common::AiScriptBasicError;
use aiscript_engine_values::{VArr, VObj};
use gc::{Gc, GcCell};

/// 中間表現
#[derive(Debug)]
pub(crate) struct Ir {
    pub native_functions: Vec<NativeFn>,
    pub entry_point: UserFn,
}

impl Default for Ir {
    fn default() -> Self {
        Self {
            native_functions: Vec::new(),
            entry_point: UserFn::new(),
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct UserFn {
    pub register_length: usize,
    pub instructions: Vec<Instruction>,
}

impl UserFn {
    pub(crate) fn new() -> Self {
        UserFn {
            register_length: 0,
            instructions: Vec::new(),
        }
    }
}

pub(crate) type NativeFnIndex = usize;

pub(crate) type UserFnIndex = usize;

pub(crate) type Register = usize;

#[derive(Clone, Debug)]
pub(crate) enum Instruction {
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

    /// strを格納
    Str(Register, Rc<[u16]>),

    /// 指定された長さの未初期化のarrの参照を格納
    Arr(Register, Gc<GcCell<VArr>>),

    /// 指定された初期容量をもつobjの参照を格納
    Obj(Register, Gc<GcCell<VObj>>),

    /// ネイティブ関数のクロージャを格納
    NativeFn(Register, NativeFnIndex),

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
    LoadProp(Register, Register, Rc<[u16]>),

    /// レジスタ0からレジスタ1[レジスタ2]にコピー
    Store(Register, Register, Register),

    /// レジスタ0からレジスタ1[即値2]にコピー
    StoreIndex(Register, Register, usize),

    /// レジスタ0からレジスタ1.即値2にコピー
    StoreProp(Register, Register, Rc<[u16]>),

    /// レジスタ1の関数をレジスタ2の配列の引数で呼び出し、返値をレジスタ0に格納
    Call(Register, Register, Register),
}
