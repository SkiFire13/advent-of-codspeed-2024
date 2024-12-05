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

    let mut iter = input.as_bytes().iter();

    while *iter.as_slice().get_unchecked(0) != b'\n' {
        let c = iter.as_slice().get_unchecked(..5);
        let d1 = ((c[0] - b'0') as usize * 10) + ((c[1] - b'0') as usize);
        let d2 = ((c[3] - b'0') as usize * 10) + ((c[4] - b'0') as usize);
        *map.get_unchecked_mut(d1) |= 1 << d2;
        iter = iter.as_slice().get_unchecked(6..).iter();
    }

    iter = iter.as_slice().get_unchecked(1..).iter();

    let mut tot = 0;
    let mut buf = [0; 24];
    let mut buf_len = 0;
    let mut mask = 0u128;

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

    let mut iter = input.as_bytes().iter();

    while *iter.as_slice().get_unchecked(0) != b'\n' {
        let c = iter.as_slice().get_unchecked(..5);
        let d1 = ((c[0] - b'0') as usize * 10) + ((c[1] - b'0') as usize);
        let d2 = ((c[3] - b'0') as usize * 10) + ((c[4] - b'0') as usize);
        *map.get_unchecked_mut(d1) |= 1 << d2;
        iter = iter.as_slice().get_unchecked(6..).iter();
    }

    iter = iter.as_slice().get_unchecked(1..).iter();

    let mut tot = 0;
    let mut buf = [0; 24];
    let mut buf_len = 0;
    let mut mask = 0u128;
    let mut valid = true;

    for c in iter.as_slice().chunks_exact(3) {
        let n = (c[0] - b'0') * 10 + (c[1] - b'0');
        *buf.get_unchecked_mut(buf_len) = n;
        buf_len += 1;
        valid &= *map.get_unchecked(n as usize) & mask == 0;
        mask |= 1 << n;

        if c[2] == b'\n' {
            if !valid {
                for i in 0..buf_len {
                    let succs = *map.get_unchecked(*buf.get_unchecked(i) as usize) & mask;
                    if succs.count_ones() == buf_len as u32 / 2 {
                        tot += *buf.get_unchecked(i) as u32;
                        break;
                    }
                }
            }
            buf_len = 0;
            mask = 0;
            valid = true;
        }
    }

    tot
}
