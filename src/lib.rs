#![no_std]
extern crate alloc;
pub mod document;
pub mod index;
pub mod jaro;
pub mod query;
pub mod tokenizer;
pub mod utils;
use alloc::{borrow::ToOwned, string::String, vec::Vec};
use document::Document;
use index::Index;
use query::Query;
use tokenizer::Tokenizer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WebScout {
    index: Index,
    tokenizer: Tokenizer,
}
#[wasm_bindgen]
impl WebScout {
    #[wasm_bindgen(constructor)]
    pub fn new(lang: String) -> WebScout {
        WebScout {
            index: Index::new(),
            tokenizer: Tokenizer::new(&lang),
        }
    }
    #[wasm_bindgen]
    pub fn document(&mut self, title: String, mut body: String) -> Document {
        Document::new(&title, &mut body, &mut self.tokenizer)
    }
    pub fn new_index(&mut self) {
        self.index = Index::new();
    }
    #[wasm_bindgen]
    pub fn add_document(&mut self, document: &mut Document) {
        self.index.add_document(document);
    }
    #[wasm_bindgen]
    pub fn search(&mut self, query: String) -> JsValue {
        let mut query = Query::new(&query, &self.index, &mut self.tokenizer);
        query.search();
        let res = query.all();
        let json = serde_json::to_string(&res).unwrap();
        JsValue::from_str(&json)
    }
    pub fn tokenize(&mut self, token: String) -> JsValue {
        let value = self.tokenizer.tokenize(&token).unwrap_or_default();
        JsValue::from(value)
    }
    #[wasm_bindgen]
    pub fn deserialize_index(&mut self, input: Vec<u8>) {
        self.index = rmp_serde::decode::from_slice(&input).unwrap();
    }
    #[wasm_bindgen]
    pub fn deserialize_tokenizer(&mut self, input: Vec<u8>) {
        self.tokenizer = Tokenizer::from_pack(&input);
    }
    #[wasm_bindgen]
    pub fn greet(&self) -> String {
        "hello".to_owned()
    }
}
