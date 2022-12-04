use criterion::{criterion_group, criterion_main, Criterion};
use aoc22::util;
use aoc22::*;

pub fn day01_benches(c: &mut Criterion) {
    let input = util::get_input(1);
    c.bench_function("Day01.1: ", |b| b.iter(|| day01::solve_1(&input)));
    c.bench_function("Day01.2: ", |b| b.iter(|| day01::solve_2(&input)));
}

pub fn day02_benches(c: &mut Criterion) {
    let input = util::get_input(2);
    c.bench_function("Day02.1: ", |b| b.iter(|| day02::solve_1(&input)));
    c.bench_function("Day02.2: ", |b| b.iter(|| day02::solve_2(&input)));
}

pub fn day03_benches(c: &mut Criterion) {
    let input = util::get_input(3);
    c.bench_function("Day03.1: ", |b| b.iter(|| day03::solve_1(&input)));
    c.bench_function("Day03.2: ", |b| b.iter(|| day03::solve_2(&input)));
}

pub fn day04_benches(c: &mut Criterion) {
    let input = util::get_input(4);
    c.bench_function("Day04.1: ", |b| b.iter(|| day04::solve_1(&input)));
    c.bench_function("Day04.2: ", |b| b.iter(|| day04::solve_2(&input)));
    c.bench_function("Day04.1 threaded: ", |b| b.iter(|| day04::solve_1_parallel(&input, 10)));
}

criterion_group!(benches, day04_benches);
criterion_main!(benches);
