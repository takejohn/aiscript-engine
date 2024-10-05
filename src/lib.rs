use wasm_bindgen::prelude::wasm_bindgen;

pub mod ast;
pub mod common;
pub mod error;
pub mod parser;
pub mod string;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
