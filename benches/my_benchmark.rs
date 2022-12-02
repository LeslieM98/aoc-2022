use criterion::{black_box, criterion_group, criterion_main, Criterion};
use aoc::day01;
use aoc::util;

pub fn day01(c: &mut Criterion) {
    let input = util::get_input(1);
    c.bench_function("Day01.1 SingleThread", |b| b.iter(|| day01::solve_1_single_thread(&input)));
}

criterion_group!(benches, day01);
criterion_main!(benches);