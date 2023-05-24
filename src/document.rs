use crate::tokenizer::Tokenizer;
use crate::utils::mean;
use crc32fast::hash;
use serde::{Deserialize, Serialize};
extern crate alloc;
use alloc::borrow::ToOwned;
use alloc::string::String;
use alloc::vec::Vec;
use hashbrown::{HashMap, HashSet};
use regex::Regex;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Clone, Debug, Copy)]
pub struct Weight {
    pub freq: u32,
    pub mean: u32,
}
#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    // document id
    id: u32,
    //document title
    title: String,
    //word count
    count: u32,
    // token - weight
    index: HashMap<String, Weight>,
    #[serde(skip)]
    data: HashMap<String, HashSet<u32>>,
}
impl Document {
    // creates a new Document, and index immediatly its content using the provided tokenizer
    pub fn new(title: &str, body: &mut str, tokenizer: &mut Tokenizer) -> Document {
        let mut document: Document = Document {
            id: hash(title.as_bytes()),
            title: title.to_ascii_lowercase(),
            index: HashMap::default(),
            data: HashMap::default(),
            count: 0,
        };
        document.index_string(body, tokenizer);
        document.transform_data();
        return document;
    }

    // index text document UTF-8, into tokens, and their position on the document
    fn index_string(&mut self, text: &mut str, tokenizer: &mut Tokenizer) {
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

    // adds the token and its positions to a hashmap
    fn add_entry(&mut self, k: String, v: u32) {
        self.data.entry(k).or_default().insert(v);
    }
    // calculates the weight of the word : frequency in the document & the mean position
    fn transform_data(&mut self) {
        let s = self.data.iter().map(|(token, pos)| {
            let pos_vec: Vec<f32> = pos.into_iter().map(|x| *x as f32).collect();
            let weight = Weight {
                freq: pos_vec.len() as u32,
                mean: mean(&pos_vec) as u32,
            };
            (token.to_owned(), weight)
        });
        self.index = HashMap::from_iter(s);
    }

    // get document ID
    pub fn id(&self) -> u32 {
        self.id
    }

    // get document Title
    pub fn title(&self) -> String {
        self.title.to_owned()
    }

    // get document Index
    pub fn index(&self) -> HashMap<String, Weight> {
        self.index.to_owned()
    }

    // get document Word Count
    pub fn count(&self) -> u32 {
        self.count
    }
}
