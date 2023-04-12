#![allow(dead_code, unused)]
use std::{
    collections::{BTreeMap, BTreeSet, Bound, HashSet},
    fs,
    io::{read_to_string, Write},
    path::{Path, PathBuf},
};

use hnsw_rs::{dist::DistCosine, hnsw::Hnsw, hnsw::Point};

use webscout::{
    document::Document,
    index::Index,
    query::Query,
    tokenizer::{self, Tokenizer},
    utils::standard_deviation,
    WebScout,
};
fn main() {
    let mut map: Hnsw<f32, DistCosine> = Hnsw::new(24, 20000, 12, 400, DistCosine);
    map.insert((&vec![21.0, 32.2, 4.4], 1));
    map.insert((&vec![2.0, 2.2, 6.4], 2));
    map.insert((&vec![214.0, 322.2, 43.4], 3));
    let s = map.search(&[21.0, 32.2, 4.4], 10, 12);
    println!("{:?}", s);
}
