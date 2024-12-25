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

    static mut PREVS: [u64; 500] = [0; 500];
    let prevs = &mut PREVS;
    let mut prevs_len = 0;

    let mut count = 0;

    let ptr = input.as_ptr();
    let mut i = 0;
    let len = input.len();

    let mut b = ptr.cast::<u8x64>().read_unaligned();

    loop {
        let m = b.simd_eq(u8x64::splat(b'#')).to_bitmask() & ((1 << 42) - 1);

        // TODO: explicit SIMD
        for j in 0..prevs_len {
            if *prevs.get_unchecked(j) & m == 0 {
                count += 1;
            }
        }

        *prevs.get_unchecked_mut(prevs_len) = m;
        prevs_len += 1;

        i += 43;
        if i + 43 <= len {
            b = ptr.add(i).cast::<u8x64>().read_unaligned();
        } else if i < len {
            b = ptr
                .add(len - 64)
                .cast::<u8x64>()
                .read_unaligned()
                .rotate_elements_left::<{ 64 - 42 }>();
        } else {
            break;
        }
    }

    count
}

#[allow(unused)]
#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    0
}
