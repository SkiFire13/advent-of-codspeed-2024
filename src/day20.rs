#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

#[inline(always)]
pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    let mut offset = 0;
    let p1 = loop {
        let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
        let mask = block.simd_ge(u8x64::splat(b'E')).to_bitmask();
        if mask != 0 {
            break offset + mask.trailing_zeros() as usize;
        }
        offset += 64;
    };

    let b = (b'E' + b'S') - *input.get_unchecked(p1);

    offset = p1 + 1;
    let p2 = loop {
        let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
        let mask = block.simd_eq(u8x64::splat(b)).to_bitmask();
        if mask != 0 {
            break offset + mask.trailing_zeros() as usize;
        }
        offset += 64;
    };

    let (s, e) = if b == b'S' { (p1, p2) } else { (p2, p1) };

    const LEFT: usize = -1isize as usize;
    const RIGHT: usize = 1isize as usize;
    const UP: usize = -142isize as usize;
    const DOWN: usize = 142isize as usize;

    let mut count = 0;
    let mut prev = 0;
    let mut curr = s;
    let mut n = u16::MAX - 100;

    let mut seen = [0; 142 * 143];

    loop {
        *seen.get_unchecked_mut(curr + 142) = n;

        let mut new_curr = curr.wrapping_add(LEFT);
        for d in [RIGHT, UP, DOWN] {
            let cand = curr.wrapping_add(d);
            if *input.get_unchecked(cand) != b'#' && cand != prev {
                new_curr = curr.wrapping_add(d);
            }
        }

        prev = curr;
        curr = new_curr;
        n -= 1;

        for d in [LEFT, RIGHT, UP, DOWN] {
            if *seen.get_unchecked((curr + 142).wrapping_add(d).wrapping_add(d)) > n + 100 {
                count += 1;
            }
        }

        if curr == e {
            break;
        }
    }

    count
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    0
}
