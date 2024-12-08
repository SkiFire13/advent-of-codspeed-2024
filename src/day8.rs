#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]

use std::mem::MaybeUninit;
use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part2(input) as i64
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
    let input = input.as_bytes();
    let mut positions = [[MaybeUninit::<(u8, u8)>::uninit(); 4]; 128];
    let mut lengths = [0; 128];

    let mut marked = [false; 64 * 50];
    let mut count = 0;

    let mut y = 0;

    loop {
        let offset = 51 * y;
        let mut mask;
        if y < 49 {
            let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
            mask = block.simd_ne(u8x64::splat(b'.')).to_bitmask() & ((1 << 50) - 1);
        } else if y == 49 {
            let block = u8x64::from_slice(input.get_unchecked(input.len() - 65..));
            mask = block.simd_ne(u8x64::splat(b'.')).to_bitmask() >> 14;
        } else {
            break;
        }

        while mask != 0 {
            let x = mask.trailing_zeros();
            mask &= !(1 << x);

            let b = *input.get_unchecked(offset + x as usize);
            let len = lengths.get_unchecked_mut(b as usize);
            let poss = positions.get_unchecked_mut(b as usize);
            let (xi, yi) = (x as u8, y as u8);
            for j in 0..*len {
                let (xj, yj) = poss.get_unchecked(j).assume_init();
                let (dx, dy) = (xj.wrapping_sub(xi), yj.wrapping_sub(yi));

                let (xa, ya) = (xi.wrapping_sub(dx), yi.wrapping_sub(dy));
                if xa < 50 && ya < 50 && !marked.get_unchecked((xa as usize * 64) + ya as usize) {
                    *marked.get_unchecked_mut((xa as usize * 64) + ya as usize) = true;
                    count += 1;
                }

                let (xa, ya) = (xj.wrapping_add(dx), yj.wrapping_add(dy));
                if xa < 50 && ya < 50 && !marked.get_unchecked((xa as usize * 64) + ya as usize) {
                    *marked.get_unchecked_mut((xa as usize * 64) + ya as usize) = true;
                    count += 1;
                }
            }

            *poss.get_unchecked_mut(*len) = MaybeUninit::new((x as u8, y as u8));
            *len += 1;
        }

        y += 1;
    }

    count
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut positions = [[MaybeUninit::<(u8, u8)>::uninit(); 8]; 128];
    let mut lengths = [0; 128];

    let mut y = 0;

    loop {
        let offset = 51 * y;
        let mut mask;
        if y < 49 {
            let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
            mask = block.simd_ne(u8x64::splat(b'.')).to_bitmask() & ((1 << 50) - 1);
        } else if y == 49 {
            let block = u8x64::from_slice(input.get_unchecked(input.len() - 65..));
            mask = block.simd_ne(u8x64::splat(b'.')).to_bitmask() >> 14;
        } else {
            break;
        }

        while mask != 0 {
            let x = mask.trailing_zeros();
            mask &= !(1 << x);

            let b = *input.get_unchecked(offset + x as usize);
            let len = lengths.get_unchecked_mut(b as usize);
            let poss = positions.get_unchecked_mut(b as usize);
            *poss.get_unchecked_mut(*len) = MaybeUninit::new((x as u8, y as u8));
            *len += 1;
        }

        y += 1;
    }

    let mut marked = [false; 64 * 50];
    let mut count = 0;
    for (&len, positions) in std::iter::zip(&lengths, &positions) {
        for i in 0..len {
            let (xi, yi) = positions.get_unchecked(i).assume_init();
            for j in i + 1..len {
                let (xj, yj) = positions.get_unchecked(j).assume_init();
                let dx = xj.wrapping_sub(xi);
                let di = ((yj as i8 - yi as i8) as isize * 64 + dx as i8 as isize) as usize;

                let (mut ia, mut xa) = ((yi as usize * 64) + xi as usize, xi);
                while ia < 64 * 50 && xa < 50 {
                    count += !marked.get_unchecked(ia) as u64;
                    *marked.get_unchecked_mut(ia) = true;
                    (ia, xa) = (ia.wrapping_sub(di), xa.wrapping_sub(dx));
                }

                let (mut ia, mut xa) = ((yj as usize * 64) + xj as usize, xj);
                while ia < 64 * 50 && xa < 50 {
                    count += !marked.get_unchecked(ia) as u64;
                    *marked.get_unchecked_mut(ia) = true;
                    (ia, xa) = (ia.wrapping_add(di), xa.wrapping_add(dx));
                }
            }
        }
    }

    count
}
