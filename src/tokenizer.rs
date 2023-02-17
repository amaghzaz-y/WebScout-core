use itertools::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    hash::Hash,
};

use crate::{document::Document, pairset::PairSet};
#[derive(Serialize, Deserialize)]
pub struct Tokenizer {
    pub lang: String,
    pub tokens: PairSet,
}
impl Tokenizer {
    pub fn new(lang: &str) -> Tokenizer {
        Tokenizer {
            lang: lang.to_owned(),
            tokens: PairSet::new(),
        }
    }
    pub fn transform_token(&self, word: &String) -> String {
        let mut lemma: String = word.to_owned();
        let value = self.tokens.get(&lemma);
        if value.is_some() {
            lemma = value.unwrap().to_string();
        }
        return lemma;
    }
    pub fn filter(&self, prefix_suffix: (&String, &String)) -> HashSet<String> {
        let mut tokens: HashSet<String> = HashSet::new();
        for token in &self.tokens {
            if token.starts_with(prefix_suffix.0) || token.ends_with(prefix_suffix.1) {
                tokens.insert(token.to_owned());
            }
        }
        return tokens;
    }
    pub fn eval(&self, entry: &str, tokens: &HashSet<String>) -> Option<(String, f64)> {
        tokens
            .iter()
            .map(|token| (token.to_owned(), strsim::jaro_winkler(entry, token)))
            .sorted_by(|(a), b| b.1.total_cmp(&a.1))
            .nth(0)
    }
    pub fn get_prefix_suffix(
        &self,
        token: &mut String,
        mut limit: (usize, usize),
    ) -> (String, String) {
        if limit.0 > token.len() {
            limit.0 = token.len();
        }
        if limit.1 > token.len() {
            limit.1 = token.len();
        }
        token.make_ascii_lowercase();
        (
            token.split_at(limit.0).0.to_string(),
            token.split_at(token.len() - limit.1).1.to_string(),
        )
    }
    pub fn auto_tokenize(&self, mut word: &str) -> Option<(String, f64)> {
        let mut token: String = word.chars().filter(|c| c.is_alphanumeric()).collect();
        let prefix_suff = self.get_prefix_suffix(&mut token, (3, 3));
        // let filtred = self.filter((&prefix_suff.0, &prefix_suff.1));
        self.eval(&token, &self.tokens)
    }
    pub fn construct_tokens(&mut self, text: &str) {
        let words = text
            .lines()
            .map(|line| line.split_whitespace().next().unwrap().to_string());
        self.tokens = HashSet::from_iter(words);
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
