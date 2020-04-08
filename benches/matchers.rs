extern crate name_match;
use criterion::{criterion_group, criterion_main, Criterion};
use name_match::jaro::compare::JaroWinklerMatcher;
use name_match::soundex::compare::SoundexJaccardMatcher;
use name_match::prelude::*;

fn jaro_winkler(name1: &str, name2: &str) -> f64 {
    let matcher = JaroWinklerMatcher::default();
    matcher.get_weighted_score(name1, name2)

}

fn soundex_jaccard(name1: &str, name2: &str) -> f64{
    let matcher= SoundexJaccardMatcher::default();
    matcher.get_weighted_score(name1, name2)
}

fn criterion_benchmark(c: &mut Criterion) {
    let name1 = "John Doe Christopher";
    let name2 = "John Doe Adams";

    c.bench_function("jaro_winkler", |b| b.iter(|| jaro_winkler(name1, name2)));
    c.bench_function("soundex_jaccard", |b| b.iter(|| soundex_jaccard(name1, name2)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);