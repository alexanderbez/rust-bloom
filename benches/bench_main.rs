#[macro_use]
extern crate criterion;

use criterion::Criterion;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use rsbloom::BloomFilter;

fn random_str(len: usize) -> String {
    thread_rng().sample_iter(&Alphanumeric).take(len).collect()
}

fn populate_bloom_filter(bf: &mut BloomFilter, n: u64) {
    for _ in 0..n {
        let item = random_str(30);
        bf.set(&item);
    }
}

fn bench_bloom_filter_set(c: &mut Criterion) {
    c.bench_function("set_size_1000", |b| {
        let mut bf = BloomFilter::new(1000);

        b.iter(|| {
            let item = random_str(30);
            bf.set(&item);
        });
    });

    c.bench_function("set_size_10000", |b| {
        let mut bf = BloomFilter::new(10000);

        b.iter(|| {
            let item = random_str(30);
            bf.set(&item);
        });
    });

    c.bench_function("set_size_50000", |b| {
        let mut bf = BloomFilter::new(50000);

        b.iter(|| {
            let item = random_str(30);
            bf.set(&item);
        });
    });
}

fn bench_bloom_filter_has(c: &mut Criterion) {
    c.bench_function("has_size_1000", |b| {
        let n = 1000;
        let mut bf = BloomFilter::new(n);
        populate_bloom_filter(&mut bf, n);

        b.iter(|| {
            let item = random_str(30);
            bf.has(&item);
        });
    });

    c.bench_function("has_size_10000", |b| {
        let n = 10000;
        let mut bf = BloomFilter::new(n);
        populate_bloom_filter(&mut bf, n);

        b.iter(|| {
            let item = random_str(30);
            bf.has(&item);
        });
    });

    c.bench_function("has_size_50000", |b| {
        let n = 50000;
        let mut bf = BloomFilter::new(n);
        populate_bloom_filter(&mut bf, n);

        b.iter(|| {
            let item = random_str(30);
            bf.has(&item);
        });
    });
}

criterion_group!(benches, bench_bloom_filter_set, bench_bloom_filter_has);
criterion_main!(benches);
