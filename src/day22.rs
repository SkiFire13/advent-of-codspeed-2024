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

macro_rules! parse {
    ($ptr:ident) => {{
        let n = $ptr.cast::<u64>().read_unaligned();
        let len = _pext_u64(n, 0x1010101010101010).trailing_ones();
        let n = (n & 0x0F0F0F0F0F0F0F0F) << (8 * (8 - len));
        $ptr = $ptr.add(len as usize + 1);
        parse8(n)
    }};
}

const M: u32 = 16777216 - 1;

#[inline(always)]
fn next(mut n: u32) -> u32 {
    n ^= n << 6;
    n ^= (n & M) >> 5;
    n ^= n << 11;
    n & M
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let mut sum = u64x8::splat(0);
    let mut ptr = input.as_ptr();
    for _ in 0..200 {
        let n0 = parse!(ptr);
        let n1 = parse!(ptr);
        let n2 = parse!(ptr);
        let n3 = parse!(ptr);
        let n4 = parse!(ptr);
        let n5 = parse!(ptr);
        let n6 = parse!(ptr);
        let n7 = parse!(ptr);

        // dbg!(n0, n1, n2, n3, n4, n5, n6, n7);

        let mut n = u32x8::from_array([n0, n1, n2, n3, n4, n5, n6, n7]);
        let m = u32x8::splat(M);
        for _ in 0..2000 {
            n ^= n << 6;
            n ^= (n & m) >> 5;
            n ^= n << 11;
        }
        sum += (n & m).cast::<u64>();
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

        for _ in 0..2000 {
            n = next(n);
        }

        n & M
    };

    sum.reduce_sum() + last as u64
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut ptr = input.as_ptr();

    const COUNTS_LEN: usize = (19usize * 19 * 19 * 19).next_multiple_of(64);
    let mut counts = [0u16; COUNTS_LEN];

    macro_rules! handle {
        ($n:expr) => {{
            let mut n = $n;
            let mut seen = [0u64; (19 * 19 * 19 * 19 + 63) / 64];

            let b1 = fastdiv::fastmod_u32_10(n);
            n = next(n);
            let b2 = fastdiv::fastmod_u32_10(n);
            n = next(n);
            let b3 = fastdiv::fastmod_u32_10(n);
            n = next(n);
            let mut b4 = fastdiv::fastmod_u32_10(n);

            let mut d1 = 9 + b1 - b2;
            let mut d2 = 9 + b2 - b3;
            let mut d3 = 9 + b3 - b4;

            for _ in 3..2000 {
                n = next(n);
                let b5 = fastdiv::fastmod_u32_10(n);

                let d4 = 9 + b4 - b5;

                let idx = (d1 + 19 * (d2 + 19 * (d3 + 19 * d4))) as usize;
                let s = seen.get_unchecked_mut(idx / 64);
                if *s & (1 << (idx % 64)) == 0 {
                    *s |= 1 << (idx % 64);
                    *counts.get_unchecked_mut(idx) += b5 as u16;
                }

                (d1, d2, d3, b4) = (d2, d3, d4, b5);
            }
        }};
    }

    for _ in 0..1600 {
        let n = parse!(ptr);
        handle!(n);
    }

    {
        let len = input.as_ptr().add(input.len()).offset_from(ptr) - 1;
        let n = input
            .as_ptr()
            .add(input.len() - 1 - 8)
            .cast::<u64>()
            .read_unaligned();
        let n = (n & 0x0F0F0F0F0F0F0F0F) & (u64::MAX << (8 * (8 - len)));
        let n = parse8(n);
        handle!(n);
    }

    let mut max = u16x16::splat(0);
    for i in 0..COUNTS_LEN / 16 {
        let b = u16x16::from_slice(counts.get_unchecked(16 * i..16 * i + 16));
        max = max.simd_max(b);
    }
    max.reduce_max() as u64
}

mod fastdiv {
    #[inline]
    const fn mul128_u32(lowbits: u64, d: u32) -> u64 {
        (lowbits as u128 * d as u128 >> 64) as u64
    }

    #[inline]
    const fn compute_m_u32(d: u32) -> u64 {
        (0xFFFFFFFFFFFFFFFF / d as u64) + 1
    }

    #[inline]
    pub const fn fastmod_u32_10(a: u32) -> u32 {
        const D: u32 = 10;
        const M: u64 = compute_m_u32(D);

        let lowbits = M.wrapping_mul(a as u64);
        mul128_u32(lowbits, D) as u32
    }
}
