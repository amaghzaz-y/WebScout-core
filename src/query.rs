use crate::document::Weight;
use crate::utils::standard_deviation;
use crate::{index::Index, tokenizer::Tokenizer};
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::HashMap;
use regex::Regex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Query {
    index: Index,
    tokenizer: Tokenizer,
}

impl Query {
    pub fn new(index: &Index, tokenizer: &Tokenizer) -> Query {
        let query = Query {
            index: index.to_owned(),
            tokenizer: tokenizer.to_owned(),
        };
        return query;
    }

    pub fn default() -> Query {
        Query {
            index: Index::new(),
            tokenizer: Tokenizer::new("en"),
        }
    }

    pub fn setup(&mut self, index: &Index, tokenizer: &Tokenizer) {
        self.index = index.to_owned();
        self.tokenizer = tokenizer.to_owned();
    }

    fn tokenize_query(&mut self, mut query: String) -> Vec<String> {
        query.make_ascii_lowercase();
        let re = Regex::new(r#"\W+"#).unwrap();
        re.split(&query)
            .map(|s| self.tokenizer.tokenize(s).unwrap_or(s.to_owned()))
            .collect::<Vec<String>>()
    }

    fn filter(&self, tokens: &mut Vec<String>) -> Vec<(String, HashMap<u32, Weight>)> {
        tokens.dedup();
        tokens
            .iter()
            .map(|t| (t, self.index.get(t)))
            .filter(|(_, t)| t.is_some())
            .map(|(s, t)| (s.to_owned(), t.unwrap().to_owned()))
            .collect()
    }

    fn transpose(
        &self,
        tokens: &[(String, HashMap<u32, Weight>)],
    ) -> HashMap<u32, Vec<(String, Weight)>> {
        let mut map = HashMap::new();
        for (t, m) in tokens {
            for (d, w) in m {
                map.entry(*d)
                    .or_insert_with(Vec::new)
                    .push((t.clone(), w.clone()));
            }
        }
        map
    }

    fn score(
        &self,
        tokens: Vec<String>,
        map: &HashMap<u32, Vec<(String, Weight)>>,
    ) -> (HashMap<u32, u8>, u8) {
        let mut deviations = HashMap::with_capacity(map.len());
        let mut total_freqs: HashMap<String, u32> = HashMap::with_capacity(map.len());
        let mut token_scores = HashMap::with_capacity(map.len());
        let mut scores: u16 = 0;
        let mut result: HashMap<u32, u8> = HashMap::new();
        for (doc_id, doc) in map {
            let count = doc.len() as u32;
            let means: Vec<f32> = doc
                .iter()
                .map(|(t, w)| {
                    *total_freqs.entry(t.to_owned()).or_default() += w.freq;
                    w.mean as f32
                })
                .collect();
            let deviation = standard_deviation(&means);
            deviations.insert(*doc_id, (deviation, count));

            let mut ratio_sum: f32 = 0.0;
            for (token, w) in doc {
                ratio_sum +=
                    w.freq as f32 / *total_freqs.entry(token.to_owned()).or_default() as f32;
            }
            token_scores.insert(*doc_id, ratio_sum / count as f32);
        }

        let num_tokens = tokens.len() as f32;

        for (doc_id, _) in map {
            let (devi, count) = *deviations.get(doc_id).unwrap();
            let freq_ratio = *token_scores.get(doc_id).unwrap_or(&0.0);
            let words_found_ratio = count as f32 / num_tokens;
            let score = (((words_found_ratio * 6.0 + freq_ratio + ((1.0 / (devi + 1.0)) * 3.0))
                / 10.0)
                * 100.0)
                .floor() as u8;
            scores += score as u16;
            result.insert(*doc_id, score);
        }
        return (result, ((scores / map.len() as u16) as u8));
    }

    pub fn search(&mut self, query: &str) -> (HashMap<u32, u8>, u8) {
        let mut tokens = self.tokenize_query(query.to_owned());
        println!("{:?}", tokens);
        let filter = self.filter(&mut tokens);
        if filter.len() > 0 {
            let map = self.transpose(&filter);
            return self.score(tokens, &map);
        }
        return (HashMap::default(), 0);
    }

    pub fn above_average(&self, result: HashMap<u32, u8>, avg: u8) -> Vec<(String, u8)> {
        result
            .iter()
            .filter(|(_, score)| score >= &&avg)
            .map(|(id, score)| (self.index.get_title(id), *score))
            .collect()
    }

    pub fn all(&mut self, result: HashMap<u32, u8>) -> Vec<(String, u8)> {
        result
            .iter()
            .map(|(id, score)| (self.index.get_title(id), *score))
            .collect()
    }
}
