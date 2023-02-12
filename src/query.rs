use std::collections::{HashMap, HashSet};

use serde::__private::doc;

use crate::{
    index::Index,
    tokenizer::{self, Tokenizer},
};
pub struct Document {
    id: String,
    tokens: Vec<String>,
    score: usize,
}
pub struct Query {
    index: Index,
    lang: String,
    search: String,
    tokens: Vec<String>,
    filter: HashMap<String, HashMap<u32, HashSet<usize>>>,
    result: HashMap<u32, HashMap<String, Vec<usize>>>,
}

impl Query {
    pub fn new(index: Index, search: String, lang: String) -> Query {
        Query {
            index: index,
            lang: lang,
            search: search,
            tokens: vec![],
            filter: HashMap::new(),
            result: HashMap::new(),
        }
    }
    fn tokenize_query(&mut self) {
        let tokenizer = Tokenizer::get(&self.lang);
        for token in self.search.split_ascii_whitespace() {
            let lemma = tokenizer.transform_token(&token.to_owned());
            self.tokens.push(lemma);
        }
    }
    fn filter_tokens(&mut self) {
        for token in self.tokens.iter() {
            let keys = self.index.map.get(token);
            if keys.is_some() {
                self.filter.entry(token.to_owned());
            }
        }
    }
    pub fn normalize(&mut self) {
        let mut ndata: HashSet<(String, String, usize, Vec<usize>)> = HashSet::new();
        self.tokenize_query();
        self.filter_tokens();
        for key in self.filter.iter() {
            let token = key.0.to_owned();
            let map = key.1.to_owned();
            for doc in map {
                let mut positions: Vec<usize> = doc.1.into_iter().collect();
                positions.sort_unstable_by(|a, b| a.cmp(b));
                self.result
                    .entry(doc.0)
                    .or_insert(HashMap::from([(token.to_owned(), positions.to_owned())]))
                    .insert(token.to_owned(), positions);
            }
        }
    }
    fn word_distance(tokens: Vec<(String, Vec<usize>)>) {
        for token in tokens {}
    }
    pub fn evaluate(&self) {
        for document in &self.result {
            let tokens: Vec<_> = document.1.to_owned().into_iter().collect();
        }
    }
}
