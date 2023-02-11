#![allow(dead_code, unused)]

use fuse::{
    document::Document,
    index::Index,
    query::Query,
    tokenizer::{self, Tokenizer},
};
use std::{
    fs,
    io::read_to_string,
    path::{Path, PathBuf},
};
fn main() {
    let bin = fs::read("packs/index/index.pack").unwrap();
    let index: Index = rmp_serde::decode::from_slice(&bin).unwrap();
    let mut query = Query::new(index, "mother".to_string(), "en".to_string());
    query.normalize();
}
