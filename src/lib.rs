#![allow(unused)]
#![allow(dead_code)]
#![allow(unstable_features)]
use crc32fast::hash;
use serde::__private::doc;
use serde::{Deserialize, Serialize};
use std::{
    collections::{
        hash_map::{self, DefaultHasher},
        HashMap, HashSet,
    },
    hash::Hash,
    ops::Add,
    str, vec,
};
#[derive(PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]
pub struct Document {
    pub title: String,
    pub body: String,
}
#[derive(PartialEq, Eq, Debug, Hash, Serialize, Deserialize)]

struct Token {
    value: String,
}
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]

struct Index {
    freq: u32,
    spots: HashMap<u32, u32>, // 1st value: document id, 2nd value: freq
}
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]

struct Store {
    docs_keys: HashSet<u32>,
    dict: HashMap<Token, Index>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WebScout {
    title: &'static str,
    documents: HashMap<String, u32>, // 1st value: document name, 2nd value: document id
    store: Store,
}
impl WebScout {
    pub fn new() -> Self {
        WebScout {
            title: "WebScout LLC",
            documents: HashMap::new(),
            store: Store {
                docs_keys: HashSet::new(),
                dict: HashMap::new(),
            },
        }
    }
    pub fn parse_body(&self, document: &Document) -> HashMap<String, u32> {
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
    pub fn index_tokens(&mut self, tokens: &HashMap<String, u32>, document: &Document) {
        for token in tokens {
            let mut dict = &mut self.store.dict;
            if dict.contains_key(&Token {
                value: token.0.to_string(),
            }) {
                dict.entry(Token {
                    value: token.0.to_string(),
                })
                .and_modify(|f| {
                    f.spots
                        .insert(hash(document.title.as_bytes()), token.1.to_owned());
                });
            } else {
                dict.insert(
                    Token {
                        value: token.0.to_string(),
                    },
                    Index {
                        freq: token.1.to_owned(),
                        spots: HashMap::from([(
                            hash(document.title.as_bytes()),
                            token.1.to_owned(),
                        )]),
                    },
                );
            }
            dict.entry(Token {
                value: token.0.to_string(),
            })
            .and_modify(|f| {
                let mut freq = 0;
                for key in f.spots.clone() {
                    freq += key.1;
                }
                f.freq = freq;
            });
        }
    }
    pub fn add_document(&mut self, document: &Document) {
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
