#![allow(unused)]
#![allow(dead_code)]
#![allow(unstable_features)]
use crc32fast::hash;
use serde::{Deserialize, Serialize, __private::doc};
use std::{
    char,
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    fs::{self, File},
    hash::{Hash, Hasher},
    io::{BufRead, BufReader, Read},
    process::id,
    time::Instant,
    vec,
};
use web_scout::{Document, WebScout};

fn serialize_lemtxt() {
    let mut files: Vec<(String, BufReader<File>)> = vec![];
    let dir = fs::read_dir("lemtxt").unwrap();
    for entry in dir {
        let file = entry.unwrap();
        let rd = fs::File::open(file.path()).unwrap();
        files.push((
            file.file_name().to_str().unwrap().to_owned(),
            BufReader::new(rd),
        ));
    }
    let mut lemmap: HashMap<String, String> = HashMap::new();
    for file in files {
        println!("serialzing :: {:?}", file.1);
        let mut map: HashMap<String, String> = HashMap::new();
        for line in file.1.lines() {
            let mut line = line.unwrap();
            let lemma: Vec<&str> = line.split_whitespace().collect();
            if lemma.len() > 1 {
                map.insert(lemma[1].to_owned(), lemma[0].to_owned());
                lemmap.insert(lemma[1].to_owned(), lemma[0].to_owned());
            }
        }
        fs::write(
            format!("lembin/{:?}", file.0.to_owned()),
            bincode::serialize(&map).unwrap(),
        );
    }
    fs::write("lembin/lembin", bincode::serialize(&lemmap).unwrap());
}
fn read_docs() -> Vec<Document> {
    let mut docs: Vec<Document> = vec![];
    let dir = fs::read_dir("data").unwrap();
    for entry in dir {
        let file = entry.unwrap();
        docs.push(Document {
            title: file.file_name().to_str().unwrap().to_owned(),
            body: fs::read_to_string(file.path()).unwrap(),
        });
    }
    return docs;
}
fn main() {
    let mut docs = read_docs();
    let mut ws = WebScout::new();
    let lembin = fs::read("lembin/en.bin").unwrap();
    let lemmer: HashMap<String, String> = bincode::deserialize(&lembin).unwrap();
    let mut top_timer = Instant::now();
    for mut doc in docs {
        let mut tokens = ws.parse_body(&mut doc);
        ws.tokenize(&lemmer, &mut tokens);  
        ws.add_document(&doc);
        ws.index_tokens(&tokens, &doc);
    }
    let bin = bincode::serialize(&ws).unwrap();
    let yaml = serde_yaml::to_string(&ws).unwrap();
    fs::write("ws.bin", bin);
    fs::write("ws.yml", yaml);
    let lembin = fs::read("lembin/en.bin").unwrap();
    let lemmer: HashMap<String, String> = bincode::deserialize(&lembin).unwrap();
    let data = fs::read("ws.bin").unwrap();
    let nws = WebScout::from_binary(data);
    println!("search :::");
    nws.search(
        "COPYRIGHT 1990, JIM PRENTICE, BRANDON, MANITOBA, CANADA",
        &lemmer,
    );
}
