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

    // let lutd21p1 = Path::new(&std::env::var("OUT_DIR").unwrap()).join("d21p1.lut");
    // make_d21_lut::<u32>(2, &lutd21p1);
    // let lutd21p2 = Path::new(&std::env::var("OUT_DIR").unwrap()).join("d21p2.lut");
    // make_d21_lut::<u64>(25, &lutd21p2);

    let lutd22p1 = Path::new(&std::env::var("OUT_DIR").unwrap()).join("d22p1.lut");
    make_d22_lut(&lutd22p1);
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

    let mut lut = vec![0u64; (6 * 6 * 6) * (8 * 8 * 8 * 8)];

    let mut out = Vec::new();

    let ops = [0, 1, 4, 5];
    let arg_lo = [3, 0, 0, 5];
    let arg_hi = [4, 8, 8, 6];

    macro_rules! unnest {
        ({$($block:tt)*}) => { $($block)* };
        ([$($op:tt)*] $($rest:tt)+) => { $($op)* { unnest!($($rest)+); } };
    }

    unnest! {
        [for x1 in 0..8]
        [for o1 in 0..3]
        [for o2 in 0..3] [if o2 != o1]
        [for o3 in 0..4] [if o3 != o1 && o3 != o2]
        [let o4 = o1 ^ o2 ^ o3;] [if o4 == 0 || o4 == 3]
        [for a1 in arg_lo[o1 as usize]..arg_hi[o1 as usize]]
        [for a2 in arg_lo[o2 as usize]..arg_hi[o2 as usize]]
        [for a3 in arg_lo[o3 as usize]..arg_hi[o3 as usize]]
        [let a4 = arg_lo[o4 as usize];]
        {
            let [o1, o2, o3, o4] = [o1, o2, o3, o4].map(|o| ops[o as usize]);
            let mut prog = [2, 4, 1, x1, 7, 5, o1, a1, o2, a2, o3, a3, o4, a4, 3, 0];

            let mut offset = 0;
            offset = 0 * offset + x1 as usize;
            offset = 6 * offset + o1 as usize;
            offset = 8 * offset + a1 as usize;
            offset = 6 * offset + o2 as usize;
            offset = 8 * offset + a2 as usize;
            offset = 6 * offset + o3 as usize;
            offset = 8 * offset + a3 as usize;

            let mut new_a = 1;

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

#[allow(unused)]
fn make_d21_lut<T: TryFrom<u64>>(n: usize, path: &Path) {
    fn mov_pad(pad: usize, act: usize) -> Option<(usize, Option<usize>)> {
        let dpad = match act {
            1 => -4isize as usize,
            2 => return Some((pad, Some(pad))),
            4 => -1isize as usize,
            5 => 4isize as usize,
            6 => 1isize as usize,
            _ => unreachable!(),
        };

        let res = pad.wrapping_add(dpad);
        if res == 0 || res % 4 == 3 || res > 6 {
            return None;
        }
        Some((res, None))
    }

    fn mov_pad9(pad: usize, act: usize) -> Option<(usize, Option<usize>)> {
        let dpad = match act {
            1 => -4isize as usize,
            2 => return Some((pad, Some(pad))),
            4 => -1isize as usize,
            5 => 4isize as usize,
            6 => 1isize as usize,
            _ => unreachable!(),
        };

        let res = pad.wrapping_add(dpad);
        if res == 12 || res % 4 == 3 || res > 15 {
            return None;
        }
        Some((res, None))
    }

    const P2: [usize; 5] = [1, 2, 4, 5, 6];
    const P9: [usize; 11] = [0, 1, 2, 4, 5, 6, 8, 9, 10, 13, 14];
    const P2_SIZE: usize = 7;
    const P9_SIZE: usize = 15;

    fn pad_map<const SIZE: usize, const SIZE2: usize>(
        pads: &[usize],
        mov: impl Fn(usize, usize) -> Option<(usize, Option<usize>)>,
        distances: &[u64; P2_SIZE * P2_SIZE],
    ) -> [u64; SIZE2] {
        let mut map = [0; SIZE2];
        for &start in pads {
            for &end in pads {
                use std::cmp::Reverse;
                use std::collections::BinaryHeap;
                let mut queue = BinaryHeap::from([(Reverse(0), start, 2)]);
                let mut min = u64::MAX;
                while let Some((Reverse(pressed), p2, p1)) = queue.pop() {
                    if pressed >= min {
                        map[SIZE * start + end] = min;
                        break;
                    }
                    for p2act in P2 {
                        let pressed = pressed + distances[P2_SIZE * p1 + p2act];

                        let Some((p2, final_act)) = mov(p2, p2act) else {
                            continue;
                        };
                        let Some(final_act) = final_act else {
                            queue.push((Reverse(pressed), p2, p2act));
                            continue;
                        };
                        if final_act == end {
                            min = min.min(pressed);
                        }
                    }
                }
            }
        }
        map
    }

    fn pad9_to_byte(pad9: usize) -> u8 {
        match pad9 {
            0 => b'7',
            1 => b'8',
            2 => b'9',

            4 => b'4',
            5 => b'5',
            6 => b'6',

            8 => b'1',
            9 => b'2',
            10 => b'3',

            13 => b'0',
            14 => b'A',

            _ => unreachable!(),
        }
    }

    let mut map = [1; P2_SIZE * P2_SIZE];
    for i in 0..n {
        map = pad_map::<P2_SIZE, { P2_SIZE * P2_SIZE }>(&P2, mov_pad, &map);
    }
    let map = pad_map::<P9_SIZE, { P9_SIZE * P9_SIZE }>(&P9, mov_pad9, &map);

    let mut lut = [0; 1 << 12];

    // let mut s = String::new();

    for &d2 in &P9[..10] {
        for &d1 in &P9[..10] {
            for &d0 in &P9[..10] {
                let bs = [d0, d1, d2].map(pad9_to_byte);

                let key = u32::from_ne_bytes([bs[0], bs[1], bs[2], b'A']);
                let mask = u32::from_ne_bytes([0b1111, 0b1111, 0b1111, 0]);
                let idx = unsafe { std::arch::x86_64::_pext_u32(key, mask) } as usize;

                let mut sum = 0;
                for b in bs {
                    sum = 10 * sum + (b - b'0') as u64;
                }

                let mut cost = 0;
                for (s, e) in [(14, d0), (d0, d1), (d1, d2), (d2, 14)] {
                    cost += map[P9_SIZE * s + e];
                }

                lut[idx] = sum * cost;

                // s += &format!("lut[{idx}] = {}\n", sum * cost);
            }
        }
    }

    // std::fs::write(path.file_name().unwrap(), s).unwrap();

    let mut lut2 = vec![0; 10 * (1 << 16)];
    for b1 in 0..10 {
        for b2 in 0..10 {
            for b3 in 0..10 {
                let key = u32::from_ne_bytes([b1 + b'0', b2 + b'0', b3 + b'0', b'A']);

                let mask = u32::from_ne_bytes([0b1111, 0b1111, 0b1111, 0]);
                let idx_old = unsafe { std::arch::x86_64::_pext_u32(key, mask) } as usize;

                let idx_new =
                    key as usize - (b'A' as usize * (1 << 24) + b'0' as usize * (1 << 16));
                lut2[idx_new] = lut[idx_old];
            }
        }
    }

    let lut = (0..1 << 24)
        .map(|i| lut[i >> 12] + lut[i & ((1 << 12) - 1)])
        .chain(lut2)
        .map(|n| T::try_from(n).unwrap_or_else(|_| unreachable!()))
        .collect::<Vec<_>>();

    let size = lut.len() * std::mem::size_of::<T>();

    let lut_u8 = unsafe { std::slice::from_raw_parts(lut.as_ptr().cast::<u8>(), size) };
    std::fs::write(path, lut_u8).unwrap();
}

#[allow(unused)]
fn make_d22_lut(path: &Path) {
    const M: u32 = 16777216 - 1;

    let mut masks = [0; 24];
    for i in 0..24 {
        let mut n = 1 << i;
        for _ in 0..2000 {
            n ^= n << 6;
            n ^= (n & M) >> 5;
            n ^= n << 11;
        }
        masks[i] = n & M;
    }

    let mut lut = vec![0u32; 1 << 24];
    for i in 0..24 {
        let m = masks[i];
        for n in 0..1 << i {
            lut[(1 << i) | n] = lut[n] ^ m;
        }
    }

    let lut_u8 = unsafe { std::slice::from_raw_parts(lut.as_ptr().cast::<u8>(), 4 * lut.len()) };
    std::fs::write(path, lut_u8).unwrap();
}
