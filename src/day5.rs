#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]

pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

pub fn part1(input: &str) -> u32 {
    unsafe { inner_part1(input) }
}

pub fn part2(input: &str) -> u32 {
    unsafe { inner_part2(input) }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u32 {
    let mut map = [0u128; 100];

    let mut input = input.as_bytes();

    for (i, c) in input.chunks_exact(6).enumerate() {
        if c[0] == b'\n' {
            input = &input[6 * i + 1..];
            break;
        }

        let d1 = ((c[0] - b'0') as usize * 10) + ((c[1] - b'0') as usize);
        let d2 = ((c[3] - b'0') as usize * 10) + ((c[4] - b'0') as usize);
        *map.get_unchecked_mut(d1) |= 1 << d2;
    }

    let mut tot = 0;
    let mut buf = [0; 24];
    let mut buf_len = 0;
    let mut mask = 0u128;

    let mut iter = input.iter();

    'outer: while !iter.as_slice().is_empty() {
        let c = iter.as_slice();

        let n = (*c.get_unchecked(0) - b'0') * 10 + (*c.get_unchecked(1) - b'0');
        *buf.get_unchecked_mut(buf_len) = n;
        buf_len += 1;

        if *map.get_unchecked(n as usize) & mask != 0 {
            while *iter.as_slice().get_unchecked(2) != b'\n' {
                iter = iter.as_slice().get_unchecked(3..).iter();
            }
            iter = iter.as_slice().get_unchecked(3..).iter();
            buf_len = 0;
            mask = 0;
            continue 'outer;
        }

        mask |= 1 << n;

        if *c.get_unchecked(2) == b'\n' {
            tot += *buf.get_unchecked(buf_len / 2) as u32;
            buf_len = 0;
            mask = 0;
        }

        iter = iter.as_slice().get_unchecked(3..).iter();
    }

    tot
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u32 {
    let mut map = [0u128; 100];

    let mut input = input.as_bytes();

    for (i, c) in input.chunks_exact(6).enumerate() {
        if c[0] == b'\n' {
            input = &input[6 * i + 1..];
            break;
        }

        let d1 = ((c[0] - b'0') as usize * 10) + ((c[1] - b'0') as usize);
        let d2 = ((c[3] - b'0') as usize * 10) + ((c[4] - b'0') as usize);
        *map.get_unchecked_mut(d1) |= 1 << d2;
    }

    let mut tot = 0;
    let mut buf = [0; 24];
    let mut buf_len = 0;
    let mut mask = 0u128;
    let mut valid = true;

    for c in input.chunks_exact(3) {
        let n = (c[0] - b'0') * 10 + (c[1] - b'0');
        *buf.get_unchecked_mut(buf_len) = n;
        buf_len += 1;
        mask |= 1 << n;

        valid &= *map.get_unchecked(n as usize) & mask == 0;
        mask |= 1 << n;

        if c[2] == b'\n' {
            if !valid {
                let mut masks = [0; 24];
                for i in 0..buf_len {
                    *masks.get_unchecked_mut(i) =
                        *map.get_unchecked(*buf.get_unchecked(i) as usize) & mask;
                }

                let mut new_buf = [0; 24];
                let mut mask = u128::MAX;

                for i in 0..buf_len / 2 + 1 {
                    let mut j = 0;
                    loop {
                        if *masks.get_unchecked(j) & mask == 0 {
                            let n = *buf.get_unchecked(j);
                            *new_buf.get_unchecked_mut(i) = n;
                            mask ^= 1 << n;
                            *masks.get_unchecked_mut(j) = *masks.get_unchecked(buf_len - i - 1);
                            *buf.get_unchecked_mut(j) = *buf.get_unchecked_mut(buf_len - i - 1);

                            break;
                        }
                        j += 1;
                    }
                }

                tot += *new_buf.get_unchecked(buf_len / 2) as u32;
            }
            buf_len = 0;
            mask = 0;
            valid = true;
        }
    }

    tot
}
