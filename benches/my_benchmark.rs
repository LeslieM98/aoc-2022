use criterion::{criterion_group, criterion_main, Criterion};
use aoc22::util;
use aoc22::*;

pub fn day01_benches(c: &mut Criterion) {
    let input = util::get_input(1);
    c.bench_function("Day01.1: ", |b| b.iter(|| day01::solve_1(&input)));
    c.bench_function("Day01.2: ", |b| b.iter(|| day01::solve_2(&input)));
}

pub fn day02_benches(c: &mut Criterion) {
    let input = util::get_input(1);
    c.bench_function("Day02.1: ", |b| b.iter(|| day02::solve_1(&input)));
    c.bench_function("Day02.2: ", |b| b.iter(|| day02::solve_2(&input)));
}

criterion_group!(benches, day02_benches);
criterion_main!(benches);
