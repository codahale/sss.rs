extern crate criterion;
extern crate rand;
extern crate rand_chacha;
extern crate sss;

use std::collections::HashMap;

use criterion::{criterion_group, criterion_main, Criterion};
use rand::SeedableRng;
use rand_chacha::ChaChaRng;

fn combine(c: &mut Criterion) {
    let mut expected: HashMap<u8, Vec<u8>> = HashMap::new();
    expected.insert(1, vec![118, 163, 66, 80, 187]);
    expected.insert(2, vec![239, 91, 129, 172, 98]);
    expected.insert(3, vec![152, 250, 192, 248, 220]);

    c.bench_function("combine", move |b| b.iter(|| sss::combine(&expected)));
}

fn split(c: &mut Criterion) {
    let input = vec![1, 2, 3, 4, 5];
    let mut rng = ChaChaRng::from_seed([0u8; 32]);

    c.bench_function("split", move |b| {
        b.iter(|| sss::split(5, 3, &input, &mut rng))
    });
}

criterion_group!(benches, combine, split);
criterion_main!(benches);
