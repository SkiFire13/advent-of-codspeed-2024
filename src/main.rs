use std::sync::OnceLock;

fn bench<D: std::fmt::Display>(mut f: impl FnMut() -> D) {
    let sol = f();
    let n = if cfg!(debug_assertions) { 1 } else { 10000 };
    let now = std::time::Instant::now();
    for _ in 0..n {
        f();
    }
    println!("Took: {:?}", now.elapsed() / n);
    println!("Solution: {sol}");
}

static INPUT: OnceLock<&'static str> = OnceLock::new();

macro_rules! run {
    ($day:ident, $part:ident) => {
        advent_of_codspeed_2024::$day::$part(INPUT.get_or_init(|| {
            #[cfg(not(miri))]
            {
                std::fs::read_to_string(concat!("input/2024/", stringify!($day), ".txt"))
                    .unwrap()
                    .leak()
            }

            #[cfg(miri)]
            {
                include_str!(concat!("../input/2024/", stringify!($day), ".txt"))
            }
        }))
    };
}

fn main() {
    bench(|| run!(day19, part1));
    bench(|| run!(day19, part2));
}
