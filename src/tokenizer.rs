use itertools::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    hash::Hash,
};

use crate::document::Document;
#[derive(Serialize, Deserialize)]
pub struct Tokenizer {
    pub lang: String,
    pub tokens: HashSet<String>,
}
impl Tokenizer {
    pub fn new(lang: &str) -> Tokenizer {
        Tokenizer {
            lang: lang.to_owned(),
            tokens: HashSet::new(),
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
    pub fn filter(&self, prefix_suffix: &HashSet<(String, String)>) -> HashSet<String> {
        let mut tokens: HashSet<String> = HashSet::new();
        for token in &self.tokens {
            for (pref, suff) in prefix_suffix.iter() {
                if token.starts_with(pref) || token.ends_with(suff) {
                    tokens.insert(token.to_owned());
                }
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
        words: &HashSet<String>,
        mut limit: (usize, usize),
    ) -> HashSet<(String, String)> {
        words
            .iter()
            .map(|token| {
                if limit.0 > token.len() {
                    limit.0 = token.len();
                }
                if limit.1 > token.len() {
                    limit.1 = token.len();
                }
                (
                    token.to_owned().split_at(limit.0).0.to_string(),
                    token
                        .to_owned()
                        .split_at(token.len() - limit.1)
                        .1
                        .to_string(),
                )
            })
            .collect()
    }
    pub fn auto_tokenize(&self, text: &str) -> Vec<Option<(String, f64)>> {
        let words: HashSet<String> = text
            .split_whitespace()
            .map(|token| token.to_lowercase())
            .collect();
        let prefix_suff = self.get_prefix_suffix(&words, (3, 3));
        let filtred = self.filter(&prefix_suff);
        words
            .iter()
            .map(|token| self.eval(token, &filtred))
            .collect()
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
    pub fn from_pack(bin: &[u8]) -> Tokenizer {
        rmp_serde::from_slice(&bin).unwrap()
    }
    pub fn to_json(&self) -> String {
        let json = serde_json::to_string_pretty(&self).unwrap();
        return json;
    }
}
