use crate::types::{Document, Lemmer};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

pub fn tokenize_string(mut word: String, lemmer: &Lemmer) {
    let lemma = lemmer.map.get(&word);
    if lemma.is_some() {
        word = lemma.unwrap().to_owned();
    }
    return;
}
pub fn tokenize_document(mut document: Document, lemmer: &Lemmer) {
    for (token, _) in document.index {
        tokenize_string(token, lemmer);
    }
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
