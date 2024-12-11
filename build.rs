use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(avx512_available)");
    if is_x86_feature_detected!("avx512vl") {
        println!("cargo::rustc-cfg=avx512_available");
    }

    make_d11_lut(
        25,
        4,
        &Path::new(&std::env::var("OUT_DIR").unwrap()).join("d11p1.lut"),
    );
    make_d11_lut(
        75,
        8,
        &Path::new(&std::env::var("OUT_DIR").unwrap()).join("d11p2.lut"),
    );
}

fn make_d11_lut(iters: usize, bytes: usize, path: &Path) {
    if path.exists() {
        let metadata = std::fs::metadata(path).unwrap();
        if metadata.len() as usize == bytes * 10_000_000 {
            return;
        }
    }

    let mut levels = vec![HashMap::new(); iters];

    fn solve_rec(i: usize, j: usize, levels: &mut [HashMap<usize, usize>]) -> usize {
        if i == 0 {
            return 1;
        }

        if let Some(&res) = levels[i - 1].get(&j) {
            return res;
        }

        let res = if j == 0 {
            solve_rec(i - 1, 1, levels)
        } else if j.ilog10() % 2 == 1 {
            let pow10 = 10usize.pow((j.ilog10() + 1) / 2);
            solve_rec(i - 1, j / pow10, levels) + solve_rec(i - 1, j % pow10, levels)
        } else {
            solve_rec(i - 1, j * 2024, levels)
        };

        levels[i - 1].insert(j, res);

        res
    }

    let mut out = BufWriter::new(File::create(path).unwrap());
    for j in 0..10_000_000 {
        let n = solve_rec(iters, j, &mut levels);
        out.write(&n.to_ne_bytes()[..bytes]).unwrap();
    }
    out.flush().unwrap();
}
