use crate::tokenizer::{self, Tokenizer};
use crate::utils::{self, mean, standard_deviation};
use crc32fast::hash;
use itertools::Itertools;
use serde::{Deserialize, Serialize, __private::doc};
use std::collections::{HashMap, HashSet};
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]

pub struct Weight {
    pub freq: u32,
    pub mean: f32,
    pub devi: f32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Document {
    pub id: u32,
    pub lang: String,
    pub count: u32,
    pub index: HashMap<String, Weight>,
}

impl Document {
    pub fn new(name: String, body: String, language: String) -> Document {
        let mut document: Document = Document {
            id: hash(name.as_bytes()),
            lang: language,
            index: HashMap::new(),
            count: 0,
        };
        return document;
    }

    fn index_string(&mut self, mut body: &String) -> HashMap<String, Vec<usize>> {
        body.split_whitespace()
            .enumerate()
            .map(|(pos, word)| (word.to_owned(), pos))
            .into_group_map()
    }

    fn tokenize(
        &self,
        mut map: &HashMap<String, HashSet<usize>>,
    ) -> HashMap<String, HashSet<usize>> {
        let tokenizer = Tokenizer::new(&self.lang);
        let s = map
            .into_iter()
            .map(|(token, pos)| {
                (
                    tokenizer.auto_tokenize(token)[0]
                        .as_ref()
                        .unwrap()
                        .0
                        .to_owned(),
                    pos.to_owned(),
                )
            })
            .into_group_map();
        return self.convert_map(s);
    }
    fn convert_map(
        &self,
        input_map: HashMap<String, Vec<HashSet<usize>>>,
    ) -> HashMap<String, HashSet<usize>> {
        let mut output_map: HashMap<String, HashSet<usize>> = HashMap::new();
        let mut temp_set = HashSet::new();
        for (key, value) in input_map {
            for sub_set in value {
                temp_set.extend(sub_set);
            }
            output_map
                .entry(key)
                .or_insert_with(HashSet::new)
                .extend(&temp_set);
            temp_set.clear();
        }
        output_map
    }

    // fn transform_map(&mut self, map: HashMap<String, HashSet<usize>>) {
    //     let transform: HashMap<String, (usize, usize, usize)> = HashMap::new();
    //     for (token, positions) in map {
    //         let pos_vec: Vec<f32> = positions.into_iter().map(|x| x as f32).collect();
    //         let stats = Statistics {
    //             frequency: pos_vec.len(),
    //             average: mean(&pos_vec) as usize,
    //             deviation: standard_deviation(&pos_vec) as usize,
    //         };
    //         self.index.insert(token, stats);
    //     }
    // }

    pub fn to_pack(&self) -> Vec<u8> {
        rmp_serde::encode::to_vec(self).unwrap()
    }

    pub fn from_pack(&self, bin: &[u8]) -> Document {
        rmp_serde::decode::from_slice(bin).unwrap()
    }

    pub fn to_json(&self) -> String {
        let json = serde_json::to_string_pretty(self).unwrap();
        return json;
    }
}
