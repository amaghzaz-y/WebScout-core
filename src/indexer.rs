use crate::types::{Document, Index};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;
fn new() -> Index {
    Index {
        id: Uuid::new_v4().to_string(),
        documents: HashSet::new(),
        index: HashMap::new(),
    }
}
fn index_document(document: &Document, mut index: Index) {
    for (token, postions) in &document.index {
        if index.index.contains_key(token) {
            index
                .index
                .entry(token.to_owned())
                .or_insert(HashMap::from([(document.id.clone(), postions.to_owned())]))
                .entry(document.id.clone())
                .or_insert(postions.to_owned());
        }
    }
}
