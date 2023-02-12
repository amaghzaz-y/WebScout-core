use crate::tokenizer::{self, Tokenizer};
use crc32fast::hash;
use serde::{Deserialize, Serialize, __private::doc};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct Document {
    pub id: u32,
    pub lang: String,
    pub index: HashMap<String, HashSet<usize>>,
    pub count: usize,
}
impl Document {
    pub fn new(name: String, body: String, language: String) -> Document {
        let mut document: Document = Document {
            id: hash(name.as_bytes()),
            lang: language,
            index: HashMap::new(),
            count: 0,
        };
        document.index_string(body);
        document.tokenize();
        return document;
    }
    fn index_string(&mut self, mut body: String) {
        let mut chars: Vec<u8> = vec![];
        let mut count: usize = 0;
        body.push('/'); // to mark the end of document
        for char in body.as_bytes() {
            if char.is_ascii_alphanumeric() {
                chars.push(*char);
            } else {
                if chars.len() > 1 {
                    let mut word = String::from_utf8(chars.to_owned()).unwrap();
                    word.make_ascii_lowercase();
                    self.index
                        .entry(word)
                        .or_insert(HashSet::from([count]))
                        .insert(count);
                }
                count += 1;
                chars.clear();
            }
        }
        self.count = count;
    }
    fn tokenize(&mut self) {
        let tokenizer = Tokenizer::get(&self.lang);
        let map = tokenizer.tokenize_map(&self.index);
        self.index = map;
    }
    pub fn serialize(&self) -> Vec<u8> {
        let bin = rmp_serde::encode::to_vec(self).unwrap();
        return bin;
    }
    pub fn to_json(&self) -> String {
        let json = serde_json::to_string_pretty(self).unwrap();
        return json;
    }
}
