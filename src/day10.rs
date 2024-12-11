#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]

use std::mem::MaybeUninit;
use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part2(input) as i64
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
    let mut tot = 0;

    loop {
        let mut mask = if offset + 64 < input.len() {
            let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
            block.simd_eq(u8x64::splat(b'0')).to_bitmask()
        } else if offset < input.len() {
            let block = u8x32::from_slice(input.get_unchecked(45 * 46 - 32..));
            block.simd_eq(u8x32::splat(b'0')).to_bitmask() >> 10
        } else {
            break;
        };

        while mask != 0 {
            let o = mask.trailing_zeros();
            mask &= !(1 << o);

            let mut seen = u16x8::from_slice(&[u16::MAX; 8]);
            let mut seen_len = 0;

            let mut stack = [MaybeUninit::uninit(); 16];
            let mut stack_len = 0;

            let mut curr_o = offset + o as usize;
            let mut c = b'0';

            loop {
                if c == b'9' {
                    if seen.simd_ne(u16x8::splat(curr_o as u16)).all() {
                        *seen.as_mut_array().get_unchecked_mut(seen_len) = curr_o as u16;
                        seen_len += 1;
                        tot += 1;
                    }

                    if stack_len == 0 {
                        break;
                    }
                    stack_len -= 1;
                    (curr_o, c) = stack.get_unchecked(stack_len).assume_init();

                    continue;
                }

                let new_o = curr_o.wrapping_sub(1);
                if new_o < 45 * 46 - 1 && *input.get_unchecked(new_o) == c + 1 {
                    *stack.get_unchecked_mut(stack_len).as_mut_ptr() = (new_o, c + 1);
                    stack_len += 1;
                }
                let new_o = curr_o.wrapping_add(1);
                if new_o < 45 * 46 - 1 && *input.get_unchecked(new_o) == c + 1 {
                    *stack.get_unchecked_mut(stack_len).as_mut_ptr() = (new_o, c + 1);
                    stack_len += 1;
                }
                let new_o = curr_o.wrapping_sub(46);
                if new_o < 45 * 46 - 1 && *input.get_unchecked(new_o) == c + 1 {
                    *stack.get_unchecked_mut(stack_len).as_mut_ptr() = (new_o, c + 1);
                    stack_len += 1;
                }
                let new_o = curr_o.wrapping_add(46);
                if new_o < 45 * 46 - 1 && *input.get_unchecked(new_o) == c + 1 {
                    (curr_o, c) = (new_o, c + 1);
                } else {
                    if stack_len > 0 {
                        stack_len -= 1;
                        (curr_o, c) = *stack.get_unchecked(stack_len).as_ptr();
                    } else {
                        break;
                    }
                }
            }
        }

        offset += 64;
    }

    tot
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let input = input.as_bytes();

    let mut offset = 0;
    let mut tot = 0;

    loop {
        let mut mask = if offset + 64 < input.len() {
            let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
            block.simd_eq(u8x64::splat(b'9')).to_bitmask()
        } else if offset < input.len() {
            let block = u8x32::from_slice(input.get_unchecked(45 * 46 - 32..));
            block.simd_eq(u8x32::splat(b'9')).to_bitmask() >> 10
        } else {
            break;
        };

        while mask != 0 {
            let o = mask.trailing_zeros();
            mask &= !(1 << o);

            let mut seen1 = u16x16::from_slice(&[u16::MAX; 16]);
            let mut counts1 = [MaybeUninit::uninit(); 16];
            let mut len1 = 1;
            seen1[0] = offset as u16 + o as u16;
            counts1[0] = MaybeUninit::new(1);

            let mut seen2 = u16x16::from_slice(&[u16::MAX; 16]);
            let mut counts2 = [MaybeUninit::uninit(); 16];
            let mut len2 = 0;

            for c in (b'1'..b'8' + 1).rev() {
                for i in 0..len1 {
                    let curr_o = *seen1.as_array().get_unchecked(i);
                    let curr_count = *counts1.get_unchecked(i).as_ptr();

                    macro_rules! handle {
                        ($new_o:expr) => {{
                            let new_o = $new_o;
                            if new_o < 45 * 46 - 1 && *input.get_unchecked(new_o as usize) == c {
                                if let Some(idx) =
                                    seen2.simd_eq(u16x16::splat(new_o as u16)).first_set()
                                {
                                    *counts2.get_unchecked_mut(idx).as_mut_ptr() += curr_count;
                                } else {
                                    *counts2.get_unchecked_mut(len2).as_mut_ptr() = curr_count;
                                    *seen2.as_mut_array().get_unchecked_mut(len2) = new_o as u16;
                                    len2 += 1;
                                }
                            }
                        }};
                    }

                    handle!(curr_o.wrapping_sub(1));
                    handle!(curr_o.wrapping_add(1));
                    handle!(curr_o.wrapping_sub(46));
                    handle!(curr_o.wrapping_add(46));
                }

                seen1 = seen2;
                counts1 = counts2;
                len1 = len2;

                seen2 = u16x16::from_slice(&[u16::MAX; 16]);
                counts2 = [MaybeUninit::uninit(); 16];
                len2 = 0;
            }

            let c = b'0';
            for i in 0..len1 {
                let curr_o = *seen1.as_array().get_unchecked(i);
                let curr_count = *counts1.get_unchecked(i).as_ptr();

                macro_rules! handle {
                    ($new_o:expr) => {{
                        let new_o = $new_o;
                        if new_o < 45 * 46 - 1 && *input.get_unchecked(new_o as usize) == c {
                            tot += curr_count;
                        }
                    }};
                }

                handle!(curr_o.wrapping_sub(1));
                handle!(curr_o.wrapping_add(1));
                handle!(curr_o.wrapping_sub(46));
                handle!(curr_o.wrapping_add(46));
            }
        }

        offset += 64;
    }

    tot as u64
}
