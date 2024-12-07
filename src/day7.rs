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
            if nums.len() == 1 {
                return goal == *nums.get_unchecked(0);
            }

            let n = *nums.get_unchecked(nums.len() - 1);
            let rest = nums.get_unchecked(..nums.len() - 1);

            std::hint::assert_unchecked(n != 0);

            if goal < n {
                return false;
            }

            if goal % n == 0 && solve_rec(goal / n, rest) {
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
            let mut len = 1;
            loop {
                input = input.as_slice().get_unchecked(1..).iter();
                let d = input.as_slice().get_unchecked(0).wrapping_sub(b'0');
                if d >= 10 {
                    *buf.get_unchecked_mut(buf_len) = (n, len);
                    buf_len += 1;
                    break;
                }
                n = 10 * n + d as u32;
                len += 1;
            }
        }
        input = input.as_slice().get_unchecked(1..).iter();

        unsafe fn solve_rec(goal: u64, nums: &[(u32, u32)]) -> bool {
            if nums.len() == 1 {
                return goal == nums.get_unchecked(0).0 as u64;
            }

            let (n, l) = *nums.get_unchecked(nums.len() - 1);
            let (n, l) = (n as u64, l as usize);
            let rest = nums.get_unchecked(..nums.len() - 1);

            static LUT: [u64; 10] = {
                let mut lut = [0; 10];
                let mut acc = 1;
                let mut i = 0;
                while i < 10 {
                    lut[i] = acc;
                    acc *= 10;
                    i += 1;
                }
                lut
            };
            let pow10 = LUT.get_unchecked(l);

            std::hint::assert_unchecked(n != 0);

            if goal < n {
                return false;
            }

            if (goal - n) % pow10 == 0 && solve_rec((goal - n) / pow10, rest) {
                return true;
            }

            if goal % n == 0 && solve_rec(goal / n, rest) {
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
