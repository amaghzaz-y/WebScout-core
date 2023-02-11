use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
};
#[derive(Serialize, Deserialize)]
pub struct Tokenizer {
    pub lang: String,
    pub tokens: HashMap<String, String>,
}
impl Tokenizer {
    pub fn new(lang: &String) -> Tokenizer {
        Tokenizer {
            lang: lang.to_owned(),
            tokens: HashMap::new(),
        }
    }
    pub fn get(lang: &String) -> Tokenizer {
        let path = format!("packs/lang/{:}.pack", lang.to_lowercase());
        let bin = fs::read(path).unwrap();
        let tokenizer: Tokenizer = rmp_serde::from_slice(&bin).unwrap();
        return tokenizer;
    }
    pub fn transform_token(&self, word: &String) -> String {
        let mut lemma: String = word.to_owned();
        let value = self.tokens.get(word);
        if value.is_some() {
            lemma = value.unwrap().to_owned();
        }
        return lemma;
    }
    pub fn tokenize_map(
        &self,
        index: &HashMap<String, HashSet<usize>>,
    ) -> HashMap<String, HashSet<usize>> {
        let mut map: HashMap<String, HashSet<usize>> = HashMap::new();
        for (token, positions) in index {
            let lemma = self.transform_token(&token);
            map.entry(lemma)
                .or_insert(positions.to_owned())
                .union(positions);
        }
        return map;
    }
    pub fn from_text(text: &String, lang: &String) -> Tokenizer {
        let mut map: HashMap<String, String> = HashMap::new();
        for mut line in text.lines() {
            let lemma: Vec<&str> = line.split_whitespace().collect();
            if lemma.len() > 1 {
                map.insert(lemma[1].to_owned(), lemma[0].to_owned());
            }
        }
        return Tokenizer {
            lang: lang.to_owned(),
            tokens: map,
        };
    }
    pub fn serialize(&self) -> Vec<u8> {
        let bin = rmp_serde::encode::to_vec(&self).unwrap();
        return bin;
    }
    pub fn deserialize(bin: Vec<u8>) -> Tokenizer {
        let tokenizer: Tokenizer = rmp_serde::from_slice(&bin).unwrap();
        return tokenizer;
    }
    pub fn to_json(&self) -> String {
        let json = serde_json::to_string_pretty(&self).unwrap();
        return json;
    }
}
