#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

use std::arch::x86_64::*;
use std::mem::MaybeUninit;
use std::simd::prelude::*;

use rayon::prelude::*;

#[inline(always)]
pub(crate) fn parse8(n: u64) -> u32 {
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
pub(crate) use parse;

pub(crate) const M: u32 = 16777216 - 1;

#[inline(always)]
pub(crate) fn next(mut n: u32) -> u32 {
    n ^= n << 6;
    n ^= (n & M) >> 5;
    n ^= n << 11;
    n
}

pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let input = input.as_bytes();

    const COUNTS_LEN: usize = (20usize * 20 * 20 * 20).next_multiple_of(64);
    static mut COUNTS: [u16; 128 * COUNTS_LEN] = [0; 128 * COUNTS_LEN];

    let cores = std::thread::available_parallelism()
        .unwrap()
        .get()
        .clamp(8, 128);
    let init_len = cores * COUNTS_LEN;
    COUNTS
        .as_mut_ptr()
        .cast::<u8>()
        .write_bytes(0, 2 * init_len);

    let counts = std::slice::from_raw_parts_mut(COUNTS.as_mut_ptr().cast::<u16>(), init_len);

    macro_rules! handle {
        ($n:expr, $i:expr, $seen:ident, $count:ident) => {{
            let mut n = $n;

            let b1 = fastdiv::fastmod_u32_10(n);
            n = next(n) & M;
            let b2 = fastdiv::fastmod_u32_10(n);
            n = next(n) & M;
            let b3 = fastdiv::fastmod_u32_10(n);
            n = next(n) & M;
            let mut b4 = fastdiv::fastmod_u32_10(n);

            let mut d1 = 9 + b1 - b2;
            let mut d2 = 9 + b2 - b3;
            let mut d3 = 9 + b3 - b4;

            for _ in 3..2000 {
                n = next(n) & M;
                let b5 = fastdiv::fastmod_u32_10(n);

                let d4 = 9 + b4 - b5;

                let idx = (d1 + 20 * (d2 + 20 * (d3 + 20 * d4))) as usize;
                let s = $seen.get_unchecked_mut(idx);
                if *s != $i {
                    *s = $i;
                    *$count.get_unchecked_mut(idx) += b5 as u16;
                }

                (d1, d2, d3, b4) = (d2, d3, d4, b5);
            }
        }};
    }

    let mut nums = [MaybeUninit::<u32>::uninit(); 3000];
    let mut nums_len = 0;

    let mut ptr = input.as_ptr();
    while ptr <= input.as_ptr().add(input.len() - 8) {
        *nums.get_unchecked_mut(nums_len).as_mut_ptr() = parse!(ptr);
        nums_len += 1;
    }

    if ptr != input.as_ptr().add(input.len()) {
        let len = input.as_ptr().add(input.len()).offset_from(ptr) - 1;
        let n = input
            .as_ptr()
            .add(input.len() - 1 - 8)
            .cast::<u64>()
            .read_unaligned();
        let n = (n & 0x0F0F0F0F0F0F0F0F) & (u64::MAX << (8 * (8 - len)));
        let n = parse8(n);
        *nums.get_unchecked_mut(nums_len).as_mut_ptr() = n;
        nums_len += 1;
    }

    let nums = std::slice::from_raw_parts(nums.as_ptr().cast::<u32>(), nums_len);

    nums.par_chunks((nums.len() + cores - 1) / cores)
        .zip(counts.par_chunks_mut(COUNTS_LEN))
        .with_max_len(1)
        .for_each(|(chunk, counts)| {
            let mut seen = [0u8; COUNTS_LEN];
            for (i, &n) in chunk.iter().enumerate() {
                handle!(n, i as u8 + 1, seen, counts);
            }
        });

    let mut max = u16x16::splat(0);
    for i in 0..COUNTS_LEN / 16 {
        let mut sum = u16x16::splat(0);
        for j in 0..cores {
            let b = u16x16::from_slice(
                counts
                    .get_unchecked(COUNTS_LEN * j + 16 * i..)
                    .get_unchecked(..16),
            );
            sum += b;
        }
        max = max.simd_max(sum);
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
