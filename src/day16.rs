#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

use std::mem::MaybeUninit;

pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

#[inline(always)]
pub fn part1(input: &str) -> u32 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> u32 {
    unsafe { inner_part2(input) }
}

const START: u32 = 142 * 139 + 1;
const END: u32 = 142 * 1 + 139;

const UP: usize = -142isize as _;
const DOWN: usize = 142;
const LEFT: usize = -1isize as _;
const RIGHT: usize = 1;

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut seen_h = [0u64; (2 * 141 * 142 + 63) / 64];
    let mut seen_v = [0u64; (2 * 141 * 142 + 63) / 64];

    let mut queue_h = [MaybeUninit::uninit(); 128];
    let mut queue_v = [MaybeUninit::uninit(); 128];

    let mut queue_h_len = 1;
    let mut queue_v_len = 1;

    *queue_h.get_unchecked_mut(0).as_mut_ptr() = (START, 0);
    *queue_v.get_unchecked_mut(0).as_mut_ptr() = (START, 1000);

    let mut end_cost = u32::MAX;

    macro_rules! advance {
        ($pos:ident, $cost:ident, $dir:ident, $pdir1:ident, $pdir2:ident, $queue:ident, $queue_len:ident, $seen:ident) => {{
            let mut next_pos = $pos as usize;
            let mut next_cost = $cost;
            'advance: loop {
                next_pos = next_pos.wrapping_add($dir);
                next_cost += 1;

                if *input.get_unchecked(next_pos) != b'.' {
                    if next_pos as u32 == END {
                        end_cost = end_cost.min(next_cost);
                    }
                    break;
                }

                let up = next_pos.wrapping_add($pdir1);
                let down = next_pos.wrapping_add($pdir2);

                if *input.get_unchecked(up) != b'#' || *input.get_unchecked(down) != b'#' {
                    if *$seen.get_unchecked(next_pos / 64) & (1 << (next_pos % 64)) == 0 {
                        let idx = 2 * next_pos;
                        if $seen.get_unchecked(idx / 64) & (1 << (idx % 64)) != 0 {
                            for i in 0..$queue_len {
                                let (old_pos, old_cost) =
                                    &mut *$queue.get_unchecked_mut(i).as_mut_ptr();
                                if *old_pos == next_pos as u32 {
                                    *old_cost = (*old_cost).min(next_cost + 1000);
                                    continue 'advance;
                                }
                            }
                        }
                        *$seen.get_unchecked_mut(idx / 64) |= 1 << (idx % 64);
                        let ptr = $queue.get_unchecked_mut($queue_len).as_mut_ptr();
                        *ptr = (next_pos as u32, next_cost + 1000);
                        $queue_len += 1;
                    }
                }
            }
        }};
    }

    loop {
        for i in 0..std::mem::take(&mut queue_h_len) {
            let (pos, cost) = *queue_h.get_unchecked(i).as_ptr();
            *seen_h.get_unchecked_mut(pos as usize / 64) |= 1 << (pos % 64);
            advance!(pos, cost, LEFT, UP, DOWN, queue_v, queue_v_len, seen_v);
            advance!(pos, cost, RIGHT, UP, DOWN, queue_v, queue_v_len, seen_v);
        }

        if end_cost != u32::MAX {
            return end_cost;
        }

        for i in 0..std::mem::take(&mut queue_v_len) {
            let (pos, cost) = *queue_v.get_unchecked(i).as_ptr();
            *seen_v.get_unchecked_mut(pos as usize / 64) |= 1 << (pos % 64);
            advance!(pos, cost, UP, LEFT, RIGHT, queue_h, queue_h_len, seen_h);
            advance!(pos, cost, DOWN, LEFT, RIGHT, queue_h, queue_h_len, seen_h);
        }

        if end_cost != u32::MAX {
            return end_cost;
        }
    }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u32 {
    0
}
