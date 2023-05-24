extern crate alloc;
use crate::jaro;
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use hashbrown::{HashMap, HashSet};
use patricia_tree::PatriciaSet;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tokenizer {
    lang: String,
    tokens: PatriciaSet,
    #[serde(skip)]
    cache: HashMap<String, String>,
}

impl Tokenizer {
    pub fn new(lang: &str) -> Tokenizer {
        Tokenizer {
            lang: lang.to_owned(),
            tokens: PatriciaSet::new(),
            cache: HashMap::default(),
        }
    }

    fn eval(&self, entry: &str, tokens: &HashSet<String>) -> Option<String> {
        let mut best_score: f32 = 0.0;
        let mut best_token: Option<String> = None;
        for token in tokens.iter() {
            let score = jaro::jaro(entry, token) as f32;
            if score > 0.65 && score > best_score {
                best_score = score;
                best_token = Some(token.to_owned());
            }
        }
        best_token
    }

    fn cache_check(&self, value: &str) -> Option<String> {
        self.cache.get(value).map(|token| token.to_owned())
    }

    fn add_to_cache(&mut self, k: &str, v: &str) {
        self.cache.insert(k.to_string(), v.to_string());
    }

    fn tree_check(&self, value: &str) -> Option<String> {
        self.tokens
            .iter_prefix(value.as_bytes())
            .filter_map(|b| String::from_utf8(b.to_vec()).ok())
            .next()
    }

    fn search(&self, value: &str) -> Option<String> {
        let prefix: &[u8];
        if value.len() > 4 {
            prefix = value
                .as_bytes()
                .get(..(value.len() as f32 * 0.65) as usize)
                .unwrap_or_else(|| value.as_bytes());
        } else {
            prefix = value.as_bytes();
        }
        let mut tokens = HashSet::default();
        for b in self.tokens.iter_prefix(prefix) {
            if let Ok(s) = String::from_utf8(b.to_vec()) {
                tokens.insert(s);
            }
        }
        self.eval(&value, &tokens)
    }

    pub fn tokenize(&mut self, value: &str) -> Option<String> {
        match self.cache_check(value) {
            Some(c) => Some(c),
            None => {
                let result = self.tree_check(value).or_else(|| self.search(value));
                result.map(|r| {
                    self.add_to_cache(value, &r);
                    r
                })
            }
        }
    }

    pub fn construct_tokens(&mut self, text: &str) {
        let words = text
            .lines()
            .map(|line| line.split_whitespace().next().unwrap().to_string())
            .map(|token| token);
        self.tokens = PatriciaSet::from_iter(words);
    }

    pub fn to_pack(&self) -> Vec<u8> {
        rmp_serde::encode::to_vec(&self).unwrap()
    }

    pub fn from_pack(bin: &[u8]) -> Tokenizer {
        rmp_serde::from_slice(&bin).unwrap()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}
