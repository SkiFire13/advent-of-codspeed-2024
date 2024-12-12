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

const EXTRA_MASKS: [[i8; 32]; 32] = {
    let mut masks = [[0; 32]; 32];
    let mut i = 0;
    while i < 32 {
        let mut j = i;
        while j < 32 {
            masks[i][j] = 1;
            j += 1;
        }
        i += 1;
    }
    masks
};

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    let mut edges = const {
        let mut edges = [0; 140 * 141];

        let mut i = 0;
        while i < 141 {
            edges[0 * 141 + i] = 1;
            edges[139 * 141 + i] = 1;
            i += 1;
        }

        edges[0] += 1;
        edges[140 * 141 - 1] += 1;

        edges
    };

    // TODO: bitmap to which neighbours are equal -> switch table

    {
        let mut off = 0;
        while off + 141 + 32 <= 140 * 141 {
            let o1 = off;
            let o2 = off + 1;
            let o3 = off + 141;

            let b1 = u8x32::from_slice(input.get_unchecked(o1..o1 + 32));
            let b2 = u8x32::from_slice(input.get_unchecked(o2..o2 + 32));
            let b3 = u8x32::from_slice(input.get_unchecked(o3..o3 + 32));

            let t = b1.simd_ne(b2);
            let l = b1.simd_ne(b3);

            let mut s1 = i8x32::from_slice(edges.get_unchecked(o1..o1 + 32));
            s1 += t.to_int() & i8x32::splat(1);
            s1 += l.to_int() & i8x32::splat(1);
            *edges.get_unchecked_mut(o1) = s1[0];

            let mut s2 = s1.rotate_elements_left::<1>();
            s2[31] = *edges.get_unchecked(o2 + 32 - 1);
            s2 += t.to_int() & i8x32::splat(1);
            s2.copy_to_slice(edges.get_unchecked_mut(o2..o2 + 32));

            let mut s3 = i8x32::from_slice(edges.get_unchecked(o3..o3 + 32));
            s3 += l.to_int() & i8x32::splat(1);
            s3.copy_to_slice(edges.get_unchecked_mut(o3..o3 + 32));

            off += 32;
        }

        let extra = (off + 141 + 1 + 32) - 140 * 141;
        if extra != 32 {
            let mask = i8x32::from_slice(EXTRA_MASKS.get_unchecked(extra));
            off = 140 * 141 - (141 + 1 + 32);

            let o1 = off;
            let o2 = off + 1;
            let o3 = off + 141;

            let b1 = u8x32::from_slice(input.get_unchecked(o1..o1 + 32));
            let b2 = u8x32::from_slice(input.get_unchecked(o2..o2 + 32));
            let b3 = u8x32::from_slice(input.get_unchecked(o3..o3 + 32));

            let t = b1.simd_ne(b2);
            let l = b1.simd_ne(b3);

            let mut s1 = i8x32::from_slice(edges.get_unchecked(o1..o1 + 32));
            s1 += t.to_int() & mask;
            s1 += l.to_int() & mask;
            *edges.get_unchecked_mut(o1) = s1[0];

            let mut s2 = s1.rotate_elements_left::<1>();
            s2[31] = *edges.get_unchecked(o2 + 32 - 1);
            s2 += t.to_int() & mask;
            s2.copy_to_slice(edges.get_unchecked_mut(o2..o2 + 32));

            let mut s3 = i8x32::from_slice(edges.get_unchecked(o3..o3 + 32));
            s3 += l.to_int() & mask;
            s3.copy_to_slice(edges.get_unchecked_mut(o3..o3 + 32));

            off += 32;
        }

        while off + 32 + 1 <= 140 * 141 {
            let o1 = off;
            let o2 = off + 1;

            let b1 = u8x32::from_slice(input.get_unchecked(o1..o1 + 32));
            let b2 = u8x32::from_slice(input.get_unchecked(o2..o2 + 32));

            let m = b1.simd_ne(b2);

            let mut s1 = i8x32::from_slice(edges.get_unchecked(o1..o1 + 32));
            s1 += m.to_int() & i8x32::splat(1);
            *edges.get_unchecked_mut(o1) = s1[0];

            let mut s2 = s1.rotate_elements_left::<1>();
            s2[31] = *edges.get_unchecked(o2 + 32 - 1);
            s2 += m.to_int() & i8x32::splat(1);
            s2.copy_to_slice(edges.get_unchecked_mut(o2..o2 + 32));

            off += 32;
        }

        let extra = (off + 32 + 1) - 140 * 141;
        if extra != 32 {
            let mask = i8x32::from_slice(EXTRA_MASKS.get_unchecked(extra));
            off = 140 * 141 - (32 + 1);

            let o1 = off;
            let o2 = off + 1;

            let b1 = u8x32::from_slice(input.get_unchecked(o1..o1 + 32));
            let b2 = u8x32::from_slice(input.get_unchecked(o2..o2 + 32));

            let m = b1.simd_ne(b2);

            let mut s1 = i8x32::from_slice(edges.get_unchecked(o1..o1 + 32));
            s1 += m.to_int() & mask;
            *edges.get_unchecked_mut(o1) = s1[0];

            let mut s2 = s1.rotate_elements_left::<1>();
            s2[31] = *edges.get_unchecked(o2 + 32 - 1);
            s2 += m.to_int() & mask;
            s2.copy_to_slice(edges.get_unchecked_mut(o2..o2 + 32));
        }
    }

    #[cfg(debug_assertions)]
    {
        let mut expected = [0; 140 * 141];
        for y in 0..140 {
            for x in 0..141 {
                let c = input[141 * y + x];
                let mut n = 0;
                if x == 0 || input[141 * y + x - 1] != c {
                    n += 1;
                }
                if x == 140 || input[141 * y + x + 1] != c {
                    n += 1;
                }
                if y == 0 || input[141 * (y - 1) + x] != c {
                    n += 1;
                }
                if y == 139 || input[141 * (y + 1) + x] != c {
                    n += 1;
                }
                expected[141 * y + x] = n;
            }
        }

        for y in 0..140 {
            for x in 0..141 {
                assert_eq!(edges[141 * y + x], expected[141 * y + x], "x={x} y={y}");
            }
        }
    }

    // TODO: bitset seen and/or simd search?
    let mut tot = 0;
    let mut off = 0;
    let mut seen = [false; 140 * 141];
    let mut stack = [MaybeUninit::uninit(); 128];
    let mut stack_len = 0;

    for _ in 0..140 {
        for _ in 0..140 {
            if *seen.get_unchecked(off) {
                off += 1;
                continue;
            }
            *seen.get_unchecked_mut(off) = true;

            let c = *input.get_unchecked(off);
            let mut curr_off = off;
            let mut area = 0;
            let mut perimeter = 0;

            loop {
                area += 1;
                perimeter += *edges.get_unchecked(curr_off) as u8 as u64;

                macro_rules! handle {
                    ($off:expr) => {{
                        let new_off = $off;
                        if !*seen.get_unchecked(new_off) && *input.get_unchecked(new_off) == c {
                            *seen.get_unchecked_mut(new_off) = true;
                            *stack.get_unchecked_mut(stack_len).as_mut_ptr() = new_off;
                            stack_len += 1;
                        }
                    }};
                }

                handle!(curr_off + 1);
                if curr_off + 141 < 140 * 141 {
                    handle!(curr_off + 141);
                }
                if curr_off.wrapping_sub(141) < 140 * 141 {
                    handle!(curr_off - 141);
                    handle!(curr_off - 1);
                } else if curr_off.wrapping_sub(1) < 140 * 141 {
                    handle!(curr_off - 1);
                }

                if stack_len == 0 {
                    break;
                } else {
                    stack_len -= 1;
                    curr_off = *stack.get_unchecked(stack_len).as_ptr();
                }
            }

            tot += area * perimeter;
            off += 1;
        }
        off += 1;
    }

    tot
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    0
}
