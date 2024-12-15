#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

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

static D1_LUT: [i8; 128] = {
    let mut lut = [0; 128];
    lut[b'<' as usize] = -1;
    lut[b'>' as usize] = 1;
    lut[b'^' as usize] = -64;
    lut[b'v' as usize] = 64;
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
    let mut pos = loop {
        let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
        let mask = block.simd_eq(u8x64::splat(b'@'));
        if let Some(idx) = mask.first_set() {
            offset += idx;
            let (x, y) = (offset % 51, offset / 51);
            break 64 * y + x;
        }
        offset += 64;
    };

    let mut walls = MaybeUninit::<[u64; 50]>::uninit();
    let mut boxes = MaybeUninit::<[u64; 50]>::uninit();

    *walls.as_mut_ptr().cast::<u64>().add(0) = u64::MAX;
    *walls.as_mut_ptr().cast::<u64>().add(49) = u64::MAX;
    *boxes.as_mut_ptr().cast::<u64>().add(0) = 0;
    *boxes.as_mut_ptr().cast::<u64>().add(49) = 0;

    let mut i = 1;
    let mut offset = 51;
    loop {
        let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
        let maskw = block.simd_eq(u8x64::splat(b'#')).to_bitmask() & (u64::MAX >> (64 - 50));
        let maskb = block.simd_eq(u8x64::splat(b'O')).to_bitmask() & (u64::MAX >> (64 - 50));
        *walls.as_mut_ptr().cast::<u64>().add(i) = maskw;
        *boxes.as_mut_ptr().cast::<u64>().add(i) = maskb;

        i += 1;
        offset += 51;
        if i == 49 {
            break;
        }
    }

    let walls = walls.assume_init_ref();
    let boxes = boxes.assume_init_mut();

    for &instr in input.get_unchecked(50 * 51 + 1..) {
        let d = *D1_LUT.get_unchecked(instr as usize) as isize;
        let new_pos = pos.wrapping_add_signed(d);
        let (nx, ny) = (new_pos & 0b111111, new_pos >> 6);

        if (*boxes.get_unchecked(ny) | *walls.get_unchecked(ny)) & (1 << nx) == 0 {
            pos = new_pos;
            continue;
        }

        if *walls.get_unchecked(ny) & (1 << nx) != 0 {
            continue;
        }

        match instr {
            b'<' => {
                let sx = nx - (boxes.get_unchecked(ny) << (63 - nx)).leading_ones() as usize;
                if *walls.get_unchecked(ny) & (1 << sx) == 0 {
                    *boxes.get_unchecked_mut(ny) |= 1 << sx;
                    *boxes.get_unchecked_mut(ny) &= !(1 << nx);
                    pos = new_pos;
                }
            }
            b'>' => {
                let sx = nx + (boxes.get_unchecked(ny) >> nx).trailing_ones() as usize;
                if *walls.get_unchecked(ny) & (1 << sx) == 0 {
                    *boxes.get_unchecked_mut(ny) |= 1 << sx;
                    *boxes.get_unchecked_mut(ny) &= !(1 << nx);
                    pos = new_pos;
                }
            }
            b'^' => {
                let mut sy = ny;
                while *boxes.get_unchecked(sy) & (1 << nx) != 0 {
                    sy -= 1;
                }
                if *walls.get_unchecked(sy) & (1 << nx) == 0 {
                    *boxes.get_unchecked_mut(sy) |= 1 << nx;
                    *boxes.get_unchecked_mut(ny) &= !(1 << nx);
                    pos = new_pos;
                }
            }
            b'v' => {
                let mut sy = ny;
                while *boxes.get_unchecked(sy) & (1 << nx) != 0 {
                    sy += 1;
                }
                if *walls.get_unchecked(sy) & (1 << nx) == 0 {
                    *boxes.get_unchecked_mut(sy) |= 1 << nx;
                    *boxes.get_unchecked_mut(ny) &= !(1 << nx);
                    pos = new_pos;
                }
            }
            b'\n' => {}
            _ => std::hint::unreachable_unchecked(),
        }
    }

    let mut tot = 0;
    for y in 1..49 {
        let mut b = *boxes.get_unchecked(y);
        while b != 0 {
            let x = b.trailing_zeros();
            b &= !(1 << x);
            tot += 100 * y as u32 + x;
        }
    }
    tot
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
