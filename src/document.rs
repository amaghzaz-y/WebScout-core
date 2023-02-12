use crate::tokenizer::{self, Tokenizer};
use crate::utils::{self, mean, standard_deviation};
use crc32fast::hash;
use serde::{Deserialize, Serialize, __private::doc};
use std::collections::{HashMap, HashSet};
#[derive(Serialize, Deserialize, Clone, Copy, Debug)]

pub struct Statistics {
    frequency: usize,
    average: usize,
    deviation: usize,
}
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Document {
    pub id: u32,
    pub lang: String,
    // Map? Token -> (freq, mean, deviation)
    pub index: HashMap<String, Statistics>,
    pub count: usize,
}
impl Document {
    pub fn new(name: String, body: String, language: String) -> Document {
        let mut document: Document = Document {
            id: hash(name.as_bytes()),
            lang: language,
            index: HashMap::new(),
            count: 0,
        };
        let mut map = document.index_string(body);
        map = document.tokenize(map);
        document.transform_map(map);
        return document;
    }
    fn index_string(&mut self, mut body: String) -> HashMap<String, HashSet<usize>> {
        let mut chars: Vec<u8> = vec![];
        let mut count: usize = 0;
        let mut map: HashMap<String, HashSet<usize>> = HashMap::new();
        body.push('/'); // to mark the end of document
        for char in body.as_bytes() {
            if char.is_ascii_alphanumeric() {
                chars.push(*char);
            } else {
                if chars.len() > 1 {
                    let mut word = String::from_utf8(chars.to_owned()).unwrap();
                    word.make_ascii_lowercase();
                    map.entry(word)
                        .or_insert(HashSet::from([count.to_owned()]))
                        .insert(count.to_owned());
                }
                count += 1;
                chars.clear();
            }
        }
        self.count = count;
        return map;
    }

    fn tokenize(
        &self,
        mut map: HashMap<String, HashSet<usize>>,
    ) -> HashMap<String, HashSet<usize>> {
        let tokenizer = Tokenizer::get(&self.lang);
        map = tokenizer.tokenize_map(&map);
        return map;
    }

    fn transform_map(&mut self, map: HashMap<String, HashSet<usize>>) {
        let transform: HashMap<String, (usize, usize, usize)> = HashMap::new();
        for (token, positions) in map {
            let pos_vec: Vec<f32> = positions.into_iter().map(|x| x as f32).collect();
            let stats = Statistics {
                frequency: pos_vec.len(),
                average: mean(&pos_vec) as usize,
                deviation: standard_deviation(&pos_vec) as usize,
            };
            self.index.insert(token, stats);
        }
    }
    pub fn serialize(&self) -> Vec<u8> {
        let bin = rmp_serde::encode::to_vec(self).unwrap();
        return bin;
    }
    pub fn to_json(&self) -> String {
        let json = serde_json::to_string_pretty(self).unwrap();
        return json;
    }
}
