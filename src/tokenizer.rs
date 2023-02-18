use crate::document::Document;
use itertools::*;
use patricia_tree::PatriciaSet;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    hash::Hash,
};
#[derive(Serialize, Deserialize)]
pub struct Tokenizer {
    pub lang: String,
    pub tokens: PatriciaSet,
}
impl Tokenizer {
    pub fn new(lang: &str) -> Tokenizer {
        Tokenizer {
            lang: lang.to_owned(),
            tokens: PatriciaSet::new(),
        }
    }
    pub fn eval(&self, entry: &str, tokens: &HashSet<String>) -> Option<(String, f64)> {
        tokens
            .iter()
            .map(|token| (token.to_owned(), strsim::jaro_winkler(entry, token)))
            .sorted_by(|(a), b| b.1.total_cmp(&a.1))
            .nth(0)
    }
    // pub fn auto_tokenize(&self, mut word: &str) -> Option<(String, f64)> {
    //     let mut token: String = word.chars().filter(|c| c.is_alphanumeric()).collect();
    //     let prefix_suff = self.get_prefix_suffix(&mut token, (3, 3));
    //     // let filtred = self.filter((&prefix_suff.0, &prefix_suff.1));
    //     self.eval(&token, &self.tokens)
    // }
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
