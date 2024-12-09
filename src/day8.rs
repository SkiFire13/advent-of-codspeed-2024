#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]

use std::mem::MaybeUninit;
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
    let input = input.as_bytes();
    let mut positions = [[MaybeUninit::<(u32, u32)>::uninit(); 4]; 128];
    let mut lengths = [0; 128];

    let mut marked = [1u8; 64 * 50];
    let mut count = 0;

    for y in 0..50 {
        let offset = 51 * y;

        let block1 = u8x32::from_slice(input.get_unchecked(offset..offset + 32));
        let mask1 = block1.simd_ne(u8x32::splat(b'.')).to_bitmask();
        let block2 = u8x32::from_slice(input.get_unchecked(offset + 50 - 32..offset + 50));
        let mask2 = block2.simd_ne(u8x32::splat(b'.')).to_bitmask() << 18;
        let mut mask = mask1 | mask2;

        while mask != 0 {
            let x = mask.trailing_zeros();
            mask &= !(1 << x);

            let b = *input.get_unchecked(offset + x as usize);
            let len = lengths.get_unchecked_mut(b as usize);
            let poss = positions.get_unchecked_mut(b as usize);
            poss.get_unchecked_mut(*len).write((x as u32, y as u32));
            *len += 1;

            let (xi, yi) = (x as u32, y as u32);
            for p in poss.get_unchecked(0..*len - 1) {
                let (xj, yj) = p.assume_init();

                let (xa, ya) = ((2 * xi).wrapping_sub(xj), (2 * yi).wrapping_sub(yj));
                if xa < 50 && ya < 50 {
                    count += *marked.get_unchecked((xa as usize * 64) + ya as usize) as u64;
                    *marked.get_unchecked_mut((xa as usize * 64) + ya as usize) = 0;
                }

                let (xa, ya) = ((2 * xj).wrapping_sub(xi), (2 * yj).wrapping_sub(yi));
                if xa < 50 && ya < 50 {
                    count += *marked.get_unchecked((xa as usize * 64) + ya as usize) as u64;
                    *marked.get_unchecked_mut((xa as usize * 64) + ya as usize) = 0;
                }
            }
        }
    }

    count
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut positions = [[MaybeUninit::<(u32, u32)>::uninit(); 4]; 128];
    let mut lengths = [0; 128];

    for y in 0..50 {
        let offset = 51 * y;
        let block1 = u8x32::from_slice(input.get_unchecked(offset..offset + 32));
        let mask1 = block1.simd_ne(u8x32::splat(b'.')).to_bitmask();
        let block2 = u8x32::from_slice(input.get_unchecked(offset + 50 - 32..offset + 50));
        let mask2 = block2.simd_ne(u8x32::splat(b'.')).to_bitmask() << 18;
        let mut mask = mask1 | mask2;

        while mask != 0 {
            let x = mask.trailing_zeros();
            mask &= !(1 << x);

            let b = *input.get_unchecked(offset + x as usize);
            let len = lengths.get_unchecked_mut(b as usize);
            let poss = positions.get_unchecked_mut(b as usize);
            poss.get_unchecked_mut(*len).write((x as u32, y as u32));
            *len += 1;
        }
    }

    let mut marked = [1u8; 50 * 50];
    let mut count = 0;
    for (&len, poss) in std::iter::zip(&lengths, &positions) {
        for (i, pi) in poss.get_unchecked(0..len).iter().enumerate() {
            let (xi, yi) = pi.assume_init();
            for pj in poss.get_unchecked(i + 1..len) {
                let (xj, yj) = pj.assume_init();
                let dx = xj.wrapping_sub(xi);
                let di = ((yj as isize - yi as isize) * 50 + dx as i32 as isize) as usize;

                let (mut ia, mut xa) = ((yi as usize * 50) + xi as usize, xi);
                while ia < 50 * 50 && xa < 50 {
                    count += *marked.get_unchecked(ia) as u64;
                    *marked.get_unchecked_mut(ia) = 0;
                    (ia, xa) = (ia.wrapping_sub(di), xa.wrapping_sub(dx));
                }

                let (mut ia, mut xa) = ((yj as usize * 50) + xj as usize, xj);
                while ia < 50 * 50 && xa < 50 {
                    count += *marked.get_unchecked(ia) as u64;
                    *marked.get_unchecked_mut(ia) = 0;
                    (ia, xa) = (ia.wrapping_add(di), xa.wrapping_add(dx));
                }
            }
        }
    }

    count
}
