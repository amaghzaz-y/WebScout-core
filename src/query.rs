use itertools::Itertools;
use serde::__private::doc;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::iter::FlatMap;
use std::thread::Result;

use crate::utils::{mean, standard_deviation};
use crate::{
    document::Statistics,
    index::Index,
    tokenizer::{self, Tokenizer},
};
#[derive(Debug, Clone)]

pub struct DocStat {
    pub mean_of_means: f32,
    pub mean_of_deviations: f32,
    pub mean_of_frequencies: f32,
    pub deviation_of_means: f32,
    pub deviation_of_deviations: f32,
}
pub struct QueryStats {
    pub means: Vec<usize>,
    pub deviations: Vec<usize>,
    pub frequencies: Vec<usize>,
}
#[derive(Debug, Clone)]

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
    }
    fn normalize(&self) -> HashMap<u32, QueryStats> {
        let mut ndata: HashMap<u32, QueryStats> = HashMap::new();
        for (key, value) in &self.result {
            let mut means = Vec::new();
            let mut deviations = Vec::new();
            let mut freqs = Vec::new();
            for (_, s) in value.iter() {
                means.push(s.average);
                deviations.push(s.deviation);
                freqs.push(s.frequency);
            }
            ndata.insert(
                key.to_owned(),
                QueryStats {
                    means: means,
                    deviations: deviations,
                    frequencies: freqs,
                },
            );
        }
        return ndata;
    }
    pub fn evaluate(self) {
        let query = self.normalize();
        let mut doc_stat: HashMap<u32, DocStat> = HashMap::new();
        for (d, stats) in query {
            let means: &[f32] = &stats.means.iter().map(|x| *x as f32).collect::<Vec<f32>>();
            let deviations: &[f32] = &stats
                .deviations
                .iter()
                .map(|x| *x as f32)
                .collect::<Vec<f32>>();
            let frequencies: &[f32] = &stats
                .frequencies
                .iter()
                .map(|x| *x as f32)
                .collect::<Vec<f32>>();
            let mean_of_means = mean(means);
            let mean_of_freqs = mean(frequencies);
            let mean_of_deviations = mean(deviations);
            let deviation_of_means = standard_deviation(means);
            let deviation_of_deviations = standard_deviation(deviations);
            doc_stat.insert(
                d,
                DocStat {
                    mean_of_means: mean_of_means,
                    mean_of_deviations: mean_of_deviations,
                    mean_of_frequencies: mean_of_freqs,
                    deviation_of_means: deviation_of_means,
                    deviation_of_deviations: deviation_of_deviations,
                },
            );
        }
        println!("{:?}", doc_stat)
    }
}
