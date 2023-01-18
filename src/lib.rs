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
    documents: HashMap<u32, String>, // 1st value: document name, 2nd value: document id
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
    pub fn parse_body(&self, document: &mut Document) -> HashMap<String, u32> {
        let mut tokens: HashMap<String, u32> = HashMap::default();
        let mut word: Vec<u8> = vec![];
        document.body.push('/'); // to mark the end of document
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
            .insert(hash(document.title.as_bytes()), document.title.to_owned());
    }
    pub fn query(&self, tokens: &Vec<String>) -> Vec<IndexedToken> {
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
    fn raw_to_vec(&self, query: &mut String) -> Vec<String> {
        query.push('/');
        let mut word: Vec<u8> = vec![];
        let mut tokens: Vec<String> = vec![];
        let query = query.as_bytes().to_vec();
        for char in query {
            if char.is_ascii_alphanumeric() {
                word.push(char);
            } else {
                if word.len() > 1 {
                    let mut fword = unsafe { String::from_utf8(word.to_owned()).unwrap() };
                    fword.make_ascii_lowercase();
                    tokens.push(fword);
                }
                word.clear();
            }
        }
        return tokens;
    }
    pub fn tokenize_search(
        &self,
        search: Vec<String>,
        lemmer: &HashMap<String, String>,
    ) -> Vec<String> {
        let mut tokens: Vec<String> = vec![];
        for mut key in search {
            if lemmer.contains_key(&key) {
                let lemma = lemmer.get(&key).unwrap().to_owned();
                tokens.push(lemma);
            } else {
                tokens.push(key);
            }
        }
        tokens.sort();
        tokens.dedup();
        return tokens;
    }
    fn evalute_query(&self, tokens: &Vec<IndexedToken>) {
        let mut documents: HashMap<u32, HashSet<(String, u32, u32)>> = HashMap::default();
        for token in tokens {
            for doc in &token.index.spots {
                if documents.contains_key(doc.0) {
                    documents.entry(doc.0.to_owned()).or_default().insert((
                        token.token.value.to_owned(),
                        doc.1.to_owned(),
                        token.index.freq.to_owned(),
                    ));
                } else {
                    documents.insert(
                        doc.0.to_owned(),
                        HashSet::from([(
                            token.token.value.to_owned(),
                            doc.1.to_owned(),
                            token.index.freq.to_owned(),
                        )]),
                    );
                }
            }
        }
        let mut scores: Vec<(u32, f32)> = vec![];
        for doc in &documents {
            let mut word_freq_ratio: f32 = 0.0;
            for token in doc.1 {
                word_freq_ratio += token.1 as f32 / token.2 as f32;
                println!(
                    "freq: {:?}, frqt: {:?} wfr: {:?}",
                    token.1, token.2, word_freq_ratio
                );
            }
            let query_ratio: f32 = (doc.1.len() as f32 / tokens.len() as f32);
            let total_word_freq_ratio = word_freq_ratio / (tokens.len() as f32);
            let score = query_ratio * total_word_freq_ratio;
            println!(
                "qr: {:?}, twfr: {:?}, score: {:?}",
                query_ratio, total_word_freq_ratio, score
            );
            scores.push((doc.0.to_owned(), (score * 100.0)));
            println!();
        }
        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        println!("{:?}", scores);
    }
    pub fn search(&self, search: &'static str, lemmer: &HashMap<String, String>) {
        let mut tokens = self.raw_to_vec(&mut search.to_string());
        tokens = self.tokenize_search(tokens, lemmer);
        let r = self.query(&tokens);
        self.evalute_query(&r);
    }
    pub fn from_binary(data: Vec<u8>) -> WebScout {
        let ws: WebScout = bincode::deserialize(&data).unwrap();
        return ws;
    }
}
