use crate::document::{self, Document, Weight};
extern crate alloc;
use crate::index::alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crc32fast::hash;
use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Index {
    pub id: String,
    pub documents: HashSet<u32>,
    pub map: HashMap<String, HashMap<u32, Weight>>,
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
            let doc_name = document.id;
            let stats: Weight = positions.to_owned();
            let map = HashMap::from([(doc_name, stats)]);
            self.map
                .entry(token.to_owned())
                .or_insert(map)
                .insert(doc_name, stats);
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
