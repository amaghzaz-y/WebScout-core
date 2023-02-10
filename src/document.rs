use crate::tokenizer::{self, Tokenizer};
use serde::__private::doc;
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
#[derive(Clone)]
struct Document {
    id: String,
    lang: String,
    index: HashMap<String, HashSet<usize>>,
}
impl Document {
    fn new(body: String, language: String) -> Document {
        let mut document: Document = Document {
            id: Uuid::new_v4().to_string(),
            lang: language,
            index: HashMap::new(),
        };
        document.index_string(body);
        document.transform();
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
    }
    fn transform(&mut self) {
        let tokenizer = Tokenizer::get(self.lang.to_owned());
        let map = tokenizer.tokenize_map(&self.index);
        self.index = map;
    }
}
