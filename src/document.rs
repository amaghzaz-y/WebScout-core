use crate::tokenizer::Tokenizer;
use crate::utils::{mean, standard_deviation};
use crc32fast::hash;
use serde::{Deserialize, Serialize};
extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::{HashMap, HashSet};
use regex::Regex;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]

pub struct Weight {
    pub freq: u32,
    pub mean: u32,
    pub devi: u32,
}
#[derive(Serialize, Deserialize, Debug, Clone)]

pub struct Document {
    pub id: u32,
    pub lang: String,
    pub count: u32,
    pub index: HashMap<String, Weight>,
    #[serde(skip)]
    data: HashMap<String, HashSet<u32>>,
}

impl Document {
    pub fn new(
        name: String,
        body: &mut str,
        language: String,
        tokenizer: &mut Tokenizer,
    ) -> Document {
        let mut document: Document = Document {
            id: hash(name.as_bytes()),
            lang: language,
            index: HashMap::default(),
            data: HashMap::default(),
            count: 0,
        };

        document.index(body, tokenizer);
        document.transform_data();
        return document;
    }
    pub fn index(&mut self, text: &mut str, tokenizer: &mut Tokenizer) {
        let re = Regex::new(r#"\W+"#).unwrap();
        let mut count: u32 = 0;
        re.split(&text.to_ascii_lowercase())
            .map(|s| {
                count += 1;
                (tokenizer.tokenize(s).unwrap_or(s.to_owned()), count)
            })
            .for_each(|(k, v)| self.add_entry(k, v));
        self.count = count;
    }
    fn add_entry(&mut self, k: String, v: u32) {
        self.data.entry(k).or_default().insert(v);
    }
    fn transform_data(&mut self) {
        let s = self.data.iter().map(|(token, pos)| {
            let pos_vec: Vec<f32> = pos.into_iter().map(|x| *x as f32).collect();
            let weight = Weight {
                freq: pos_vec.len() as u32,
                mean: mean(&pos_vec) as u32,
                devi: standard_deviation(&pos_vec) as u32,
            };
            (token.to_owned(), weight)
        });
        self.index = HashMap::from_iter(s);
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
