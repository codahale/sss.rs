#![feature(collections, test)]

extern crate rand;
extern crate sss;
extern crate test;

use std::collections::HashMap;
use test::Bencher;

#[bench]
fn bench_combine(b: &mut Bencher) {
    let mut expected: HashMap<u8, Vec<u8>> = HashMap::new();
    expected.insert(1, vec![118, 163, 66, 80, 187]);
    expected.insert(2, vec![239, 91, 129, 172, 98]);
    expected.insert(3, vec![152, 250, 192, 248, 220]);

    b.iter(|| sss::combine(&expected))
}

#[bench]
fn bench_split(b: &mut Bencher) {
    let mut rng = rand::ChaChaRng::new_unseeded();
    let input = vec![1, 2, 3, 4, 5];

    b.iter(|| sss::split(5, 3, &input, &mut rng))
}
