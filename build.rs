use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(avx512_available)");
    if is_x86_feature_detected!("avx512vl") {
        println!("cargo::rustc-cfg=avx512_available");
    }

    // let lutd11p1 = Path::new(&std::env::var("OUT_DIR").unwrap()).join("d11p1.lut");
    // make_d11_lut(25, 4, &lutd11p1);
    // let lutd11p2 = Path::new(&std::env::var("OUT_DIR").unwrap()).join("d11p2.lut");
    // make_d11_lut(75, 8, &lutd11p2);
}

#[allow(unused)]
fn make_d11_lut(iters: usize, bytes: usize, path: &Path) {
    if path.exists() {
        let metadata = std::fs::metadata(path).unwrap();
        if metadata.len() as usize == bytes * 10_000_000 {
            return;
        }
    }

    let mut levels = vec![[0; 1000]; iters];

    fn solve_rec(i: usize, j: usize, levels: &mut [[usize; 1000]]) -> usize {
        if i == 0 {
            return 1;
        }

        if j < 1000 {
            let c = levels[i - 1][j];
            if c != 0 {
                return c;
            }
        }

        let res = if j == 0 {
            solve_rec(i - 1, 1, levels)
        } else if j.ilog10() % 2 == 1 {
            let pow10 = 10usize.pow((j.ilog10() + 1) / 2);
            solve_rec(i - 1, j / pow10, levels) + solve_rec(i - 1, j % pow10, levels)
        } else {
            solve_rec(i - 1, j * 2024, levels)
        };

        if j < 1000 {
            levels[i - 1][j] = res;
        }

        res
    }

    let mut out = BufWriter::new(File::create(path).unwrap());
    for j in 0..10_000_000 {
        let n = solve_rec(iters, j, &mut levels);
        out.write(&n.to_ne_bytes()[..bytes]).unwrap();
    }
    out.flush().unwrap();
}
