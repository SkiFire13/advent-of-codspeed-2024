fn bench<D: std::fmt::Display>(mut f: impl FnMut() -> D) {
    if !cfg!(debug_assertions) {
        f();
        f();
    }
    let now = std::time::Instant::now();
    let sol = f();
    println!("Took: {:?}", now.elapsed());
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
    bench(|| run!(day6, part1));
    bench(|| run!(day6, part2));
}
