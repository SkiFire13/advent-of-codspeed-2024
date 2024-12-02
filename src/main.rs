#![feature(portable_simd)]

fn bench<D: std::fmt::Display>(f: impl FnOnce() -> D) {
    let now = std::time::Instant::now();
    let sol = f();
    println!("Took: {:?}", now.elapsed());
    println!("Solution: {sol}");
}

macro_rules! run {
    ($day:ident, $part:ident) => {
        aoc::$day::$part(include_str!(concat!(
            "../input/2024/",
            stringify!($day),
            ".txt"
        )))
    };
}

fn main() {
    bench(|| run!(day2, part1));
    bench(|| run!(day2, part2));
}
