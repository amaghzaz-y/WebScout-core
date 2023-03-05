#![allow(unused)]
use std::{collections::HashSet, fs, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use webscout::{document::Document, index::Index, query::Query, tokenizer::Tokenizer};

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
            Document::new("Albott", &mut doc, &mut tokenizer);
        });
    });
}

pub fn benchmark_query(c: &mut Criterion) {
    let idx_bin = fs::read("temp/index/index.pack").unwrap();
    let index: Index = rmp_serde::from_slice(&idx_bin).unwrap();
    let pack = fs::read("packs/en.pack").unwrap();
    let mut tokenizer = Tokenizer::from_pack(&pack);
    let mut doc = fs::read_to_string("assets/books/Alcott-1.txt").unwrap();
    let mut query = Query::new(&index, &tokenizer);
    c.bench_function("Query", |b| {
        b.iter(|| {
            let s = query.search("hello-world-fire-fly");
            query.above_average(s.0, s.1);
        });
    });
}
criterion_group! {
  name = benches;
  config = Criterion::default().measurement_time(Duration::from_secs(5));
  targets = benchmark_query
}

criterion_main!(benches);
