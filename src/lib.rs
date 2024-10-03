use wasm_bindgen::prelude::wasm_bindgen;

pub mod ast;
mod common;
pub mod error;
mod parser;

pub use parser::Parser;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
