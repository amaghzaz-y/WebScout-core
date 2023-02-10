#![allow(dead_code, unused)]

use fuse::{
    document::Document,
    index::Index,
    tokenizer::{self, Tokenizer},
};
use std::{
    fs,
    io::read_to_string,
    path::{Path, PathBuf},
};
fn main() {
    let folder = fs::read_dir("assets/books").unwrap();
    let mut idx = Index::new();
    for entry in folder {
        let name = &entry.as_ref().unwrap().file_name().into_string().unwrap();
        let path = &entry.unwrap().path();
        let file = fs::read_to_string(path).unwrap();
        println!("Parsing ::{:}", name.to_owned());
        let doc: Document = Document::new(file, "en".to_owned());
        println!("Adding to index ::{:}", name.to_owned());
        idx.add_document(&doc);
    }
    println!("serializing index");
    fs::write("packs/index/index.pack", idx.serialize());
    fs::write("packs/index/index.json", idx.to_json());
}
