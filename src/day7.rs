#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]

use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let mut tot = 0;

    let mut input = input.as_bytes().iter();
    while !input.as_slice().is_empty() {
        let len = u8x16::from_slice(input.as_slice().get_unchecked(..16))
            .simd_eq(u8x16::splat(b':'))
            .first_set()
            .unwrap_unchecked();

        let goal =
            atoi_radix10::parse::<u64>(input.as_slice().get_unchecked(..len)).unwrap_unchecked();

        let mut buf = [0; 24];
        let mut buf_len = 0;
        input = input.as_slice().get_unchecked(len + 1..).iter();
        while *input.as_slice().get_unchecked(0) == b' ' {
            input = input.as_slice().get_unchecked(1..).iter();
            let mut n = input.as_slice().get_unchecked(0).wrapping_sub(b'0') as u64;
            loop {
                input = input.as_slice().get_unchecked(1..).iter();
                let d = input.as_slice().get_unchecked(0).wrapping_sub(b'0');
                if d >= 10 {
                    *buf.get_unchecked_mut(buf_len) = n;
                    buf_len += 1;
                    break;
                }
                n = 10 * n + d as u64;
            }
        }
        input = input.as_slice().get_unchecked(1..).iter();

        unsafe fn solve_rec(goal: u64, nums: &[u64]) -> bool {
            let n = *nums.get_unchecked(nums.len() - 1) as u64;

            if nums.len() == 1 {
                return goal == n;
            }

            let rest = nums.get_unchecked(..nums.len() - 1);

            std::hint::assert_unchecked(n != 0);
            let (div, rem) = (goal / n, goal % n);
            if rem == 0 && solve_rec(div, rest) {
                return true;
            }

            goal > n && solve_rec(goal - n, rest)
        }

        if solve_rec(goal, &buf.get_unchecked(..buf_len)) {
            tot += goal;
        }
    }

    tot
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let mut tot = 0;

    let mut input = input.as_bytes().iter();
    while !input.as_slice().is_empty() {
        let len = u8x16::from_slice(input.as_slice().get_unchecked(..16))
            .simd_eq(u8x16::splat(b':'))
            .first_set()
            .unwrap_unchecked();

        let goal =
            atoi_radix10::parse::<u64>(input.as_slice().get_unchecked(..len)).unwrap_unchecked();

        let mut buf = const { [(0, 0); 24] };
        let mut buf_len = 0;
        input = input.as_slice().get_unchecked(len + 1..).iter();
        while *input.as_slice().get_unchecked(0) == b' ' {
            input = input.as_slice().get_unchecked(1..).iter();
            let mut n = input.as_slice().get_unchecked(0).wrapping_sub(b'0') as u32;
            let mut pow10idx = 0;
            loop {
                input = input.as_slice().get_unchecked(1..).iter();
                let d = input.as_slice().get_unchecked(0).wrapping_sub(b'0');
                if d >= 10 {
                    *buf.get_unchecked_mut(buf_len) = (n, pow10idx);
                    buf_len += 1;
                    break;
                }
                n = 10 * n + d as u32;
                pow10idx += 1;
            }
        }
        input = input.as_slice().get_unchecked(1..).iter();

        unsafe fn solve_rec(goal: u64, nums: &[(u32, u32)]) -> bool {
            let (n, l) = *nums.get_unchecked(nums.len() - 1);
            let (n, l) = (n as u64, l as usize);

            if nums.len() == 1 {
                return goal == n;
            }

            if goal < n {
                return false;
            }

            let rest = nums.get_unchecked(..nums.len() - 1);

            use fastdiv::PrecomputedDivU64;
            static LUT: [PrecomputedDivU64; 3] = [
                PrecomputedDivU64::new(10),
                PrecomputedDivU64::new(100),
                PrecomputedDivU64::new(1000),
            ];
            let pow10 = *LUT.get_unchecked(l);
            let sub = goal - n;
            if PrecomputedDivU64::is_multiple_of(sub, pow10)
                && solve_rec(PrecomputedDivU64::fast_div(sub, pow10), rest)
            {
                return true;
            }

            std::hint::assert_unchecked(n != 0);
            let (div, rem) = (goal / n, goal % n);
            if rem == 0 && solve_rec(div, rest) {
                return true;
            }

            solve_rec(goal - n, rest)
        }

        if solve_rec(goal, &buf.get_unchecked(..buf_len)) {
            tot += goal;
        }
    }

    tot
}

mod fastdiv {
    #[inline]
    const fn mul128_u64(lowbits: u128, d: u64) -> u64 {
        let mut bottom_half = (lowbits & 0xFFFFFFFFFFFFFFFF) * d as u128;
        bottom_half >>= 64;
        let top_half = (lowbits >> 64) * d as u128;
        let both_halves = bottom_half + top_half;
        (both_halves >> 64) as u64
    }

    #[inline]
    const fn compute_m_u64(d: u64) -> u128 {
        (0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF / d as u128) + 1
    }
    // for d > 1
    #[inline]
    const fn fastdiv_u64(a: u64, m: u128) -> u64 {
        mul128_u64(m, a)
    }
    #[inline]
    const fn is_divisible_u64(n: u64, m: u128) -> bool {
        (n as u128).wrapping_mul(m) <= m - 1
    }

    #[derive(Clone, Copy, Eq, PartialEq)]
    pub struct PrecomputedDivU64(u128);

    impl PrecomputedDivU64 {
        #[inline]
        pub const fn new(n: u64) -> Self {
            Self(compute_m_u64(n))
        }

        #[inline]
        pub fn fast_div(n: u64, precomputed: Self) -> u64 {
            fastdiv_u64(n, precomputed.0)
        }

        #[inline]
        pub fn is_multiple_of(n: u64, precomputed: Self) -> bool {
            is_divisible_u64(n, precomputed.0)
        }
    }
}
