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
// fn parse_chars(document: &mut Document) {
//     let mut keys = document.content.split_whitespace().into_iter();
//     let words: Vec<String> = vec![];
//     for key in keys {
//         document.chars.push(key.to_owned().to_lowercase())
//     }
// }
// #[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]

// struct Token {
//     value: String,
// }
// #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
// struct Value {
//     freq: u32,
//     spots: HashMap<u32, u32>,
// }
// #[derive(Debug, Serialize, Deserialize)]

// struct Index {
//     docs: HashMap<String, i32>,
//     chars: HashMap<String, HashSet<i32>>,
//     table: HashMap<Token, Value>,
// }
// fn group_by_frequency(arr: &[i32]) -> HashMap<i32, i32> {
//     let mut frequency_map = HashMap::new();
//     for &x in arr {
//         *frequency_map.entry(x).or_insert(0) += 1;
//     }
//     return frequency_map;
// }
// impl Index {
//     fn add_document(&mut self, document: &mut Document) {
//         self.docs.push((document.title.to_owned(), document.id));
//         for key in document.chars.clone() {
//             if self.chars.contains_key(&key) {
//                 let key_entry = self.chars.entry(key).or_default();
//                 key_entry.insert(document.id);
//             } else {
//                 self.chars.insert(key.to_owned(), HashSet::default());
//                 let key_entry = self.chars.entry(key).or_default();
//                 key_entry.insert(document.id);
//             }
//         }
//     }
//     fn search_by_char(&self, char: String) -> &HashSet<i32> {
//         let result = self.chars.get(&char).unwrap();
//         return result;
//     }
//     fn raw_search(&self, query: &str) -> Vec<i32> {
//         let chars = query.split_whitespace().into_iter();
//         let mut docs_id: Vec<i32> = vec![];
//         for mut char in chars {
//             if self.chars.contains_key(&char.to_owned().to_lowercase()) {
//                 let docs = self
//                     .chars
//                     .get(&char.to_owned().to_lowercase())
//                     .unwrap()
//                     .to_owned();
//                 for id in docs {
//                     docs_id.push(id);
//                 }
//             }
//         }
//         return docs_id;
//     }
//     fn group_raw_search(&self, rsearch: &mut Vec<i32>) -> Vec<(i32, i32)> {
//         let frequency_map = group_by_frequency(&rsearch);
//         let result = frequency_map
//             .iter()
//             .map(|(k, v)| (*k, *v))
//             .collect::<Vec<(i32, i32)>>();
//         return result;
//     }
//     fn evaluate_result(&self, result: &mut Vec<(i32, i32)>) -> Vec<(i32, i32)> {
//         result.sort_by_cached_key(|&(a, b)| b);
//         let mut max: i32 = result.last().unwrap().1;
//         let mut evaluation: Vec<(i32, i32)> = vec![];
//         for res in result {
//             let ratio = res.1 * 100 / max;
//             evaluation.push((res.0, ratio));
//         }
//         evaluation.sort_by_cached_key(|&(a, b)| b);
//         println!("evaluation : {:?}", evaluation);
//         return evaluation;
//     }
//     fn search_with_retrieval(&mut self, eval: &mut Vec<(i32, i32)>) {
//         eval.reverse();
//         for elem in eval {}
//     }
// }
fn main() {
    let mut docs = read_docs();
    let mut ws = WebScout::new();

    for doc in docs {
        let mut tokens = ws.parse_body(&doc);
        ws.add_document(&doc);
        ws.index_tokens(&tokens, &doc);
    }
    let bin = bincode::serialize(&ws).unwrap();
    let yaml = serde_yaml::to_string(&ws).unwrap();
    fs::write("ws.bin", bin);
    fs::write("ws.yml", yaml);
}
