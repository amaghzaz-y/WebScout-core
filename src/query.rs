use std::collections::{HashMap, HashSet};

use crate::{
    index::Index,
    tokenizer::{self, Tokenizer},
};

pub struct Query {
    index: Index,
    lang: String,
    search: String,
    tokens: Vec<String>,
    filter: HashMap<String, HashMap<String, HashSet<usize>>>,
    result: Vec<(String, usize)>,
}

impl Query {
    pub fn new(index: Index, search: String, lang: String) -> Query {
        Query {
            index: index,
            lang: lang,
            search: search,
            tokens: vec![],
            filter: HashMap::new(),
            result: vec![],
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
                self.filter
                    .entry(token.to_owned())
                    .or_insert(keys.unwrap().to_owned());
            }
        }
    }
    pub fn normalize(&mut self) {
        self.tokenize_query();
        self.filter_tokens();
        println!("{:?}", self.filter);
    }
}
