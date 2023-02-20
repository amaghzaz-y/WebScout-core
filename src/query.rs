use crate::document::Weight;
use crate::utils::{mean, standard_deviation};
use crate::{index::Index, tokenizer::Tokenizer};
use alloc::borrow::{Cow, ToOwned};
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Query {
    index: Index,
    tokenizer: Tokenizer,
    search: String,
    result: HashSet<(String, i32)>,
}

impl Query {
    pub fn new(search: &str, index: &Index, tokenizer: &mut Tokenizer) -> Query {
        let query = Query {
            index: index.clone(),
            tokenizer: tokenizer.to_owned(),
            search: search.to_string(),
            result: HashSet::new(),
        };
        return query;
    }
    fn tokenize_query(&mut self) -> Vec<String> {
        let re = Regex::new(r#"\W+"#).unwrap();
        re.split(&self.search.to_ascii_lowercase())
            .map(|s| self.tokenizer.tokenize(s).unwrap_or(s.to_owned()))
            .collect::<Vec<String>>()
    }
    fn filter(&mut self, tokens: &Vec<String>) -> Vec<(String, HashMap<u32, Weight>)> {
        tokens.to_owned().dedup();
        let s: Vec<(String, HashMap<u32, Weight>)> = tokens
            .iter()
            .map(|t| (t, self.index.get(t)))
            .filter(|(s, t)| t.is_some())
            .map(|(s, t)| (s.to_owned(), t.unwrap().to_owned()))
            .collect();
        s
    }
}
