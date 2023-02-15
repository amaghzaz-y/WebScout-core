use itertools::*;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    hash::Hash,
};
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
    pub fn filter(&self, prefix: &[&str], suffix: &[&str]) -> HashSet<String> {
        let prefix_set: HashSet<_> = prefix.iter().collect();
        let suffix_set: HashSet<_> = suffix.iter().collect();
        let mut tokens: HashSet<String> = HashSet::new();
        for token in &self.tokens {
            for pref in prefix_set.iter() {
                if token.starts_with(*pref) {
                    tokens.insert(token.to_owned());
                }
            }
            for suff in suffix_set.iter() {
                if token.ends_with(*suff) {
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
    pub fn auto_eval(&self, entry: &str) {
        
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
