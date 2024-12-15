#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

#[inline(always)]
pub fn part1(input: &str) -> u32 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut offset = 0;
    let mut pos = loop {
        let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
        let mask = block.simd_eq(u8x64::splat(b'@'));
        if let Some(idx) = mask.first_set() {
            break offset + idx;
        }
        offset += 64;
    };

    let mut grid = *input.as_ptr().cast::<[u8; 50 * 51 + (64 - 51)]>();
    *grid.get_unchecked_mut(pos) = b'.';

    static LUT: [i8; 128] = {
        let mut lut = [0; 128];
        lut[b'<' as usize] = -1;
        lut[b'>' as usize] = 1;
        lut[b'^' as usize] = -51;
        lut[b'v' as usize] = 51;
        lut
    };

    for &instr in input.get_unchecked(50 * 51 + 1..) {
        let d = *LUT.get_unchecked(instr as usize) as isize;
        let new_pos = pos.wrapping_add_signed(d);
        let mut search_pos = new_pos;
        while *grid.get_unchecked(search_pos) == b'O' {
            search_pos = search_pos.wrapping_add_signed(d);
        }
        if *grid.get_unchecked(search_pos) == b'.' {
            *grid.get_unchecked_mut(search_pos) = b'O';
            *grid.get_unchecked_mut(new_pos) = b'.';
            pos = new_pos;
        }
    }

    let mut tot = 0;
    let mut offset = 51;
    let mut y = 1;
    loop {
        let block = u8x64::from_slice(grid.get_unchecked(offset..offset + 64));
        let mut mask = block.simd_eq(u8x64::splat(b'O')).to_bitmask() & (u64::MAX >> (64 - 50));
        while mask != 0 {
            let x = mask.trailing_zeros();
            mask &= !(1 << x);
            tot += 100 * y + x;
        }

        y += 1;
        offset += 51;
        if y == 49 {
            break;
        }
    }

    tot
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    0
}
