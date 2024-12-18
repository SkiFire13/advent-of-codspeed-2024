use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

fn main() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rustc-check-cfg=cfg(avx512_available)");
    if is_x86_feature_detected!("avx512vl") {
        println!("cargo::rustc-cfg=avx512_available");
    }

    // let lutd11p1 = Path::new(&std::env::var("OUT_DIR").unwrap()).join("d11p1.lut");
    // make_d11_lut(25, 4, &lutd11p1);
    // let lutd11p2 = Path::new(&std::env::var("OUT_DIR").unwrap()).join("d11p2.lut");
    // make_d11_lut(75, 8, &lutd11p2);
    let lutd17p2 = Path::new(&std::env::var("OUT_DIR").unwrap()).join("d17p2.lut");
    make_d17_lut(&lutd17p2);
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

#[allow(unused)]
fn make_d17_lut(path: &Path) {
    fn simulate(mut regs: [u64; 3], prog: &[u8], out: &mut Vec<u8>) {
        let mut ip = 0;
        while ip < prog.len() {
            let instr = prog[ip];
            let op = prog[ip + 1];
            let combo_op = || match op {
                0..=3 => op,
                4..=6 => regs[op as usize - 4] as u8,
                _ => unreachable!(),
            };
            match instr {
                0 => regs[0] >>= combo_op(),
                1 => regs[1] ^= op as u64,
                2 => regs[1] = combo_op() as u64 & 0b111,
                3 => {
                    if regs[0] != 0 {
                        ip = op as usize;
                        continue;
                    }
                }
                4 => regs[1] ^= regs[2],
                5 => out.push(combo_op() & 0b111),
                6 => regs[1] = regs[0] >> combo_op(),
                7 => regs[2] = regs[0] >> combo_op(),
                _ => unreachable!(),
            }
            ip += 2;
        }
    }

    // let mut f = std::io::BufWriter::new(File::create("manual.txt").unwrap());

    let mut lut = vec![0u64; (5 * 5 * 5) * (8 * 8 * 8 * 8)];

    let mut out = Vec::new();
    let mut i = [usize::MAX; 4];

    let map1 = [0, 1, 4, 5];
    let map2 = [3, 9, 9, 5];

    let ops = [0, 1, 4, 5];
    let arg_lo = [3, 0, 0, 5];
    let arg_hi = [4, 8, 8, 6];

    macro_rules! unnest {
        ({$($block:tt)*}) => { $($block)* };
        ([$($op:tt)*] $($rest:tt)+) => { $($op)* { unnest!($($rest)+); } };
    }

    unnest! {
        [for x1 in 0..8]
        [for o1 in 0..4]
        [for o2 in 0..4]
        [if o2 != o1]
        [for o3 in 0..4]
        [if o3 != o1 && o3 != o2]
        [let o4 = o1 ^ o2 ^ o3;]
        [if o4 == 0 || o4 == 3]
        [for a1 in arg_lo[o1 as usize]..arg_hi[o1 as usize]]
        [for a2 in arg_lo[o2 as usize]..arg_hi[o2 as usize]]
        [for a3 in arg_lo[o3 as usize]..arg_hi[o3 as usize]]
        [let a4 = arg_lo[o4 as usize];]
        {
            let [o1, o2, o3, o4] = [o1, o2, o3, o4].map(|o| ops[o as usize]);
            let mut prog = [2, 4, 1, x1, 7, 5, o1, a1, o2, a2, o3, a3, o4, a4, 3, 0];

            let mut offset = 0;
            offset = 0 * offset + x1 as usize;
            offset = 8 * offset + o1 as usize;
            offset = 5 * offset + a1 as usize;
            offset = 8 * offset + o2 as usize;
            offset = 5 * offset + a2 as usize;
            offset = 8 * offset + o3 as usize;
            offset = 5 * offset + a3 as usize;

            let mut new_a = 0;

            'search: for _ in 0..1000 {
                out.clear();
                simulate([new_a, 0, 0], &prog, &mut out);

                if prog.ends_with(&out) {
                    if prog.len() == out.len() {
                        lut[offset] = new_a;
                        // f.write_fmt(format_args!("    lut[{offset}] = {new_a};\n")).unwrap();
                        break 'search;
                    } else {
                        new_a <<= 3;
                    }
                } else {
                    while new_a & 0b111 == 0b111 {
                        new_a >>= 3;
                        if new_a == 0 {
                            break 'search;
                        }
                    }
                    new_a += 1;
                }
            }
        }
    }
    // f.flush().unwrap();

    let lut_u8 = unsafe { std::slice::from_raw_parts(lut.as_ptr().cast::<u8>(), 8 * lut.len()) };
    std::fs::write(path, lut_u8).unwrap();
}
