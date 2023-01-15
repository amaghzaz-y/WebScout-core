#![allow(unused)]
#![allow(dead_code)]
#![allow(unstable_features)]
use std::{
    collections::{HashMap, HashSet},
    vec,
};

struct Document {
    title: String,
    body: String,
}
struct Token {
    value: String,
}
struct Index {
    freq: u32,
    spots: HashMap<u32, u32>, // 1st value: document id, 2nd value: freq
}
struct Store {
    docs_keys: HashSet<u32>,
    dict: HashMap<Token, Index>,
}
struct WebScout {
    documents: HashMap<String, u32>, // 1st value: document name, 2nd value: document id
    store: Store,
}
impl WebScout {
    fn parse_body(document: &mut Document) -> Vec<(String, u32)> {
        let mut tokens: HashMap<String, u32> = HashMap::default();
        todo!();
    }
    fn add_document() {
        todo!();
    }
    fn export_store() {
        todo!();
    }
    fn read_store() {
        todo!()
    }
}
