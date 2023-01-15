#![allow(unused)]
#![allow(dead_code)]
#![allow(unstable_features)]
use crc32fast::hash;
use serde::{Deserialize, Serialize, __private::doc};
use std::{
    char,
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    fs,
    hash::{Hash, Hasher},
    process::id,
    time::Instant,
    vec,
};
use web_scout::{Document, WebScout};

fn read_docs() -> Vec<Document> {
    let mut docs: Vec<Document> = vec![];
    let dir = fs::read_dir("data").unwrap();
    let mut count = 0;
    for entry in dir {
        let file = entry.unwrap();
        docs.push(Document {
            title: file.file_name().to_str().unwrap().to_owned(),
            body: fs::read_to_string(file.path()).unwrap(),
        });
        count += 1;
    }
    return docs;
}

fn main() {
    let mut docs = read_docs();
    let mut ws = WebScout::new();
    let mut length = 0;
    let mut top_timer = Instant::now();
    for doc in docs {
        let mut now = Instant::now();
        let bin_body = doc.body.as_bytes().to_owned();
        println!("To bin: {:?}", now.elapsed().as_micros());
        println!("Document len : {:?}", doc.body.len());
        now = Instant::now();
        let mut tokens = ws.parse_body(&doc);
        println!("Parsing: {:?}", now.elapsed().as_micros());
        now = Instant::now();
        ws.add_document(&doc);
        println!("Adding Document: {:?}", now.elapsed().as_micros());
        now = Instant::now();
        ws.index_tokens(&tokens, &doc);
        println!("Indexing Document: {:?}", now.elapsed().as_micros());
        println!();
        length += doc.body.len();
    }
    let elapsed = top_timer.elapsed().as_millis();
    let mut now = Instant::now();
    let bin = bincode::serialize(&ws).unwrap();
    println!("Bincode Serialize: {:?}", now.elapsed().as_micros());
    now = Instant::now();
    let yaml = serde_yaml::to_string(&ws).unwrap();
    println!("Yaml Serialize: {:?}", now.elapsed().as_micros());
    fs::write("ws.bin", bin);
    fs::write("ws.yml", yaml);
    println!("Indexed {:?} words in {:?}", length, elapsed);
}
