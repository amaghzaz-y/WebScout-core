#![allow(unused)]
use std::{collections::HashSet, fs, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use webscout::{
    document::Document,
    tokenizer::{self, Tokenizer},
};

pub fn benchmark_tokenizer(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tokenizer");
    let pack = fs::read("packs/en.pack").unwrap();
    let lemmer = fs::read_to_string("assets/lemmers/en").unwrap();
    let tokenizer = Tokenizer::from_pack(&pack);
    // group.bench_function("get prefix & suffix", |b| {
    //     b.iter(|| tokenizer.get_prefix_suffix(&HashSet::from(["motherfucker".to_owned()]), (3, 2)));
    // });
    // group.bench_function("Query", |b| {
    //     b.iter(|| tokenizer.filter(&HashSet::from([("hom".to_owned(), "gir".to_owned())])))
    // });
    // group.bench_function("Auto Tokenize", |b| {
    //     b.iter(|| tokenizer.auto_tokenize("eplephant"))
    // });
    // group.bench_function("contruct en pack", |b| {
    //     b.iter(|| Tokenizer::new("en").construct_tokens(&lemmer));
    // });
    // group.bench_function("pack Deserialize", |b| {
    //     b.iter(|| Tokenizer::from_pack(&pack));
    // });
    // group.bench_function("pack serialize", |b| {
    //     b.iter(|| tokenizer.to_pack());
    // });
}

pub fn benchmark_document(c: &mut Criterion) {
    let mut group = c.benchmark_group("Document");
    let pack = fs::read("packs/en.pack").unwrap();
    let mut tokenizer = Tokenizer::from_pack(&pack);
    let doc = fs::read_to_string("assets/books/Alcott-1.txt").unwrap();
    // let map =
    //     Document::new("Albott".to_string(), doc.to_owned(), "en".to_owned()).index_string(&doc);
    // let tokenized =
    //     Document::new("Albott".to_string(), doc.to_owned(), "en".to_owned()).index(&doc);
    group.bench_function("auto index", |b| {
        b.iter(|| {
            Document::new(
                "Albott".to_string(),
                doc.to_owned(),
                "en".to_owned(),
                &mut tokenizer,
            );
        });
    });
    // group.bench_function("indexing", |b| {
    //     b.iter(|| {
    //         Document::new("Albott".to_string(), doc.to_owned(), "en".to_owned()).index(&doc);
    //     });
    // });
    // group.bench_function("tokenizing", |b| {
    //     b.iter(|| {
    //         Document::new("Albott".to_string(), doc.to_owned(), "en".to_owned()).tokenize(&map);
    //     });
    // });
    // group.bench_function("transform", |b| {
    //     b.iter(|| {
    //         Document::new("Albott".to_string(), doc.to_owned(), "en".to_owned())
    //             .transform_map(&tokenized);
    //     });
    // });
}
criterion_group! {
  name = benches;
  config = Criterion::default().measurement_time(Duration::from_secs(5));
  targets = benchmark_document
}

criterion_main!(benches);
