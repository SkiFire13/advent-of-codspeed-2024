use codspeed_criterion_compat::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! bench {
    ($day:ident) => {
        use advent_of_codspeed_2024::$day::{part1, part2};

        fn bench(c: &mut Criterion) {
            let mut g = c.benchmark_group(stringify!($day));

            let input = include_str!(concat!("./", stringify!($day), ".txt"));
            g.bench_function("part1", |b| b.iter(|| part1(black_box(input))));
            g.bench_function("part2", |b| b.iter(|| part2(black_box(input))));
        }
    };
}

bench!(day16);

criterion_group!(benches, bench);
criterion_main!(benches);
