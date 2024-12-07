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

            let pow10 = *[10, 100, 1000].get_unchecked(l);
            std::hint::assert_unchecked(pow10 != 0);
            let sub = goal - n;
            let (div, rem) = (sub / pow10, sub % pow10);
            if rem == 0 && solve_rec(div, rest) {
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
