#![no_std]
extern crate alloc;
pub mod document;
pub mod index;
pub mod jaro;
pub mod query;
pub mod tokenizer;
pub mod utils;
use alloc::{string::String, vec::Vec};
use document::Document;
use index::Index;
use query::Query;
use tokenizer::Tokenizer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}
#[wasm_bindgen]
pub struct WebScout {
    index: Index,
    tokenizer: Tokenizer,
}
#[wasm_bindgen]
impl WebScout {
    pub fn document(&mut self, title: String, mut body: String) -> Document {
        Document::new(&title, &mut body, &mut self.tokenizer)
    }
    pub fn add_document(&mut self, document: &mut Document) {
        self.index.add_document(document);
    }
    pub fn search(&mut self, query: String) -> JsValue {
        let mut query = Query::new(&query, &self.index, &mut self.tokenizer);
        query.search();
        let res = query.all();
        let json = serde_json::to_string(&res).unwrap();
        JsValue::from_str(&json)
    }
    pub fn deserialize_index(&mut self, input: Vec<u8>) {
        self.index = rmp_serde::decode::from_slice(&input).unwrap();
    }
    pub fn deserialize_tokenizer(&mut self, input: Vec<u8>) {
        self.tokenizer = Tokenizer::from_pack(&input);
    }
}
