fn bench<D: std::fmt::Display>(mut f: impl FnMut() -> D) {
    let n = if cfg!(debug_assertions) { 1 } else { 1000 };
    let now = std::time::Instant::now();
    for _ in 0..n - 1 {
        f();
    }
    let sol = f();
    println!("Took: {:?}", now.elapsed() / n);
    println!("Solution: {sol}");
}

macro_rules! run {
    ($day:ident, $part:ident) => {
        advent_of_codspeed_2024::$day::$part(include_str!(concat!(
            "../input/2024/",
            stringify!($day),
            ".txt"
        )))
    };
}

fn main() {
    bench(|| run!(day13, part1));
    bench(|| run!(day13, part2));
}
