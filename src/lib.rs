#![allow(unused)]
#![allow(dead_code)]
#![allow(unstable_features)]
use crc32fast::hash;
use serde::{Deserialize, Serialize, __private::doc};
use std::{
    borrow::Borrow,
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
#[derive(PartialEq, Eq, Debug, Hash, Serialize, Deserialize, Default, Clone)]

struct Token {
    value: String,
}

#[derive(Default, Debug)]
pub struct IndexedToken {
    token: Token,
    index: Index,
}
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Default, Clone)]

struct Index {
    freq: u32,
    spots: HashMap<u32, u32>, // 1st value: document id, 2nd value: freq
}
#[derive(PartialEq, Eq, Debug, Serialize, Deserialize)]

struct Store {
    dict: HashMap<Token, Index>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WebScout {
    title: String,
    documents: HashMap<String, u32>, // 1st value: document name, 2nd value: document id
    store: Store,
}
impl WebScout {
    pub fn new() -> Self {
        WebScout {
            title: "WebScout LLC".to_string(),
            documents: HashMap::new(),
            store: Store {
                dict: HashMap::new(),
            },
        }
    }
    pub fn parse_body(&self, document: &Document) -> HashMap<String, u32> {
        let mut tokens: HashMap<String, u32> = HashMap::default();
        let mut word: Vec<u8> = vec![];
        let bin_body = document.body.as_bytes();
        for byte in bin_body.to_vec() {
            if byte.is_ascii_alphanumeric() {
                word.push(byte);
            } else {
                if word.len() > 1 {
                    let mut fword = unsafe { String::from_utf8_unchecked(word.to_owned()) };
                    fword.make_ascii_lowercase();
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
    pub fn tokenize(&self, lemmer: &HashMap<String, String>, tokens: &mut HashMap<String, u32>) {
        for token in tokens.clone() {
            if lemmer.contains_key(&token.0) {
                let lemma = lemmer.get(&token.0).unwrap();
                let prev_token = tokens.remove_entry(&token.0).unwrap();
                tokens.insert(lemma.to_owned(), prev_token.1);
            }
        }
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
    pub fn get_tokens(self, tokens: &Vec<String>) -> Vec<IndexedToken> {
        let mut result: Vec<IndexedToken> = vec![];
        for token in tokens {
            let key = &Token {
                value: token.to_owned(),
            };
            if self.store.dict.contains_key(key) {
                let pair = self.store.dict.get_key_value(key).unwrap();
                let idxtoken = IndexedToken {
                    token: pair.0.to_owned(),
                    index: pair.1.to_owned(),
                };
                result.push(idxtoken)
            }
        }
        return result;
    }
    pub fn from_binary(data: Vec<u8>) -> WebScout {
        let ws: WebScout = bincode::deserialize(&data).unwrap();
        return ws;
    }
}
