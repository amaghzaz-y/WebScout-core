use crate::document::Document;
use crate::utils::to_lower_alphanumeric;
use hashbrown::{hash_map::Entry, HashMap, HashSet};
use itertools::*;
use patricia_tree::PatriciaSet;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::{collections::BTreeMap, fs, hash::Hash};
#[derive(Serialize, Deserialize)]
pub struct Tokenizer {
    pub lang: String,
    pub tokens: PatriciaSet,
    pub cache: HashMap<String, String>,
}
impl Tokenizer {
    pub fn new(lang: &str) -> Tokenizer {
        Tokenizer {
            lang: lang.to_owned(),
            tokens: PatriciaSet::new(),
            cache: HashMap::new(),
        }
    }
    pub fn eval(&self, entry: &str, tokens: &HashSet<String>) -> Option<String> {
        tokens
            .iter()
            .map(|token| (token.to_owned(), strsim::jaro_winkler(entry, token)))
            .filter(|(_, score)| score > &0.65)
            .sorted_by(|(a), b| b.1.total_cmp(&a.1))
            .nth(0)
            .map(|(t, s)| t)
    }
    pub fn auto_tokenize(&mut self, word: &str) -> Option<String> {
        let token = to_lower_alphanumeric(word);
        let result = match self.cache.get(&token) {
            Some(string) => Some(string.to_owned()),
            None => {
                let prefix: &[u8];
                if token.len() > 4 {
                    prefix = token
                        .as_bytes()
                        .get(..(token.len() as f32 * 0.6) as usize)
                        .unwrap_or_else(|| token.as_bytes());
                } else {
                    prefix = token.as_bytes();
                }
                let tokens: HashSet<String> = HashSet::from_iter(
                    self.tokens
                        .iter_prefix(prefix)
                        .filter_map(|b| String::from_utf8(b.to_vec()).ok()),
                );
                let lemma = self.eval(&token, &tokens);
                if lemma.is_some() {
                    self.cache.insert(token, lemma.to_owned().unwrap());
                }
                lemma
            }
        };
        result
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
    pub fn from_fs(lang: &String) -> Tokenizer {
        let bin = fs::read(format!("packs/{}.pack", lang)).unwrap();
        Tokenizer::from_pack(&bin)
    }
    pub fn from_pack(bin: &[u8]) -> Tokenizer {
        rmp_serde::from_slice(&bin).unwrap()
    }
    pub fn to_json(&self) -> String {
        let json = serde_json::to_string_pretty(&self).unwrap();
        return json;
    }
}
