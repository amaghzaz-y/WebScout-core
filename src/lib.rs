// #![no_std]
extern crate alloc;
pub mod document;
pub mod index;
pub mod jaro;
pub mod query;
pub mod tokenizer;
pub mod utils;
use alloc::{string::String, vec::Vec};
use index::Index;
use query::Query;
use tokenizer::Tokenizer;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct WebScout {
    index: Index,
    tokenizer: Tokenizer,
    query: Query,
}
#[wasm_bindgen]
impl WebScout {
    #[wasm_bindgen(constructor)]
    pub fn new(lang: String) -> WebScout {
        WebScout {
            index: Index::new(),
            tokenizer: Tokenizer::new(&lang),
            query: Query::default(),
        }
    }
    #[wasm_bindgen]
    pub fn setup(&mut self) {
        self.query.setup(&self.index, &self.tokenizer)
    }

    #[wasm_bindgen]
    pub fn index(&mut self, title: String, mut body: String) {
        let doc = document::Document::new(&title, &mut body, &mut self.tokenizer);
        self.index.add_document(&doc);
    }

    #[wasm_bindgen]
    // return a JSON with DocumentName : Score
    pub fn search_all(&mut self, search: String) -> JsValue {
        let mut query = Query::new(&self.index, &self.tokenizer);
        let res = query.search(&search);
        let res = query.all(res.0);
        let json = serde_json::to_string(&res).unwrap();
        JsValue::from_str(&json)
    }

    #[wasm_bindgen]
    pub fn search_above_average(&mut self, search: String) -> JsValue {
        let mut query = Query::new(&self.index, &self.tokenizer);
        let res = query.search(&search);
        let res = query.above_average(res.0, res.1);
        let json = serde_json::to_string(&res).unwrap();
        JsValue::from_str(&json)
    }

    #[wasm_bindgen]
    pub fn tokenize(&mut self, token: String) -> String {
        self.tokenizer.tokenize(&token).unwrap_or_default()
    }

    #[wasm_bindgen]
    pub fn export_index(&self) -> Box<[u8]> {
        self.index.serialize().into()
    }

    #[wasm_bindgen]
    pub fn deserialize_index(&mut self, input: Vec<u8>) {
        self.index = rmp_serde::decode::from_slice(&input).unwrap();
    }

    #[wasm_bindgen]
    pub fn deserialize_tokenizer(&mut self, input: Vec<u8>) {
        self.tokenizer = Tokenizer::from_pack(&input);
    }
}
