#![allow(unused)]
#![allow(dead_code)]
#![allow(unstable_features)]
use crc32fast::hash;
use std::{
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    hash::Hash,
    ops::Add,
    str, vec,
};

use serde::__private::doc;

pub struct Document {
    pub title: String,
    pub body: String,
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
pub struct WebScout {
    documents: HashMap<String, u32>, // 1st value: document name, 2nd value: document id
    store: Store,
}
impl WebScout {
    pub fn new() -> Self {
        WebScout {
            documents: HashMap::new(),
            store: Store {
                docs_keys: HashSet::new(),
                dict: HashMap::new(),
            },
        }
    }
    pub fn parse_body(&self, document: &mut Document) -> HashMap<String, u32> {
        let mut tokens: HashMap<String, u32> = HashMap::default();
        let bin_body = document.body.as_bytes().to_owned();
        let mut words: Vec<String> = vec![];
        let mut word: Vec<u8> = vec![];
        for byte in bin_body {
            if byte.is_ascii_alphanumeric() {
                word.push(byte.to_ascii_lowercase());
            } else {
                if word.len() > 1 {
                    let fword = String::from_utf8(word.to_owned()).unwrap();
                    words.push(fword.to_owned());
                    if tokens.contains_key(&fword) {
                        tokens.entry(fword).and_modify(|e| *e += 1);
                    } else {
                        tokens.insert(fword, 1);
                    }
                }
                word.clear();
            }
        }
        return tokens;
    }
    fn add_document(&mut self, document: &mut Document) {
        self.documents
            .insert(document.title.to_owned(), hash(document.title.as_bytes()));
    }
    fn export_store() {
        todo!();
    }
    fn read_store() {
        todo!()
    }
}
