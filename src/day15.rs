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

static D2_LUT: [i8; 128] = {
    let mut lut = [0; 128];
    lut[b'<' as usize] = -1;
    lut[b'>' as usize] = 1;
    lut[b'^' as usize] = -100;
    lut[b'v' as usize] = 100;
    lut
};

static GRID_LUT: [[u8; 2]; 128] = {
    let mut lut = [[0; 2]; 128];
    lut[b'.' as usize] = [b'.', b'.'];
    lut[b'O' as usize] = [b'[', b']'];
    lut[b'#' as usize] = [b'#', b'#'];
    lut[b'@' as usize] = [b'.', b'.'];
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
    let mut pos = loop {
        let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
        let mask = block.simd_eq(u8x64::splat(b'@'));
        if let Some(idx) = mask.first_set() {
            let offset = offset + idx;
            let (x, y) = (offset % 51, offset / 51);
            break 100 * y + 2 * x;
        }
        offset += 64;
    };

    let mut grid = MaybeUninit::<[u8; 50 * 100 + (128 - 100)]>::uninit();

    let mut grid_ptr = grid.as_mut_ptr().cast::<[u8; 2]>();
    let mut input_ptr = input.as_ptr();
    let mut input_ptr_goal = input_ptr.add(50);
    let input_ptr_end = input_ptr.add(51 * 50 - 1 + 51);
    loop {
        loop {
            *grid_ptr = *GRID_LUT.get_unchecked(*input_ptr as usize);
            input_ptr = input_ptr.add(1);
            grid_ptr = grid_ptr.add(1);
            if input_ptr == input_ptr_goal {
                break;
            }
        }
        input_ptr = input_ptr.add(1);
        input_ptr_goal = input_ptr_goal.add(51);
        if input_ptr_goal == input_ptr_end {
            break;
        }
    }
    let grid = grid.assume_init_mut();

    'instr: for &instr in input.get_unchecked(50 * 51 + 1..) {
        let d = *D2_LUT.get_unchecked(instr as usize) as isize as usize;
        let new_pos = pos.wrapping_add(d);
        if *grid.get_unchecked(new_pos) == b'.' {
            pos = new_pos;
            continue;
        }

        match instr {
            b'<' => {
                let d = -1isize as usize;
                let new_pos = pos.wrapping_add(d);
                let mut search_pos = new_pos;

                while *grid.get_unchecked(search_pos) == b']' {
                    search_pos = search_pos.wrapping_add(d).wrapping_add(d);
                }
                if *grid.get_unchecked(search_pos) == b'.' {
                    while search_pos != new_pos {
                        *grid.get_unchecked_mut(search_pos) = b'[';
                        *grid.get_unchecked_mut(search_pos.wrapping_sub(d)) = b']';
                        search_pos = search_pos.wrapping_sub(d).wrapping_sub(d);
                    }
                    *grid.get_unchecked_mut(new_pos) = b'.';
                    pos = new_pos;
                }
            }
            b'>' => {
                let d = 1isize as usize;
                let new_pos = pos.wrapping_add(d);
                let mut search_pos = new_pos;

                while *grid.get_unchecked(search_pos) == b'[' {
                    search_pos = search_pos.wrapping_add(d).wrapping_add(d);
                }
                if *grid.get_unchecked(search_pos) == b'.' {
                    while search_pos != new_pos {
                        *grid.get_unchecked_mut(search_pos) = b']';
                        *grid.get_unchecked_mut(search_pos.wrapping_sub(d)) = b'[';
                        search_pos = search_pos.wrapping_sub(d).wrapping_sub(d);
                    }
                    *grid.get_unchecked_mut(new_pos) = b'.';
                    pos = new_pos;
                }
            }
            b'^' | b'v' => {
                let mut queue = u16x64::splat(0);
                let mut queue_start = 0;
                let mut queue_end = 1;
                queue.as_mut_array()[0] = new_pos as u16;

                macro_rules! add {
                    ($pos:expr) => {{
                        let pos = $pos as u16;
                        if !queue.simd_eq(u16x64::splat(pos as u16)).any() {
                            *queue.as_mut_array().get_unchecked_mut(queue_end) = pos;
                            queue_end += 1;
                        }
                    }};
                }

                macro_rules! try_add {
                    ($pos:expr) => {{
                        let pos = $pos as u16;
                        if *grid.get_unchecked(pos as usize) != b'.' {
                            add!(pos);
                        }
                    }};
                }

                while queue_start != queue_end {
                    let search_pos = *queue.as_array().get_unchecked(queue_start) as usize;
                    queue_start += 1;
                    let b = *grid.get_unchecked(search_pos);

                    if b == b'#' {
                        continue 'instr;
                    } else if b == b'[' {
                        add!(search_pos + 1);
                        try_add!(search_pos.wrapping_add(d));
                        try_add!(search_pos.wrapping_add(d) + 1);
                    } else if b == b']' {
                        add!(search_pos - 1);
                        try_add!(search_pos.wrapping_add(d));
                        try_add!(search_pos.wrapping_add(d) - 1);
                    } else {
                        std::hint::unreachable_unchecked();
                    }
                }

                for &pos in queue.as_array().get_unchecked(..queue_end).iter().rev() {
                    let pos = pos as usize;
                    *grid.get_unchecked_mut(pos.wrapping_add(d)) = *grid.get_unchecked(pos);
                    *grid.get_unchecked_mut(pos) = b'.';
                }

                pos = new_pos;
            }
            _ => std::hint::unreachable_unchecked(),
        }
    }

    // TODO: One pass?
    let mut tot = 0;
    let mut offset = 100;
    let mut y = 1;
    loop {
        let block = u8x64::from_slice(grid.get_unchecked(offset..offset + 64));
        let mut mask = block.simd_eq(u8x64::splat(b'[')).to_bitmask();
        while mask != 0 {
            let x = mask.trailing_zeros();
            mask &= !(1 << x);
            tot += 100 * y + x;
        }

        let block = u8x64::from_slice(grid.get_unchecked(offset + 64..offset + 128));
        let mut mask = block.simd_eq(u8x64::splat(b'[')).to_bitmask() & (u64::MAX >> (128 - 100));
        while mask != 0 {
            let x = mask.trailing_zeros();
            mask &= !(1 << x);
            tot += 100 * y + (64 + x);
        }

        y += 1;
        offset += 100;
        if y == 49 {
            break;
        }
    }

    tot
}
