use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! day_bench {
    ($day:ident) => {
        pub fn $day(c: &mut Criterion) {
            let input = black_box(include_str!(concat!(
                "../../input/2024/",
                stringify!($day),
                ".txt"
            )));
            c.bench_function(concat!(stringify!($day), " part 1"), |b| {
                b.iter(|| advent_of_code::year2024::$day::part1(input))
            });
            c.bench_function(concat!(stringify!($day), " part 2"), |b| {
                b.iter(|| advent_of_code::year2024::$day::part2(input))
            });
        }
    };
}

day_bench!(day1);
// day_bench!(day2);

criterion_group!(
    name = year2024;
    config = Criterion::default();
    targets = day1
);

// Entry point for Criterion
criterion_main!(year2024);
