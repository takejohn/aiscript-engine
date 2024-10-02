use wasm_bindgen::prelude::wasm_bindgen;

pub mod ast;
mod parser;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
