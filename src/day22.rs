#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

use std::arch::x86_64::*;
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

#[inline(always)]
fn parse8(n: u64) -> u32 {
    use std::num::Wrapping as W;

    let mut n = W(n);
    let mask = W(0xFF | (0xFF << 32));
    let mul1 = W(100 + (1000000 << 32));
    let mul2 = W(1 + (10000 << 32));

    n = (n * W(10)) + (n >> 8);
    n = (((n & mask) * mul1) + (((n >> 16) & mask) * mul2)) >> 32;

    n.0 as u32
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let mut sum = u64x8::splat(0);
    let mut ptr = input.as_ptr();
    for _ in 0..200 {
        macro_rules! parse {
            () => {{
                let n = ptr.cast::<u64>().read_unaligned();
                let len = _pext_u64(n, 0x1010101010101010).trailing_ones();
                let n = (n & 0x0F0F0F0F0F0F0F0F) << (8 * (8 - len));
                ptr = ptr.add(len as usize + 1);
                parse8(n)
            }};
        }

        let n0 = parse!();
        let n1 = parse!();
        let n2 = parse!();
        let n3 = parse!();
        let n4 = parse!();
        let n5 = parse!();
        let n6 = parse!();
        let n7 = parse!();

        // dbg!(n0, n1, n2, n3, n4, n5, n6, n7);

        let mut n = u32x8::from_array([n0, n1, n2, n3, n4, n5, n6, n7]);
        for _ in 0..2000 {
            let m = u32x8::splat(16777216 - 1);
            n = (n ^ (n << 6)) & m;
            n = (n ^ (n >> 5)) & m;
            n = (n ^ (n << 11)) & m;
        }
        sum += n.cast::<u64>();
    }

    let last = {
        let len = input.as_ptr().add(input.len()).offset_from(ptr) - 1;
        let n = input
            .as_ptr()
            .add(input.len() - 1 - 8)
            .cast::<u64>()
            .read_unaligned();
        let n = (n & 0x0F0F0F0F0F0F0F0F) & (u64::MAX << (8 * (8 - len)));
        let mut n = parse8(n);

        // dbg!(len, n);

        for _ in 0..2000 {
            let m = 16777216 - 1;
            n = (n ^ (n << 6)) & m;
            n = (n ^ (n >> 5)) & m;
            n = (n ^ (n << 11)) & m;
        }

        n
    };

    sum.reduce_sum() + last as u64
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    0
}
