use crate::types::{Document, Lemmer};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
};

pub fn tokenize_string(word: &String, lemmer: &Lemmer) -> String {
    let mut lemma: String = String::new();
    let value = lemmer.map.get(word);
    if value.is_some() {
        lemma = value.unwrap().to_owned();
    }
    return lemma;
}
pub fn tokenize_document(document: &Document, lemmer: &Lemmer) -> HashMap<String, HashSet<usize>> {
    let mut map: HashMap<String, HashSet<usize>> = HashMap::new();
    for (token, positions) in document.index.clone() {
        let lemma = tokenize_string(&token, lemmer);
        map.insert(lemma, positions);
    }
    return map;
}

pub fn construct_lemmer(lemmas: String, lang: String) -> Lemmer {
    let mut map: HashMap<String, String> = HashMap::new();
    for mut line in lemmas.lines() {
        let lemma: Vec<&str> = line.split_whitespace().collect();
        if lemma.len() > 1 {
            map.insert(lemma[1].to_owned(), lemma[0].to_owned());
        }
    }
    return Lemmer {
        lang: lang,
        map: map,
    };
}
pub fn get_lemmer(lang: String) -> Lemmer {
    let path = format!("output/packs/{:?}.pack", lang.to_lowercase());
    let bin = fs::read(path).unwrap();
    let lemmer: Lemmer = rmp_serde::from_slice(&bin).unwrap();
    return lemmer;
}
