use crate::document::{Document, Weight};
use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Index {
    id: String,
    count: u32,
    documents: HashMap<u32, (String, u32)>,
    map: HashMap<String, HashMap<u32, Weight>>,
}

impl Index {
    pub fn new() -> Index {
        Index {
            id: Uuid::new_v4().to_string(),
            count: 0,
            documents: HashMap::new(),
            map: HashMap::new(),
        }
    }

    pub fn add_document(&mut self, document: &Document) {
        self.documents
            .insert(document.id(), (document.title(), document.count()));
        for (token, positions) in &document.index() {
            let doc_name = document.id();
            let stats: Weight = positions.to_owned();

            let map = HashMap::from([(doc_name, stats)]);

            self.map
                .entry(token.to_owned())
                .or_insert(map)
                .insert(doc_name, stats);
        }
    }

    pub fn get_title(&self, id: &u32) -> String {
        self.documents.get(id).unwrap().0.to_owned()
    }

    pub fn get(&self, token: &str) -> Option<&HashMap<u32, Weight>> {
        self.map.get(token)
    }

    pub fn from(bin: &Vec<u8>) -> Index {
        rmp_serde::decode::from_slice(bin).unwrap()
    }

    pub fn serialize(&self) -> Vec<u8> {
        rmp_serde::encode::to_vec(self).unwrap()
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(self).unwrap()
    }
}
