#![allow(dead_code, unused)]
use std::{
    collections::{BTreeMap, BTreeSet, Bound, HashSet},
    fs,
    io::{read_to_string, Write},
    path::{Path, PathBuf},
};

use webscout::{
    document::Document,
    index::Index,
    query::Query,
    tokenizer::{self, Tokenizer},
    utils::standard_deviation,
    WebScout,
};

fn main() {
    println!("hello");
}
