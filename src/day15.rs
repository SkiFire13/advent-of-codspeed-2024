#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

use std::arch::x86_64::*;
use std::mem::MaybeUninit;
use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

#[inline(always)]
pub fn part1(input: &str) -> u32 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> u32 {
    unsafe { inner_part2(input) }
}

static D1_LUTX: [i8; 128] = {
    let mut lut = [0; 128];
    lut[b'<' as usize] = -1;
    lut[b'>' as usize] = 1;
    lut
};
static D1_LUTY: [i8; 128] = {
    let mut lut = [0; 128];
    lut[b'^' as usize] = -1;
    lut[b'v' as usize] = 1;
    lut
};

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut offset = 0;
    let (mut x, mut y) = loop {
        let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
        let mask = block.simd_eq(u8x64::splat(b'@'));
        if let Some(idx) = mask.first_set() {
            offset += idx;
            break (offset % 51, offset / 51);
        }
        offset += 64;
    };

    let mut wallsh = MaybeUninit::<[u64; 50]>::uninit();
    let mut boxesh = MaybeUninit::<[u64; 50]>::uninit();

    *wallsh.as_mut_ptr().cast::<u64>().add(0) = u64::MAX;
    *wallsh.as_mut_ptr().cast::<u64>().add(49) = u64::MAX;
    *boxesh.as_mut_ptr().cast::<u64>().add(0) = 0;
    *boxesh.as_mut_ptr().cast::<u64>().add(49) = 0;

    let mut i = 1;
    let mut offset = 51;
    loop {
        let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
        let maskw = block.simd_eq(u8x64::splat(b'#')).to_bitmask();
        let maskb = block.simd_eq(u8x64::splat(b'O')).to_bitmask();
        *wallsh.as_mut_ptr().cast::<u64>().add(i) = maskw;
        *boxesh.as_mut_ptr().cast::<u64>().add(i) = maskb;

        i += 1;
        offset += 51;
        if i == 49 {
            break;
        }
    }

    let wallsh = wallsh.assume_init_ref();
    let boxesh = boxesh.assume_init_mut();

    let mut boxesv = MaybeUninit::<[u64; 50]>::uninit();
    *boxesv.as_mut_ptr().cast::<u64>().add(0) = 0;
    *boxesv.as_mut_ptr().cast::<u64>().add(49) = 0;
    for x in 1..49 {
        let mut b = 0;
        for y in 1..49 {
            b |= ((boxesh.get_unchecked(y) >> x) & 1) << y;
        }
        *boxesv.as_mut_ptr().cast::<u64>().add(x) = b;
    }
    let boxesv = boxesv.assume_init_mut();

    let mut ptr = input.as_ptr().add(50 * 51 + 1).sub(1);
    let mut ptr_goal;
    let ptr_goal_end = ptr.add(20 * 1001);
    loop {
        ptr_goal = ptr.add(1001);
        loop {
            ptr = ptr.add(1);
            if ptr == ptr_goal {
                break;
            }

            let instr = *ptr;
            let dx = *D1_LUTX.get_unchecked(instr as usize) as isize;
            let dy = *D1_LUTY.get_unchecked(instr as usize) as isize;
            let (nx, ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));

            if (*boxesh.get_unchecked(ny) | *wallsh.get_unchecked(ny)) & (1 << nx) == 0 {
                (x, y) = (nx, ny);
                continue;
            }

            if *wallsh.get_unchecked(ny) & (1 << nx) != 0 {
                continue;
            }

            let (mut sx, mut sy) = (nx, ny);

            match instr {
                b'<' => sx -= (boxesh.get_unchecked(ny) << (63 - nx)).leading_ones() as usize,
                b'>' => sx += (boxesh.get_unchecked(ny) >> nx).trailing_ones() as usize,
                b'^' => sy -= (boxesv.get_unchecked(nx) << (63 - ny)).leading_ones() as usize,
                b'v' => sy += (boxesv.get_unchecked(nx) >> ny).trailing_ones() as usize,
                _ => std::hint::unreachable_unchecked(),
            }

            if *wallsh.get_unchecked(sy) & (1 << sx) == 0 {
                *boxesh.get_unchecked_mut(sy) |= 1 << sx;
                *boxesh.get_unchecked_mut(ny) &= !(1 << nx);
                *boxesv.get_unchecked_mut(sx) |= 1 << sy;
                *boxesv.get_unchecked_mut(nx) &= !(1 << ny);
                (x, y) = (nx, ny);
            }
        }
        if ptr_goal == ptr_goal_end {
            break;
        }
    }

    static X_ADD1: [i8; 32] = {
        let mut add = [0; 32];
        let mut i = 0;
        while i < 32 {
            add[i] = i as i8;
            i += 1;
        }
        add
    };
    static X_ADD2: [i8; 32] = {
        let mut add = [0; 32];
        let mut i = 32;
        while i < 64 {
            add[i - 32] = i as i8;
            i += 1;
        }
        add
    };

    let mut tot_y = u64x4::splat(0);
    let mut tot_x = u64x4::splat(0);
    let zero = i8x32::splat(0);
    for y in 1..49 {
        let b = *boxesh.get_unchecked(y) & (u64::MAX >> (64 - 50));
        let y = i8x32::splat(y as i8);

        let m1 = mask8x32::from_bitmask(b as u32 as u64).to_int();
        let m2 = mask8x32::from_bitmask((b >> 32) & (u64::MAX >> (64 - 50 + 32))).to_int();

        let x = (i8x32::from_slice(&X_ADD1) & m1) + (i8x32::from_slice(&X_ADD2) & m2);
        tot_x += u64x4::from(_mm256_sad_epu8(x.into(), zero.into()));

        let y = (y & m1) + (y & m2);
        tot_y += u64x4::from(_mm256_sad_epu8(y.into(), zero.into()));
    }
    (u64x4::splat(100) * tot_y + tot_x).reduce_sum() as u32
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut offset = 0;
    let (mut x, mut y) = loop {
        let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
        let mask = block.simd_eq(u8x64::splat(b'@'));
        if let Some(idx) = mask.first_set() {
            offset += idx;
            break (2 * (offset % 51), offset / 51);
        }
        offset += 64;
    };

    let mut wallsh = MaybeUninit::<[u128; 50]>::uninit();
    let mut boxesh = MaybeUninit::<[u128; 50]>::uninit();

    *wallsh.as_mut_ptr().cast::<u128>().add(0) = u128::MAX;
    *wallsh.as_mut_ptr().cast::<u128>().add(49) = u128::MAX;
    *boxesh.as_mut_ptr().cast::<u128>().add(0) = 0;
    *boxesh.as_mut_ptr().cast::<u128>().add(49) = 0;

    let mut i = 1;
    let mut offset = 51;
    loop {
        let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
        let maskw = block.simd_eq(u8x64::splat(b'#')).to_int();
        let maskb = block.simd_eq(u8x64::splat(b'O')).to_int();
        let (maskw1, maskw2) = maskw.interleave(maskw);
        let (maskb1, maskb2) = maskb.interleave(i8x64::splat(0));
        let maskw1 = mask8x64::from_int_unchecked(maskw1).to_bitmask();
        let maskw2 = mask8x64::from_int_unchecked(maskw2).to_bitmask();
        let maskb1 = mask8x64::from_int_unchecked(maskb1).to_bitmask();
        let maskb2 = mask8x64::from_int_unchecked(maskb2).to_bitmask();
        *wallsh.as_mut_ptr().cast::<u128>().add(i) = (maskw1 as u128) | ((maskw2 as u128) << 64);
        *boxesh.as_mut_ptr().cast::<u128>().add(i) = (maskb1 as u128) | ((maskb2 as u128) << 64);

        i += 1;
        offset += 51;
        if i == 49 {
            break;
        }
    }

    let wallsh = wallsh.assume_init_ref();
    let boxesh = boxesh.assume_init_mut();

    let mut ptr = input.as_ptr().add(50 * 51 + 1).sub(1);
    let mut ptr_goal;
    let ptr_goal_end = ptr.add(20 * 1001);
    loop {
        ptr_goal = ptr.add(1001);
        'mov: loop {
            ptr = ptr.add(1);
            if ptr == ptr_goal {
                break;
            }

            let instr = *ptr;
            let dx = *D1_LUTX.get_unchecked(instr as usize) as isize;
            let dy = *D1_LUTY.get_unchecked(instr as usize) as isize;
            let (nx, mut ny) = (x.wrapping_add_signed(dx), y.wrapping_add_signed(dy));

            let occupied = *boxesh.get_unchecked(ny)
                | (*boxesh.get_unchecked(ny) << 1)
                | *wallsh.get_unchecked(ny);

            if occupied & (1 << nx) == 0 {
                (x, y) = (nx, ny);
                continue;
            }

            if *wallsh.get_unchecked(ny) & (1 << nx) != 0 {
                continue;
            }

            let (mut sx, mut sy) = (nx, ny);

            match instr {
                b'<' => {
                    let boxes = boxesh.get_unchecked(ny);
                    let occupied = boxes | (boxes << 1);
                    let len = (occupied << (127 - nx)).leading_ones() as usize;
                    let mask = ((1 << len) - 1) << (nx - len);
                    let newm = u128::from_ne_bytes([0b01010101; 16]) << (nx - len);
                    sx -= len;

                    if *wallsh.get_unchecked(sy) & (1 << sx) == 0 {
                        *boxesh.get_unchecked_mut(sy) &= !mask;
                        *boxesh.get_unchecked_mut(ny) |= newm & mask;
                        (x, y) = (nx, ny);
                    }
                }
                b'>' => {
                    let boxes = boxesh.get_unchecked(ny);
                    let occupied = boxes | (boxes << 1);
                    let len = (occupied >> nx).trailing_ones() as usize;
                    let mask = ((1u128 << len) - 1) << nx;
                    let newm = u128::from_ne_bytes([0b01010101; 16]) << (nx + 1);
                    sx += len;

                    if *wallsh.get_unchecked(sy) & (1 << sx) == 0 {
                        *boxesh.get_unchecked_mut(sy) &= !mask;
                        *boxesh.get_unchecked_mut(ny) |= newm & mask;
                        (x, y) = (nx, ny);
                    }
                }
                b'^' => {
                    let mut hit = 1 << nx;
                    let boxes_hit = *boxesh.get_unchecked(sy) & (hit | (hit >> 1));
                    hit = boxes_hit | (boxes_hit << 1);
                    loop {
                        sy -= 1;
                        if (*wallsh.get_unchecked(sy) & hit) != 0 {
                            continue 'mov;
                        }
                        let boxes_hit = *boxesh.get_unchecked(sy) & (hit | (hit >> 1));
                        if boxes_hit == 0 {
                            break;
                        }
                        hit = boxes_hit | (boxes_hit << 1);
                    }

                    (x, y) = (nx, ny);

                    let mut boxes_hit = 0;
                    let mut hit = 0b11 << (nx - 1);

                    loop {
                        let old_ny = *boxesh.get_unchecked(ny);
                        *boxesh.get_unchecked_mut(ny) = (old_ny & !hit) | boxes_hit;
                        boxes_hit = old_ny & hit;
                        hit = boxes_hit | (boxes_hit << 1) | (boxes_hit >> 1);
                        if ny == sy {
                            break;
                        }
                        ny -= 1;
                    }
                }
                b'v' => {
                    let mut hit = 1 << nx;
                    let boxes_hit = *boxesh.get_unchecked(sy) & (hit | (hit >> 1));
                    hit = boxes_hit | (boxes_hit << 1);
                    loop {
                        sy += 1;
                        if (*wallsh.get_unchecked(sy) & hit) != 0 {
                            continue 'mov;
                        }
                        let boxes_hit = *boxesh.get_unchecked(sy) & (hit | (hit >> 1));
                        if boxes_hit == 0 {
                            break;
                        }
                        hit = boxes_hit | (boxes_hit << 1);
                    }

                    (x, y) = (nx, ny);

                    let mut boxes_hit = 0;
                    let mut hit = 0b11 << (nx - 1);

                    loop {
                        let old_ny = *boxesh.get_unchecked(ny);
                        *boxesh.get_unchecked_mut(ny) = (old_ny & !hit) | boxes_hit;
                        boxes_hit = old_ny & hit;
                        hit = boxes_hit | (boxes_hit << 1) | (boxes_hit >> 1);
                        if ny == sy {
                            break;
                        }
                        ny += 1;
                    }
                }
                _ => std::hint::unreachable_unchecked(),
            }
        }
        if ptr_goal == ptr_goal_end {
            break;
        }
    }

    static X_ADD: [i8x32; 4] = {
        let mut add1 = [0; 32];
        let mut i = 0;
        while i < 32 {
            add1[i] = i as i8;
            i += 1;
        }

        let mut add2 = [0; 32];
        while i < 64 {
            add2[i - 32] = i as i8;
            i += 1;
        }

        let mut add3 = [0; 32];
        while i < 96 {
            add3[i - 64] = i as i8;
            i += 1;
        }

        let mut add4 = [0; 32];
        while i < 128 {
            add4[i - 96] = i as i8;
            i += 1;
        }

        [
            i8x32::from_array(add1),
            i8x32::from_array(add2),
            i8x32::from_array(add3),
            i8x32::from_array(add4),
        ]
    };

    let mut tot_y = u64x4::splat(0);
    let mut tot_x = u64x4::splat(0);
    let zero = i8x32::splat(0);
    for y in 1..49 {
        let by = i8x32::splat(y as i8);

        let b = *boxesh.get_unchecked(y);
        let b1 = b as u32;
        let b2 = (b >> 32) as u32;
        let b3 = (b >> 64) as u32;
        let b4 = (b >> 96) as u32 & 0b1111;

        let m1 = mask8x32::from_bitmask(b1 as u64).to_int();
        let m2 = mask8x32::from_bitmask(b2 as u64).to_int();

        let bx12 = (X_ADD[0] & m1) + (X_ADD[1] & m2);
        tot_x += u64x4::from(_mm256_sad_epu8(bx12.into(), zero.into()));

        let by12 = (by & m1) + (by & m2);
        tot_y += u64x4::from(_mm256_sad_epu8(by12.into(), zero.into()));

        let m3 = mask8x32::from_bitmask(b3 as u64).to_int();
        let m4 = mask8x32::from_bitmask(b4 as u64).to_int();

        let bx34 = (X_ADD[2] & m3) + (X_ADD[3] & m4);
        tot_x += u64x4::from(_mm256_sad_epu8(bx34.into(), zero.into()));

        let by34 = (by & m3) + (by & m4);
        tot_y += u64x4::from(_mm256_sad_epu8(by34.into(), zero.into()));
    }
    (u64x4::splat(100) * tot_y + tot_x).reduce_sum() as u32
}
