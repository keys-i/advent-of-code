use advent_of_code::year2024::part1;
use criterion::{criterion_group, criterion_main, Criterion};

const SAMPLE: &str = indoc! {"
    3   4
    4   3
    2   5
    1   3
    3   9
    3   3
"};

fn benchmark_part1(c: &mut Criterion) {
    c.bench_function("part1", |b| b.iter(|| part1(black_box(SAMPLE))));
}

fn benchmark_part2(c: &mut Criterion) {
    c.bench_function("part2", |b| b.iter(|| part2(black_box(SAMPLE))));
}

criterion_group!(benches, benchmark_part1, benchmark_part2);
criterion_main!(benches);
