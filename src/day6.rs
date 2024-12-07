#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]

use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

pub fn part1(input: &str) -> u32 {
    unsafe { inner_part1(input) }
}

pub fn part2(input: &str) -> u32 {
    unsafe { inner_part2(input) }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut offset = 0;
    let start = loop {
        let block = u8x32::from_slice(input.get_unchecked(offset..offset + 32));
        if let Some(start_pos) = block.simd_eq(u8x32::splat(b'^')).first_set() {
            break offset + start_pos;
        }
        offset += 32;
    };

    let mut seen = [0u64; (130 * 131 + 63) / 64];
    let mut seen_count = 0;
    let mut pos = start;
    seen[pos / 64] |= 1 << (pos % 64);
    seen_count += 1;
    loop {
        loop {
            let next = pos.wrapping_add(-131isize as usize);
            if next >= 131 * 130 {
                return seen_count;
            }

            if *input.get_unchecked(next) == b'#' {
                break;
            }

            pos = next;

            let seen_elem = seen.get_unchecked_mut(pos / 64);
            let seen_mask = 1 << (pos % 64);
            if *seen_elem & seen_mask == 0 {
                *seen_elem |= seen_mask;
                seen_count += 1;
            }
        }

        loop {
            let next = pos.wrapping_add(1 as usize);
            if next % 131 == 130 {
                return seen_count;
            }

            if *input.get_unchecked(next) == b'#' {
                break;
            }

            pos = next;

            let seen_elem = seen.get_unchecked_mut(pos / 64);
            let seen_mask = 1 << (pos % 64);
            if *seen_elem & seen_mask == 0 {
                *seen_elem |= seen_mask;
                seen_count += 1;
            }
        }

        loop {
            let next = pos.wrapping_add(131 as usize);
            if next >= 131 * 130 {
                return seen_count;
            }

            if *input.get_unchecked(next) == b'#' {
                break;
            }

            pos = next;

            let seen_elem = seen.get_unchecked_mut(pos / 64);
            let seen_mask = 1 << (pos % 64);
            if *seen_elem & seen_mask == 0 {
                *seen_elem |= seen_mask;
                seen_count += 1;
            }
        }

        loop {
            let next = pos.wrapping_add(-1isize as usize);
            if next % 130 == 131 {
                return seen_count;
            }

            if *input.get_unchecked(next) == b'#' {
                break;
            }

            pos = next;

            let seen_elem = seen.get_unchecked_mut(pos / 64);
            let seen_mask = 1 << (pos % 64);
            if *seen_elem & seen_mask == 0 {
                *seen_elem |= seen_mask;
                seen_count += 1;
            }
        }
    }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut start = 0;

    let mut rocks_len = 0;
    let mut rocks_x = const { [0; 1024] };
    let mut rocks_y = const { [0; 1024] };
    let mut rocksx_id = const { [[0; 16]; 130] };
    let mut rocksx_len = const { [0; 130] };
    let mut rocksy_id = const { [[0; 16]; 130] };
    let mut rocksy_len = const { [0; 130] };

    // Parse the input into a list of rocks and the start position
    let mut offset = 0;
    while start == 0 {
        let block = u8x32::from_slice(input.get_unchecked(offset..offset + 32));

        let mut rocks_mask = block.simd_eq(u8x32::splat(b'#')).to_bitmask();
        while rocks_mask != 0 {
            let pos = rocks_mask.trailing_zeros();
            rocks_mask &= !(1 << pos);
            let pos = offset + pos as usize;
            let x = pos % 131;
            let y = pos / 131;

            *rocks_x.get_unchecked_mut(rocks_len) = x as u8;
            *rocks_y.get_unchecked_mut(rocks_len) = y as u8;

            let x_len = rocksx_len.get_unchecked_mut(x as usize);
            *rocksx_id
                .get_unchecked_mut(x as usize)
                .get_unchecked_mut(*x_len) = rocks_len as u16;
            *x_len += 1;

            let y_len = rocksy_len.get_unchecked_mut(y as usize);
            *rocksy_id
                .get_unchecked_mut(y as usize)
                .get_unchecked_mut(*y_len) = rocks_len as u16;
            *y_len += 1;

            rocks_len += 1;
        }

        if let Some(start_pos) = block.simd_eq(u8x32::splat(b'^')).first_set() {
            start = (offset + start_pos) as u32;
        }

        offset += 32;
    }
    while offset + 32 <= input.len() {
        let block = u8x32::from_slice(input.get_unchecked(offset..offset + 32));

        let mut rocks_mask = block.simd_eq(u8x32::splat(b'#')).to_bitmask();
        while rocks_mask != 0 {
            let pos = rocks_mask.trailing_zeros();
            rocks_mask &= !(1 << pos);
            let pos = offset + pos as usize;
            let x = pos % 131;
            let y = pos / 131;

            *rocks_x.get_unchecked_mut(rocks_len) = x as u8;
            *rocks_y.get_unchecked_mut(rocks_len) = y as u8;

            let x_len = rocksx_len.get_unchecked_mut(x as usize);
            *rocksx_id
                .get_unchecked_mut(x as usize)
                .get_unchecked_mut(*x_len) = rocks_len as u16;
            *x_len += 1;

            let y_len = rocksy_len.get_unchecked_mut(y as usize);
            *rocksy_id
                .get_unchecked_mut(y as usize)
                .get_unchecked_mut(*y_len) = rocks_len as u16;
            *y_len += 1;

            rocks_len += 1;
        }

        offset += 32;
    }
    while offset < input.len() {
        if *input.get_unchecked(offset) == b'#' {
            let pos = offset;
            let x = pos % 131;
            let y = pos / 131;

            *rocks_x.get_unchecked_mut(rocks_len) = x as u8;
            *rocks_y.get_unchecked_mut(rocks_len) = y as u8;

            let x_len = rocksx_len.get_unchecked_mut(x as usize);
            *rocksx_id
                .get_unchecked_mut(x as usize)
                .get_unchecked_mut(*x_len) = rocks_len as u16;
            *x_len += 1;

            let y_len = rocksy_len.get_unchecked_mut(y as usize);
            *rocksy_id
                .get_unchecked_mut(y as usize)
                .get_unchecked_mut(*y_len) = rocks_len as u16;
            *y_len += 1;

            rocks_len += 1;
        }

        offset += 1;
    }

    // A rock representing going out of bounds
    let out_rock_idx = rocks_len as usize + 1;

    // Map of moves.
    // Each move is encoded as a rock idx left-shifted by 2, plus the
    // position relative to the rock as the last 2 bits.
    const BOT_M: usize = 0b00;
    const TOP_M: usize = 0b01;
    const LEFT_M: usize = 0b10;
    const RIGHT_M: usize = 0b11;
    let mut move_map = [(out_rock_idx as u32) << 2; 1024 * 4];
    let move_map = &mut move_map[..(out_rock_idx + 1) << 2];

    fn mov(pos: usize, mask: usize) -> usize {
        (pos << 2) | mask
    }

    // Build next rock map for vertical-to-horizontal turns
    for y in 0..130 - 1 {
        let line1_ids = rocksy_id.get_unchecked(y as usize);
        let line1_len = *rocksy_len.get_unchecked(y as usize);
        let line2_ids = rocksy_id.get_unchecked(y as usize + 1);
        let line2_len = *rocksy_len.get_unchecked(y as usize + 1);

        let mut next1 = 0;
        let mut next2 = 0;

        let mut curr1i = out_rock_idx;
        let mut next1i = *line1_ids.get_unchecked(next1) as usize;
        let mut next2i = *line2_ids.get_unchecked(next2) as usize;

        loop {
            if next2 >= line2_len {
                // Line 2 has no rocks left
                while next1 < line1_len {
                    *move_map.get_unchecked_mut(mov(next1i, BOT_M)) =
                        mov(out_rock_idx, LEFT_M) as u32;
                    next1 += 1;
                    next1i = *line1_ids.get_unchecked(next1) as usize;
                }
                break;
            }

            while next1 < line1_len
                && rocks_x.get_unchecked(next1i) <= rocks_x.get_unchecked(next2i)
            {
                *move_map.get_unchecked_mut(mov(next1i, BOT_M)) = mov(next2i, LEFT_M) as u32;
                curr1i = next1i;
                next1 += 1;
                next1i = *line1_ids.get_unchecked(next1) as usize;
            }

            if next1 >= line1_len {
                // Line 1 has no rocks left
                while next2 < line2_len {
                    *move_map.get_unchecked_mut(mov(next2i, TOP_M)) = mov(curr1i, RIGHT_M) as u32;
                    next2 += 1;
                    next2i = *line2_ids.get_unchecked(next2) as usize;
                }
                break;
            }

            while next2 < line2_len
                && rocks_x.get_unchecked(next2i) <= rocks_x.get_unchecked(next1i)
            {
                *move_map.get_unchecked_mut(mov(next2i, TOP_M)) = mov(curr1i, RIGHT_M) as u32;
                next2 += 1;
                next2i = *line2_ids.get_unchecked(next2) as usize;
            }
        }
    }

    // Build next rock map for horizontal-to-vertical turns
    for x in 0..130 - 1 {
        let line1_ids = rocksx_id.get_unchecked(x as usize);
        let line1_len = *rocksx_len.get_unchecked(x as usize);
        let line2_ids = rocksx_id.get_unchecked(x as usize + 1);
        let line2_len = *rocksx_len.get_unchecked(x as usize + 1);

        let mut next1 = 0;
        let mut next2 = 0;

        let mut curr2i = out_rock_idx;
        let mut next1i = *line1_ids.get_unchecked(next1) as usize;
        let mut next2i = *line2_ids.get_unchecked(next2) as usize;

        loop {
            if next2 >= line2_len {
                // Line 2 has no rocks left
                while next1 < line1_len {
                    *move_map.get_unchecked_mut(mov(next1i, RIGHT_M)) = mov(curr2i, BOT_M) as u32;
                    next1 += 1;
                    next1i = *line1_ids.get_unchecked(next1) as usize;
                }
                break;
            }

            while next1 < line1_len
                && rocks_y.get_unchecked(next1i) <= rocks_y.get_unchecked(next2i)
            {
                *move_map.get_unchecked_mut(mov(next1i, RIGHT_M)) = mov(curr2i, BOT_M) as u32;
                next1 += 1;
                next1i = *line1_ids.get_unchecked(next1) as usize;
            }

            if next1 >= line1_len {
                // Line 1 has no rocks left
                while next2 < line2_len {
                    *move_map.get_unchecked_mut(mov(next2i, LEFT_M)) =
                        mov(out_rock_idx, TOP_M) as u32;
                    next2 += 1;
                    next2i = *line2_ids.get_unchecked(next2) as usize;
                }
                break;
            }

            while next2 < line2_len
                && rocks_y.get_unchecked(next2i) <= rocks_y.get_unchecked(next1i)
            {
                *move_map.get_unchecked_mut(mov(next2i, LEFT_M)) = mov(next1i, TOP_M) as u32;
                curr2i = next2i;
                next2 += 1;
                next2i = *line2_ids.get_unchecked(next2) as usize;
            }
        }
    }

    let startx = start % 131;
    let starty = start / 131;
    let mut pos = start as usize;

    let rockx_idx = rocksx_id[startx as usize][..rocksx_len[startx as usize]]
        .partition_point(|&id| rocks_y[id as usize] as (u32) < starty)
        - 1;
    let rock_id = rocksx_id[startx as usize][rockx_idx as usize];
    let rock_pos = rocks_y[rock_id as usize] as usize * 131 + startx as usize;

    let mut count = 0;
    let mut seen = [0u64; (130 * 131 + 63) / 64];
    seen[pos % 64] |= 1 << (pos % 64);

    macro_rules! check_loop {
        ($dir:expr) => {
            check_loop(
                &rocks_x,
                &rocks_y,
                &rocksx_id,
                &rocksx_len,
                &rocksy_id,
                &rocksy_len,
                rocks_len,
                move_map,
                pos,
                $dir,
            )
        };
    }

    loop {
        let next = pos - 131;
        if next == rock_pos {
            debug_assert_eq!(input[next], b'#');
            break;
        }

        pos = next;

        let seen_elem = seen.get_unchecked_mut(pos / 64);
        let seen_mask = 1 << (pos % 64);
        *seen_elem |= seen_mask;
        if check_loop!(BOT_M) {
            count += 1;
        }
    }

    loop {
        loop {
            let next = pos.wrapping_add(1 as usize);
            if next % 131 == 130 {
                return count;
            }

            if *input.get_unchecked(next) == b'#' {
                break;
            }

            pos = next;

            let seen_elem = seen.get_unchecked_mut(pos / 64);
            let seen_mask = 1 << (pos % 64);
            if *seen_elem & seen_mask == 0 {
                *seen_elem |= seen_mask;
                if check_loop!(LEFT_M) {
                    count += 1;
                }
            }
        }

        loop {
            let next = pos.wrapping_add(131 as usize);
            if next >= 131 * 130 {
                return count;
            }

            if *input.get_unchecked(next) == b'#' {
                break;
            }

            pos = next;

            let seen_elem = seen.get_unchecked_mut(pos / 64);
            let seen_mask = 1 << (pos % 64);
            if *seen_elem & seen_mask == 0 {
                *seen_elem |= seen_mask;
                if check_loop!(TOP_M) {
                    count += 1;
                }
            }
        }

        loop {
            let next = pos.wrapping_add(-1isize as usize);
            if next % 130 == 131 {
                return count;
            }

            if *input.get_unchecked(next) == b'#' {
                break;
            }

            pos = next;

            let seen_elem = seen.get_unchecked_mut(pos / 64);
            let seen_mask = 1 << (pos % 64);
            if *seen_elem & seen_mask == 0 {
                *seen_elem |= seen_mask;
                if check_loop!(RIGHT_M) {
                    count += 1;
                }
            }
        }

        loop {
            let next = pos.wrapping_add(-131isize as usize);
            if next >= 131 * 130 {
                return count;
            }

            if *input.get_unchecked(next) == b'#' {
                break;
            }

            pos = next;

            let seen_elem = seen.get_unchecked_mut(pos / 64);
            let seen_mask = 1 << (pos % 64);
            if *seen_elem & seen_mask == 0 {
                *seen_elem |= seen_mask;
                if check_loop!(BOT_M) {
                    count += 1;
                }
            }
        }
    }

    #[inline(always)]
    unsafe fn check_loop(
        rocks_x: &[u8; 1024],
        rocks_y: &[u8; 1024],
        rocksx_id: &[[u16; 16]; 130],
        rocksx_len: &[usize; 130],
        rocksy_id: &[[u16; 16]; 130],
        rocksy_len: &[usize; 130],
        rocks_len: usize,
        move_map: &mut [u32],
        new_rock_pos: usize,
        dir: usize,
    ) -> bool {
        #[cfg(debug_assertions)]
        let orig = move_map[..rocks_len * 4].to_vec();

        for &mov in &move_map[..(rocks_len + 2) * 4] {
            debug_assert!(mov as (usize) < (rocks_len + 2) * 4);
        }

        let new_rock_idx = rocks_len;
        let out_rock_idx = rocks_len + 1;

        let new_x = (new_rock_pos % 131) as u8;
        let new_y = (new_rock_pos / 131) as u8;

        // Update move_map

        let (mut top_idx, mut top_old) = (usize::MAX, u32::MAX);
        if new_y != 0 {
            let len = *rocksy_len.get_unchecked(new_y as usize - 1);
            let ids = rocksy_id
                .get_unchecked(new_y as usize - 1)
                .get_unchecked(..len);
            let mut idx = ids
                .iter()
                .position(|&id| rocks_x[id as usize] > new_x)
                .unwrap_or(ids.len());
            if idx != 0 {
                idx = idx - 1;
                let mut id = *ids.get_unchecked(idx) as usize;
                *move_map.get_unchecked_mut(mov(new_rock_idx, TOP_M)) = mov(id, RIGHT_M) as u32;
                let prev_move = *move_map.get_unchecked(mov(id, BOT_M));
                if prev_move >> 2 == out_rock_idx as u32
                    || *rocks_x.get_unchecked(prev_move as usize >> 2) > new_x
                {
                    loop {
                        *move_map.get_unchecked_mut(mov(id, BOT_M)) =
                            mov(new_rock_idx, LEFT_M) as u32;
                        if idx == 0 {
                            break;
                        }
                        id = *ids.get_unchecked(idx - 1) as usize;
                        if *move_map.get_unchecked(mov(id, BOT_M)) != prev_move {
                            break;
                        }
                        idx -= 1;
                    }
                    (top_idx, top_old) = (idx, prev_move);
                }
            } else {
                *move_map.get_unchecked_mut(mov(new_rock_idx, TOP_M)) =
                    mov(out_rock_idx, RIGHT_M) as u32;
            }
        }

        let (mut bot_idx, mut bot_old) = (usize::MAX, u32::MAX);
        if new_y != 130 - 1 {
            let len = *rocksy_len.get_unchecked(new_y as usize + 1);
            let ids = rocksy_id
                .get_unchecked(new_y as usize + 1)
                .get_unchecked(..len);
            let mut idx = ids
                .iter()
                .position(|&id| rocks_x[id as usize] > new_x)
                .unwrap_or(ids.len());
            if idx != ids.len() {
                let mut id = *ids.get_unchecked(idx) as usize;
                *move_map.get_unchecked_mut(mov(new_rock_idx, BOT_M)) = mov(id, LEFT_M) as u32;
                let prev_move = *move_map.get_unchecked(mov(id, TOP_M));
                if prev_move >> 2 == out_rock_idx as u32
                    || *rocks_x.get_unchecked(prev_move as usize >> 2) < new_x
                {
                    loop {
                        *move_map.get_unchecked_mut(mov(id, TOP_M)) =
                            mov(new_rock_idx, RIGHT_M) as u32;
                        if idx == ids.len() - 1 {
                            break;
                        }
                        id = *ids.get_unchecked(idx + 1) as usize;
                        if *move_map.get_unchecked(mov(id, TOP_M)) != prev_move {
                            break;
                        }
                        idx += 1;
                    }
                    (bot_idx, bot_old) = (idx, prev_move);
                }
            } else {
                *move_map.get_unchecked_mut(mov(new_rock_idx, BOT_M)) =
                    mov(out_rock_idx, LEFT_M) as u32;
            }
        }

        let (mut left_idx, mut left_old) = (usize::MAX, u32::MAX);
        if new_x != 0 {
            let len = *rocksx_len.get_unchecked(new_x as usize - 1);
            let ids = rocksx_id
                .get_unchecked(new_x as usize - 1)
                .get_unchecked(..len);
            let mut idx = ids
                .iter()
                .position(|&id| rocks_y[id as usize] > new_y)
                .unwrap_or(ids.len());
            if idx != ids.len() {
                let mut id = *ids.get_unchecked(idx) as usize;
                *move_map.get_unchecked_mut(mov(new_rock_idx, LEFT_M)) = mov(id, TOP_M) as u32;
                let prev_move = *move_map.get_unchecked(mov(id, RIGHT_M));
                if prev_move >> 2 == out_rock_idx as u32
                    || *rocks_y.get_unchecked(prev_move as usize >> 2) < new_y
                {
                    loop {
                        *move_map.get_unchecked_mut(mov(id, RIGHT_M)) =
                            mov(new_rock_idx, BOT_M) as u32;
                        if idx == ids.len() - 1 {
                            break;
                        }
                        id = *ids.get_unchecked(idx + 1) as usize;
                        if *move_map.get_unchecked(mov(id, RIGHT_M)) != prev_move {
                            break;
                        }
                        idx += 1;
                    }
                    (left_idx, left_old) = (idx, prev_move);
                }
            } else {
                *move_map.get_unchecked_mut(mov(new_rock_idx, LEFT_M)) =
                    mov(out_rock_idx, TOP_M) as u32;
            }
        }
        let (mut right_idx, mut right_old) = (usize::MAX, u32::MAX);
        if new_x != 130 - 1 {
            let len = *rocksx_len.get_unchecked(new_x as usize + 1);
            let ids = rocksx_id
                .get_unchecked(new_x as usize + 1)
                .get_unchecked(..len);
            let mut idx = ids
                .iter()
                .position(|&id| rocks_y[id as usize] > new_y)
                .unwrap_or(ids.len());
            if idx != 0 {
                idx -= 1;
                let mut id = *ids.get_unchecked(idx) as usize;
                *move_map.get_unchecked_mut(mov(new_rock_idx, RIGHT_M)) = mov(id, BOT_M) as u32;
                let prev_move = *move_map.get_unchecked(mov(id, LEFT_M));
                if prev_move >> 2 == out_rock_idx as u32
                    || *rocks_y.get_unchecked(prev_move as usize >> 2) > new_y
                {
                    loop {
                        *move_map.get_unchecked_mut(mov(id, LEFT_M)) =
                            mov(new_rock_idx, TOP_M) as u32;
                        if idx == 0 {
                            break;
                        }
                        id = *ids.get_unchecked(idx - 1) as usize;
                        if *move_map.get_unchecked(mov(id, LEFT_M)) != prev_move {
                            break;
                        }
                        idx -= 1;
                    }
                    (right_idx, right_old) = (idx, prev_move);
                }
            } else {
                *move_map.get_unchecked_mut(mov(new_rock_idx, RIGHT_M)) =
                    mov(out_rock_idx, BOT_M) as u32;
            }
        }

        if cfg!(debug_assertions) {
            for &mov in &move_map[..(rocks_len + 2) * 4] {
                debug_assert!(mov as (usize) < (rocks_len + 2) * 4);
            }
        }

        let mut pos = (new_rock_idx << 2) | dir;

        debug_assert!(pos < (rocks_len + 2) * 4);

        let mut seen = [0u64; (1024 * 4) / 64];

        let cycle = loop {
            let seen_elem = seen.get_unchecked_mut(pos / 64);
            let seen_mask = 1 << (pos % 64);
            *seen_elem |= seen_mask;
            pos = *move_map.get_unchecked(pos) as usize;

            let seen_elem = seen.get_unchecked_mut(pos / 64);
            let seen_mask = 1 << (pos % 64);
            *seen_elem |= seen_mask;
            pos = *move_map.get_unchecked(pos) as usize;

            let seen_elem = seen.get_unchecked_mut(pos / 64);
            let seen_mask = 1 << (pos % 64);
            *seen_elem |= seen_mask;
            pos = *move_map.get_unchecked(pos) as usize;

            let seen_elem = seen.get_unchecked_mut(pos / 64);
            let seen_mask = 1 << (pos % 64);
            if *seen_elem & seen_mask != 0 {
                break pos >> 2 != out_rock_idx;
            }
            *seen_elem |= seen_mask;
            pos = *move_map.get_unchecked(pos) as usize;
        };

        // Reset move_map
        if top_idx != usize::MAX {
            let len = *rocksy_len.get_unchecked(new_y as usize - 1);
            let ids = rocksy_id
                .get_unchecked(new_y as usize - 1)
                .get_unchecked(..len);
            while top_idx < len
                && *move_map.get_unchecked(mov(*ids.get_unchecked(top_idx) as usize, BOT_M))
                    == mov(new_rock_idx, LEFT_M) as u32
            {
                *move_map.get_unchecked_mut(mov(*ids.get_unchecked(top_idx) as usize, BOT_M)) =
                    top_old;
                top_idx += 1;
            }
        }
        if bot_idx != usize::MAX {
            let len = *rocksy_len.get_unchecked(new_y as usize + 1);
            let ids = rocksy_id
                .get_unchecked(new_y as usize + 1)
                .get_unchecked(..len);
            while bot_idx < len
                && *move_map.get_unchecked(mov(*ids.get_unchecked(bot_idx) as usize, TOP_M))
                    == mov(new_rock_idx, RIGHT_M) as u32
            {
                *move_map.get_unchecked_mut(mov(*ids.get_unchecked(bot_idx) as usize, TOP_M)) =
                    bot_old;
                bot_idx = bot_idx.wrapping_sub(1);
            }
        }
        if left_idx != usize::MAX {
            let len = *rocksx_len.get_unchecked(new_x as usize - 1);
            let ids = rocksx_id
                .get_unchecked(new_x as usize - 1)
                .get_unchecked(..len);
            while left_idx < len
                && *move_map.get_unchecked(mov(*ids.get_unchecked(left_idx) as usize, RIGHT_M))
                    == mov(new_rock_idx, BOT_M) as u32
            {
                *move_map.get_unchecked_mut(mov(*ids.get_unchecked(left_idx) as usize, RIGHT_M)) =
                    left_old;
                left_idx = left_idx.wrapping_sub(1);
            }
        }
        if right_idx != usize::MAX {
            let len = *rocksx_len.get_unchecked(new_x as usize + 1);
            let ids = rocksx_id
                .get_unchecked(new_x as usize + 1)
                .get_unchecked(..len);
            while right_idx < len
                && *move_map.get_unchecked(mov(*ids.get_unchecked(right_idx) as usize, LEFT_M))
                    == mov(new_rock_idx, TOP_M) as u32
            {
                *move_map.get_unchecked_mut(mov(*ids.get_unchecked(right_idx) as usize, LEFT_M)) =
                    right_old;
                right_idx += 1;
            }
        }

        if cfg!(debug_assertions) {
            for &mov in &move_map[..(rocks_len + 2) * 4] {
                debug_assert!(mov as (usize) < (rocks_len + 2) * 4);
            }
        }

        #[cfg(debug_assertions)]
        {
            for i in 0..rocks_len {
                debug_assert_eq!(move_map[(i << 2) | TOP_M], orig[(i << 2) | TOP_M]);
                debug_assert_eq!(move_map[(i << 2) | BOT_M], orig[(i << 2) | BOT_M]);
                debug_assert_eq!(move_map[(i << 2) | LEFT_M], orig[(i << 2) | LEFT_M]);
                debug_assert_eq!(move_map[(i << 2) | RIGHT_M], orig[(i << 2) | RIGHT_M]);
            }

            debug_assert_eq!(&move_map[..rocks_len * 4], orig);
        }

        cycle
    }
}
