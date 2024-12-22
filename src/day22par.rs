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

use super::day22::{next, parse, parse8, M};

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
    let mut ptr = input.as_ptr();

    const COUNTS_LEN: usize = (19usize * 19 * 19 * 19).next_multiple_of(64);
    static mut COUNTS: [u16; 128 * COUNTS_LEN] = [0; 128 * COUNTS_LEN];

    let cores = std::thread::available_parallelism().unwrap().get();
    let init_len = cores * COUNTS_LEN;
    COUNTS
        .as_mut_ptr()
        .cast::<u8>()
        .write_bytes(0, 2 * init_len);

    let counts = std::slice::from_raw_parts_mut(COUNTS.as_mut_ptr().cast::<u16>(), init_len);

    macro_rules! handle {
        ($n:expr, $count:ident) => {{
            let mut n = $n;
            let mut seen = [0u64; (19 * 19 * 19 * 19 + 63) / 64];

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

                let idx = (d1 + 19 * (d2 + 19 * (d3 + 19 * d4))) as usize;
                let s = seen.get_unchecked_mut(idx / 64);
                if *s & (1 << (idx % 64)) == 0 {
                    *s |= 1 << (idx % 64);
                    *$count.get_unchecked_mut(idx) += b5 as u16;
                }

                (d1, d2, d3, b4) = (d2, d3, d4, b5);
            }
        }};
    }

    let mut nums = [MaybeUninit::uninit(); 1601];

    for i in 0..1600 {
        nums[i].write(parse!(ptr));
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
        nums[1600].write(n);
    }

    nums.par_chunks((nums.len() + cores - 1) / cores)
        .zip(counts.par_chunks_mut(COUNTS_LEN))
        .with_max_len(1)
        .for_each(|(chunk, counts)| {
            for &n in chunk {
                handle!(n.assume_init(), counts);
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
