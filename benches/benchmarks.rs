#![allow(unused)]
use std::{collections::HashSet, fs, time::Duration};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use webscout::tokenizer::{self, Tokenizer};

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
    group.bench_function("Auto Tokenize", |b| {
        b.iter(|| tokenizer.auto_tokenize("eplephant"))
    });
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
criterion_group! {
  name = benches;
  config = Criterion::default().measurement_time(Duration::from_secs(12));
  targets = benchmark_tokenizer
}

criterion_main!(benches);
