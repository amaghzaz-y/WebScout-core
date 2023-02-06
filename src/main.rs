#![allow(unused)]
#![allow(dead_code)]
#![allow(unstable_features)]
use crc32fast::hash;
use fuse::{Document, Fuse};
use serde::{Deserialize, Serialize, __private::doc};
use std::{
    char,
    collections::{hash_map::DefaultHasher, HashMap, HashSet},
    fs::{self, File},
    hash::{Hash, Hasher},
    io::{BufRead, BufReader, Read},
    process::id,
    time::{self, Instant},
    vec,
};
fn serialize_lemtxt() {
    let mut files: Vec<(String, BufReader<File>)> = vec![];
    let dir = fs::read_dir("lemmers/src").unwrap();
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
        println!("serialzing :: {:?}", file.0);
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
            format!(
                "lemmers/bin/{:?}",
                file.0.to_owned().trim_end_matches(".txt")
            ),
            bincode::serialize(&map).unwrap(),
        );
        fs::write(
            format!(
                "lemmers/packs/{:?}",
                file.0.to_owned().trim_end_matches(".txt")
            ),
            rmp_serde::encode::to_vec(&map).unwrap(),
        );
    }
    let mut now = time::Instant::now();
    fs::write("lemmers/map/map.bin", bincode::serialize(&lemmap).unwrap());
    println!("bin elapsed: {:?}", now.elapsed().as_millis());
    now = time::Instant::now();
    fs::write(
        "lemmers/map/map.mpack",
        rmp_serde::encode::to_vec(&lemmap).unwrap(),
    );
    println!("rmp elapsed: {:?}", now.elapsed().as_millis());
}
fn read_docs() -> Vec<Document> {
    let mut docs: Vec<Document> = vec![];
    let dir = fs::read_dir("books").unwrap();
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
    let mut ws = Fuse::new();
    let lemrmp = fs::read("lemmers/packs/en.mpk").unwrap();
    let lemmer: HashMap<String, String> = rmp_serde::decode::from_slice(&lemrmp).unwrap();
    // for mut doc in docs {
    //     ws.add_document(&lemmer, &mut doc);
    // }
    // let yaml = serde_yaml::to_string(&ws).unwrap();
    // let index = rmp_serde::encode::to_vec(&ws).unwrap();
    // fs::write("ws.yml", yaml);
    // fs::write("index.mpk", index);
    let data = fs::read("index.mpk").unwrap();
    let nws = Fuse::from_pack(data);
    println!("search :::");
    nws.search("You mean Mr. Duncan, the president of the bank?", &lemmer);
}
