use day1::*;

use std::fs::File;

use criterion::{criterion_group, criterion_main, Criterion};

fn solution_a(c: &mut Criterion) {
    let f = File::open("input").unwrap();
    let nums = nums_from_readable(f).unwrap();
    c.bench_function("solution a", |bench| {
        bench.iter(|| {
            a(nums.to_owned())
        })
    });
}

fn solution_b(c: &mut Criterion) {
    let f = File::open("input").unwrap();
    let nums = nums_from_readable(f).unwrap();
    c.bench_function("solution b", |bench| {
        bench.iter(|| {
            b(nums.to_owned())
        })
    });
}

criterion_group!(benches, solution_a, solution_b);
criterion_main!(benches);
