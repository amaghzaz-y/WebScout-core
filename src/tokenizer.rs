use crate::types::{Document, Lemmer};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
};
#[derive(Serialize, Deserialize)]
pub struct Tokenizer {
    lang: String,
    tokens: HashMap<String, String>,
}
impl Tokenizer {
    pub fn new(lang: String) -> Tokenizer {
        Tokenizer {
            lang: lang,
            tokens: HashMap::new(),
        }
    }
    pub fn get(lang: String) -> Tokenizer {
        let path = format!("output/packs/{:?}.pack", lang.to_lowercase());
        let bin = fs::read(path).unwrap();
        let tokenizer: Tokenizer = rmp_serde::from_slice(&bin).unwrap();
        return tokenizer;
    }
    pub fn transform_token(&self, word: &String) -> String {
        let mut lemma: String = String::new();
        let value = self.tokens.get(word);
        if value.is_some() {
            lemma = value.unwrap().to_owned();
        }
        return lemma;
    }
    pub fn tokenize_map(
        &self,
        mut index: &HashMap<String, HashSet<usize>>,
    ) -> HashMap<String, HashSet<usize>> {
        let mut map: HashMap<String, HashSet<usize>> = HashMap::new();
        for (token, positions) in index {
            let lemma = self.transform_token(&token);
            map.insert(lemma, positions.to_owned());
        }
        return map;
    }
    pub fn from_text(mut self, text: String, lang: String) {
        let mut map: HashMap<String, String> = HashMap::new();
        for mut line in text.lines() {
            let lemma: Vec<&str> = line.split_whitespace().collect();
            if lemma.len() > 1 {
                map.insert(lemma[1].to_owned(), lemma[0].to_owned());
            }
        }
        self = Tokenizer {
            lang: lang,
            tokens: map,
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let bin = rmp_serde::encode::to_vec(&self).unwrap();
        return bin;
    }
}
