use itertools::Itertools;
use serde::__private::doc;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FlatMap;
use std::thread::Result;

use crate::{
    document::Statistics,
    index::Index,
    tokenizer::{self, Tokenizer},
};
pub struct Document {
    id: String,
    tokens: Vec<String>,
    score: usize,
}
#[derive(Debug, Clone)]
pub struct Query {
    index: Index,
    lang: String,
    pub search: String,
    tokens: Vec<String>,
    filter: HashMap<String, HashMap<u32, Statistics>>,
    result: HashMap<u32, Vec<(String, Statistics)>>,
}

impl Query {
    pub fn new(index: Index, search: String, lang: String) -> Query {
        let mut query = Query {
            index: index,
            lang: lang,
            search: search,
            tokens: vec![],
            filter: HashMap::new(),
            result: HashMap::new(),
        };
        query.tokenize_query();
        query.filter_tokens();
        query.transform();
        return query;
    }
    fn tokenize_query(&mut self) {
        let tokenizer = Tokenizer::get(&self.lang);
        self.search
            .split_ascii_whitespace()
            .map(|token| tokenizer.transform_token(&token.to_ascii_lowercase()))
            .for_each(|lemma| {
                self.tokens.push(lemma);
            });
    }
    fn filter_tokens(&mut self) {
        self.tokens
            .iter()
            .filter_map(|token| self.index.map.get(token).map(|map| (token, map)))
            .for_each(|(token, map)| {
                self.filter
                    .entry(token.to_owned())
                    .or_insert(map.to_owned());
            });
    }
    fn transform(&mut self) {
        let map: HashMap<String, HashMap<u32, Statistics>> = self.filter.clone();
        self.result = map
            .into_iter()
            .flat_map(|(k, v)| {
                v.into_iter()
                    .map(move |(key, value)| (key, (k.to_owned(), value)))
            })
            .into_group_map();
        println!("transform : {:?}", self.result);
    }
}
