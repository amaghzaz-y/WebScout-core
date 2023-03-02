use crate::document::Weight;
use crate::utils::standard_deviation;
use crate::{index::Index, tokenizer::Tokenizer};
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use hashbrown::{HashMap, HashSet};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct Query {
    index: Index,
    tokenizer: Tokenizer,
    search: String,
    result: HashSet<(u32, i32)>,
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
        tokens
            .iter()
            .map(|t| (t, self.index.get(t)))
            .filter(|(s, t)| t.is_some())
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

    fn total_freq(&self, tokens: &[(String, HashMap<u32, Weight>)]) -> HashMap<String, u32> {
        tokens
            .iter()
            .flat_map(|(t, m)| m.values().map(move |w| (t, w.freq)))
            .fold(HashMap::new(), |mut acc, (t, freq)| {
                *acc.entry(t.clone()).or_insert(0) += freq;
                acc
            })
    }
    fn score(&self, map: &HashMap<u32, Vec<(String, Weight)>>) {
        let mut deviations: HashMap<u32, (f32, u32)> = HashMap::new();
        let mut total_freqs: HashMap<String, u32> = HashMap::new();
        let mut token_scores: HashMap<u32, f32> = HashMap::new();
        for doc in map {
            let count = doc.1.len() as u32;
            let means: Vec<f32> = doc
                .1
                .iter()
                .map(|(t, w)| {
                    *total_freqs.entry(t.to_owned()).or_default() += w.freq;
                    w.mean as f32
                })
                .collect();
            let deviation = standard_deviation(&means);
            deviations.insert(doc.0.to_owned(), (deviation, count));
        }
        for doc in map {
            let mut ratio_sum: f32 = 0.0;
            for (token, w) in doc.1 {
                ratio_sum += w.freq as f32 / *total_freqs.get(token).unwrap_or(&1) as f32;
            }
            token_scores.insert(doc.0.to_owned(), ratio_sum / doc.1.len() as f32);
        }
        for doc in map {
            let (devi, count) = deviations.get(doc.0).unwrap();
        }
        println!("{:?}", deviations);
        println!("{:?}", total_freqs);
        println!("{:?}", token_scores);
    }
    pub fn search(&mut self) {
        let tokens = self.tokenize_query();
        let filter = self.filter(&tokens);
        let map = self.transpose(&filter);
        self.score(&map);
    }
}
