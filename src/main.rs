use std::sync::OnceLock;

fn bench<D: std::fmt::Display>(mut f: impl FnMut() -> D) {
    rayon::broadcast(|_| {});

    let n = if cfg!(debug_assertions) { 1 } else { 100 };
    let now = std::time::Instant::now();
    for _ in 1..n {
        f();
    }
    let sol = f();
    println!("Took: {:?}", now.elapsed() / n);
    println!("Solution: {sol}");
}

macro_rules! run {
    ($day:ident) => {{
        static INPUT: OnceLock<&'static str> = OnceLock::new();
        let input = INPUT.get_or_init(|| {
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
        });

        bench(|| advent_of_codspeed_2024::$day::part1(input));
        run!(part2 $day bench(|| advent_of_codspeed_2024::$day::part2(input)));
    }};
    (part2 day25 $($rest:tt)*) => {};
    (part2 $day:ident $($rest:tt)*) => { $($rest)* };
}

fn main() {
    run!(day25)
}
