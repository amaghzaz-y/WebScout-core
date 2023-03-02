use crate::document::Weight;
use crate::utils::standard_deviation;
use crate::{index::Index, tokenizer::Tokenizer};
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use hashbrown::HashMap;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Query {
    index: Index,
    tokenizer: Tokenizer,
    search: String,
    tokens: Vec<String>,
    result: Vec<(u32, f32)>,
}

impl Query {
    pub fn new(search: &str, index: &Index, tokenizer: &mut Tokenizer) -> Query {
        let query = Query {
            index: index.clone(),
            tokenizer: tokenizer.to_owned(),
            tokens: vec![],
            search: search.to_string(),
            result: Vec::new(),
        };
        return query;
    }
    fn tokenize_query(&mut self) {
        let re = Regex::new(r#"\W+"#).unwrap();
        self.tokens = re
            .split(&self.search.to_ascii_lowercase())
            .map(|s| self.tokenizer.tokenize(s).unwrap_or(s.to_owned()))
            .collect::<Vec<String>>()
    }
    fn filter(&mut self) -> Vec<(String, HashMap<u32, Weight>)> {
        self.tokens.to_owned().dedup();
        self.tokens
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
    fn score(&mut self, map: &HashMap<u32, Vec<(String, Weight)>>) {
        let mut deviations = HashMap::with_capacity(map.len());
        let mut total_freqs: HashMap<String, u32> = HashMap::with_capacity(map.len());
        let mut token_scores = HashMap::with_capacity(map.len());

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

        let num_tokens = self.tokens.len() as f32;

        for (doc_id, _) in map {
            let (devi, count) = *deviations.get(doc_id).unwrap();
            let freq_ratio = *token_scores.get(doc_id).unwrap_or(&0.0);
            let words_found_ratio = count as f32 / num_tokens;
            let score =
                (words_found_ratio * 7.0 + freq_ratio + ((1.0 / (devi + 1.0)) * 3.0)) / 10.0;
            self.result.push((*doc_id, (score * 100.0).floor()));
        }
    }
    pub fn search(&mut self) {
        self.tokenize_query();
        let filter = self.filter();
        let map = self.transpose(&filter);
        self.score(&map);
    }
    pub fn all(&mut self) -> Vec<(String, f32)> {
        self.result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        self.result
            .iter()
            .map(|(id, score)| (self.index.get_title(id), score.to_owned()))
            .collect()
    }
}
