#![allow(unused)]
use std::{collections::HashSet, fs, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use webscout::{
    document::Document,
    tokenizer::{self, Tokenizer},
};

pub fn benchmark_tokenizer(c: &mut Criterion) {
    let pack = fs::read("packs/en.pack").unwrap();
    let mut tokenizer = Tokenizer::from_pack(&pack);
    c.bench_function("Tokenization", |b| {
        b.iter(|| tokenizer.tokenize("elephant"))
    });
}

pub fn benchmark_document(c: &mut Criterion) {
    let pack = fs::read("packs/en.pack").unwrap();
    let mut tokenizer = Tokenizer::from_pack(&pack);
    let mut doc = fs::read_to_string("assets/books/Alcott-1.txt").unwrap();
    c.bench_function("Document Indexing", |b| {
        b.iter(|| {
            Document::new("Albott", "albott", &mut doc, "en", &mut tokenizer);
        });
    });
}
criterion_group! {
  name = benches;
  config = Criterion::default().measurement_time(Duration::from_secs(5));
  targets = benchmark_document, benchmark_tokenizer
}

criterion_main!(benches);
