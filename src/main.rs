#![allow(dead_code, unused)]
use flate2::{write::GzEncoder, Compression};
use std::{
    fs,
    io::{read_to_string, Write},
    path::{Path, PathBuf},
};
use webscout::{
    document::Document,
    index::Index,
    query::Query,
    tokenizer::{self, Tokenizer},
};
// fn serialize_docs() {
//     let dir = fs::read_dir("assets/books").unwrap();
//     let mut idx: Index = Index::new();
//     for file in dir {
//         let path = file.as_ref().unwrap().path().to_owned();
//         let name = file.as_ref().unwrap().file_name().into_string().unwrap();
//         let body = fs::read_to_string(path).unwrap();
//         println!("indexing {:}", name);
//         let doc: Document = Document::new(name.to_owned(), body, "en".to_string());
//         println!("adding document {:}", name);
//         idx.add_document(&doc);
//         println!("serializing {:}", name);
//         let content = doc.serialize();
//         println!("saving {:}", name);
//         fs::write(format!("packs/docs/{}.pack", name), content);
//     }
//     println!("saving index");
//     let mut cmp = GzEncoder::new(Vec::new(), Compression::default());
//     cmp.write_all(&idx.serialize());
//     let bin = cmp.finish().unwrap();
//     fs::write("packs/index/index.f2", bin);
//     fs::write("packs/index/index.pack", idx.serialize());
// }

fn main() {
    //serialize_docs();
    // let bin = fs::read("packs/index/index.pack").unwrap();
    // let index: Index = rmp_serde::decode::from_slice(&bin).unwrap();
    // let mut query = Query::new(index, "MARY ROBERTS RINEHART".to_string(), "en".to_string());
    // query.evaluate();
}
