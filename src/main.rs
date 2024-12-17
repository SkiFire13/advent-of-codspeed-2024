use std::sync::OnceLock;

fn bench<D: std::fmt::Display>(mut f: impl FnMut() -> D) {
    let sol = f();
    let n = if cfg!(debug_assertions) { 1 } else { 100 };
    let now = std::time::Instant::now();
    for _ in 0..n {
        f();
    }
    println!("Took: {:?}", now.elapsed() / n);
    println!("Solution: {sol}");
}

static INPUT: OnceLock<String> = OnceLock::new();

macro_rules! run {
    ($day:ident, $part:ident) => {
        advent_of_codspeed_2024::$day::$part(INPUT.get_or_init(|| {
            std::fs::read_to_string(concat!("input/2024/", stringify!($day), ".txt")).unwrap()
        }))
    };
}

fn main() {
    bench(|| run!(day16, part1));
    bench(|| run!(day16, part2));
}
