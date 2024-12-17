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

    let mut lut = vec![0u64; (4 * 4 * 4 * 4) * (8 * 8 * 8)];

    let mut out = Vec::new();
    let mut i = [usize::MAX; 4];

    let map1 = [0, 1, 4, 5];
    let map2 = [3, 9, 9, 5];

    for c1 in 0..4 {
        i[c1] = 0;
        for c2 in 0..4 {
            if c1 == c2 {
                continue;
            }
            i[c2] = 1;

            for c3 in 0..4 {
                if c1 == c3 || c2 == c3 {
                    continue;
                }
                i[c3] = 2;

                for c4 in 0..4 {
                    if c1 == c4 || c2 == c4 || c3 == c4 {
                        continue;
                    }

                    i[c4] = 3;

                    let mut prog = [2, 4, 1, 9, 7, 5, 9, 9, 9, 9, 9, 9, 9, 9, 3, 0];

                    prog[6] = map1[c1];
                    prog[7] = map2[c1];
                    prog[8] = map1[c2];
                    prog[9] = map2[c2];
                    prog[10] = map1[c3];
                    prog[11] = map2[c3];
                    prog[12] = map1[c4];
                    prog[13] = map2[c4];

                    let idx1 = 3;
                    let idx2 = 6 + 2 * i[1] + 1;
                    let idx4 = 6 + 2 * i[2] + 1;

                    let comb = 64 * c1 + 16 * c2 + 4 * c3 + c4;
                    let mut off = (8 * 8 * 8) * comb;

                    for xor1 in 0..8 {
                        for xor2 in 0..8 {
                            for four in 0..8 {
                                let mut prog = prog;
                                prog[idx1] = xor1;
                                prog[idx2] = xor2;
                                prog[idx4] = four;

                                let mut new_a = 0;

                                'search: for _ in 0..1000 {
                                    out.clear();
                                    simulate([new_a, 0, 0], &prog, &mut out);

                                    if prog.ends_with(&out) {
                                        if prog.len() == out.len() {
                                            lut[off] = new_a;
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

                                off += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    let lut_u8 = unsafe { std::slice::from_raw_parts(lut.as_ptr().cast::<u8>(), 8 * lut.len()) };
    std::fs::write(path, lut_u8).unwrap();
}
