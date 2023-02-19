#![allow(dead_code, unused)]
pub mod document;
pub mod index;
pub mod jaro;
pub mod query;
pub mod tokenizer;
pub mod utils;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
pub fn greet() {
    alert("Hello, test-wasm!");
}
