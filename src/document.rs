use crate::tokenizer::{self, Tokenizer};
use crate::utils::{self, mean, standard_deviation};
use crc32fast::hash;
use itertools::Itertools;
use patricia_tree::PatriciaMap;
use serde::{Deserialize, Serialize};
extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use hashbrown::{HashMap as FastMap, HashSet};
use std::collections::HashMap;
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
    pub index: FastMap<String, Weight>,
}

impl Document {
    pub fn new(
        name: String,
        body: String,
        language: String,
        tokenizer: &mut Tokenizer,
    ) -> Document {
        let mut document: Document = Document {
            id: hash(name.as_bytes()),
            lang: language,
            index: FastMap::new(),
            count: 0,
        };

        let index = document.index(&body, tokenizer);
        document.transform_map(&index);
        return document;
    }

    pub fn index(
        &mut self,
        mut body: &String,
        tokenizer: &mut Tokenizer,
    ) -> HashMap<String, HashSet<usize>> {
        let mut count = 0;
        let s = body
            .split_whitespace()
            .enumerate()
            .map(|(pos, word)| (word.to_owned(), pos))
            .into_group_map()
            .iter()
            .map(|(word, pos)| {
                let token = tokenizer
                    .auto_tokenize(word)
                    .unwrap_or_else(|| word.to_owned());
                count += 1;
                (token, pos.to_owned())
            })
            .into_group_map();
        self.count = count;
        self.convert_map_ref(&s)
    }

    fn convert_map_ref(
        &self,
        input_map: &HashMap<String, Vec<Vec<usize>>>,
    ) -> HashMap<String, HashSet<usize>> {
        let mut output_map: HashMap<String, HashSet<usize>> = HashMap::new();
        let mut temp_set = HashSet::new();
        for (key, value) in input_map {
            for sub_set in value {
                temp_set.extend(sub_set.to_owned());
            }
            output_map
                .entry(key.to_string())
                .or_insert_with(HashSet::new)
                .extend(&temp_set);
            temp_set.clear();
        }
        output_map
    }

    pub fn transform_map(&mut self, map: &HashMap<String, HashSet<usize>>) {
        let transform: HashMap<String, (usize, usize, usize)> = HashMap::new();
        let s = map.iter().map(|(token, pos)| {
            let pos_vec: Vec<f32> = pos.into_iter().map(|x| *x as f32).collect();
            let weight = Weight {
                freq: pos_vec.len() as u32,
                mean: mean(&pos_vec),
                devi: standard_deviation(&pos_vec),
            };
            (token.to_owned(), weight)
        });
        let u = (23 as usize, 34 as usize, 54.32 as usize);
        self.index = FastMap::from_iter(s);
    }

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
