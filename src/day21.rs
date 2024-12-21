#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

use std::arch::x86_64::*;

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

// static LUT1: [u64; 1 << 12] = {
//     let mut lut = [0; 1 << 12];

//     lut
// };
// static LUT2: [u64; 1 << 12] = {
//     let mut lut = [0; 1 << 12];

//     lut
// };

static LUT1: [u64; 1 << 12] =
    unsafe { std::mem::transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/d21p1.lut"))) };

static LUT2: [u64; 1 << 12] =
    unsafe { std::mem::transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/d21p2.lut"))) };

#[inline(always)]
unsafe fn solve(input: &[u8], lut: &[u64; 1 << 12]) -> u64 {
    let mut tot = 0;
    for off in [0, 5, 10, 15, 20] {
        let key = input
            .get_unchecked(off..off + 4)
            .as_ptr()
            .cast::<u32>()
            .read_unaligned();
        let mask = u32::from_ne_bytes([0b1111, 0b1111, 0b1111, 0]);
        let idx = _pext_u32(key, mask);
        tot += *lut.get_unchecked(idx as usize);
    }
    tot
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    solve(input.as_bytes(), &LUT1)
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    solve(input.as_bytes(), &LUT2)
}
