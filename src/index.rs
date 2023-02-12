use crate::document::{self, Document};
use crc32fast::hash;
use serde::{Deserialize, Serialize, __private::doc};
use std::{
    any::Any,
    collections::{HashMap, HashSet},
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default)]
pub struct Index {
    pub id: String,
    pub documents: HashSet<u32>,
    // Map? Token -> Map? Document -> (freq, mean, deviation)
    pub map: HashMap<String, HashMap<u32, (usize, usize, usize)>>,
}
impl Index {
    pub fn new() -> Index {
        Index {
            id: Uuid::new_v4().to_string(),
            documents: HashSet::new(),
            map: HashMap::new(),
        }
    }
    pub fn add_document(&mut self, document: &Document) {
        self.documents.insert(document.id.to_owned());
        for (token, positions) in &document.index {
            self.map.entry(token.to_owned());
        }
    }
    fn from(bin: &Vec<u8>) -> Index {
        let index: Index = rmp_serde::decode::from_slice(bin).unwrap();
        return index;
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
