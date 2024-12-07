#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]

use std::mem::MaybeUninit;
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
    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(transparent)]
    struct RockYX(u16);
    impl RockYX {
        #[inline(always)]
        const fn new(x: u8, y: u8) -> Self {
            Self(u16::from_le_bytes([x, y]))
        }
        #[inline(always)]
        const fn x(&self) -> u8 {
            self.0.to_le_bytes()[0]
        }
        #[inline(always)]
        const fn y(&self) -> u8 {
            self.0.to_le_bytes()[1]
        }
    }

    #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    #[repr(transparent)]
    struct RockXY(u32);
    impl RockXY {
        #[inline(always)]
        const fn new(x: u8, y: u8, idx: usize) -> Self {
            Self(((x as u32) << 24) | ((y as u32) << 16) | (idx as u32))
        }
        #[inline(always)]
        const fn x(&self) -> u8 {
            (self.0 >> 24) as u8
        }
        #[inline(always)]
        const fn y(&self) -> u8 {
            (self.0 >> 16) as u8
        }
        #[inline(always)]
        const fn idx(&self) -> usize {
            self.0 as u16 as usize
        }
    }

    let input = input.as_bytes();

    let mut start = 0;
    let mut rocks = [const { RockYX::new(0, 0) }; 1024];
    let mut rocks_len = 0;

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
            rocks[rocks_len] = RockYX::new(x as _, y as _);
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
            rocks[rocks_len] = RockYX::new(x as _, y as _);
            rocks_len += 1;
        }

        offset += 32;
    }
    while offset < input.len() {
        if *input.get_unchecked(offset) == b'#' {
            let pos = offset;
            let x = pos % 131;
            let y = pos / 131;
            rocks[rocks_len] = RockYX::new(x as _, y as _);
            rocks_len += 1;
        }

        offset += 1;
    }

    let rocks = &mut rocks[..rocks_len];

    // Build a (x, y)-sorted list of rocks, the other one is already (y,x)-sorted
    let mut counts = [0; 131];
    let mut rocksx = MaybeUninit::<[RockXY; 1024]>::uninit();
    let rocksx =
        std::ptr::slice_from_raw_parts_mut(rocksx.as_mut_ptr().cast::<RockXY>(), rocks_len);
    for &r in &*rocks {
        *counts.get_unchecked_mut(r.x() as usize) += 1;
    }
    for i in 0..counts.len() - 1 {
        *counts.get_unchecked_mut(i + 1) += *counts.get_unchecked(i);
    }
    for (i, &r) in rocks.iter().enumerate().rev() {
        let rock = RockXY::new(r.x(), r.y(), i);
        let idx = counts.get_unchecked_mut(r.x() as usize);
        *idx -= 1;
        *rocksx.get_unchecked_mut(*idx) = rock;
    }
    let rocksx = &mut *rocksx;

    // A rock representing going out of bounds
    let out_rock_idx = rocks_len + 1;

    // Map of moves.
    // Each move is encoded as a rock idx left-shifted by 2, plus the
    // position relative to the rock as the last 2 bits.
    const BOT_M: usize = 0b00;
    const TOP_M: usize = 0b01;
    const LEFT_M: usize = 0b10;
    const RIGHT_M: usize = 0b11;
    let mut move_map = [out_rock_idx << 2; 1024 * 4];
    let move_map = &mut move_map[..(out_rock_idx + 1) << 2];

    // Build next rock map for vertical-to-horizontal turns
    let mut line1 = 0;
    let mut line2 = rocks.partition_point(|&r| r < RockYX::new(0, 1));
    for y in 0..130 - 1 {
        let mut curr1 = out_rock_idx;
        let mut next1 = line1;
        let mut next2 = line2;

        loop {
            if next2 >= rocks.len() || rocks.get_unchecked(next2).y() != y + 1 {
                // Line 2 has no rocks left
                while rocks.get_unchecked(next1).y() == y {
                    *move_map.get_unchecked_mut((next1 << 2) | BOT_M) =
                        (out_rock_idx << 2) | LEFT_M;
                    next1 += 1;
                }
                line1 = next1;
                line2 = next2;
                break;
            }

            while rocks.get_unchecked(next1).y() == y
                && rocks.get_unchecked(next1).x() <= rocks.get_unchecked(next2).x()
            {
                *move_map.get_unchecked_mut((next1 << 2) | BOT_M) = (next2 << 2) | LEFT_M;
                curr1 = next1;
                next1 += 1;
            }

            if rocks.get_unchecked(next1).y() != y {
                // Line 1 has no rocks left
                while next2 < rocks.len() && rocks.get_unchecked(next2).y() == y + 1 {
                    *move_map.get_unchecked_mut((next2 << 2) | TOP_M) = (curr1 << 2) | RIGHT_M;
                    next2 += 1;
                }
                line1 = next1;
                line2 = next2;
                break;
            }

            while next2 < rocks.len()
                && rocks.get_unchecked(next2).y() == y + 1
                && rocks.get_unchecked(next2).x() <= rocks.get_unchecked(next1).x()
            {
                *move_map.get_unchecked_mut((next2 << 2) | TOP_M) = (curr1 << 2) | RIGHT_M;
                next2 += 1;
            }
        }
    }

    // Build next rock map for horizontal-to-vertical turns
    let mut line1 = 0;
    let mut line2 = rocksx.partition_point(|&r| r < RockXY::new(1, 0, 0));
    for x in 0..130 - 1 {
        let mut next1 = line1;
        let mut curr2i = out_rock_idx;
        let mut next2 = line2;

        loop {
            if next2 >= rocksx.len() || rocksx.get_unchecked(next2).x() != x + 1 {
                // Line 2 has no rocks left
                while rocksx.get_unchecked(next1).x() == x {
                    let next1i = rocksx.get_unchecked(next1).idx();
                    *move_map.get_unchecked_mut((next1i << 2) | RIGHT_M) = (curr2i << 2) | BOT_M;
                    next1 += 1;
                }
                line1 = next1;
                line2 = next2;
                break;
            }

            while rocksx.get_unchecked(next1).x() == x
                && rocksx.get_unchecked(next1).y() <= rocksx.get_unchecked(next2).y()
            {
                let next1i = rocksx.get_unchecked(next1).idx();
                *move_map.get_unchecked_mut((next1i << 2) | RIGHT_M) = (curr2i << 2) | BOT_M;
                next1 += 1;
            }

            if rocksx.get_unchecked(next1).x() != x {
                // Line 1 has no rocks left
                while next2 < rocksx.len() && rocksx.get_unchecked(next2).x() == x + 1 {
                    let next2i = rocksx.get_unchecked(next2).idx();
                    *move_map.get_unchecked_mut((next2i << 2) | LEFT_M) =
                        (out_rock_idx << 2) | TOP_M;
                    next2 += 1;
                }
                line1 = next1;
                line2 = next2;
                break;
            }

            while next2 < rocksx.len()
                && rocksx.get_unchecked(next2).x() == x + 1
                && rocksx.get_unchecked(next2).y() <= rocksx.get_unchecked(next1).y()
            {
                let next1i = rocksx.get_unchecked(next1).idx();
                let next2i = rocksx.get_unchecked(next2).idx();
                *move_map.get_unchecked_mut((next2i << 2) | LEFT_M) = (next1i << 2) | TOP_M;
                curr2i = rocksx.get_unchecked(next2).idx();
                next2 += 1;
            }
        }
    }

    let startx = start % 131;
    let starty = start / 131;

    let rock =
        rocksx[rocksx.partition_point(|&r| r < RockXY::new(startx as _, starty as _, 0)) - 1].idx();

    let mut count = 0;
    let mut pos = start as usize;
    let rock_pos = (rocks[rock].y()) as usize * 131 + rocks[rock].x() as usize;
    let mut seen = [0u64; (130 * 131 + 63) / 64];
    seen[pos % 64] |= 1 << (pos % 64);

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
        if check_loop::<true>(rocks, rocksx, move_map, rock, pos) {
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
                if check_loop::<false>(rocks, rocksx, move_map, rock, pos) {
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
                if check_loop::<false>(rocks, rocksx, move_map, rock, pos) {
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
                if check_loop::<false>(rocks, rocksx, move_map, rock, pos) {
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
                if check_loop::<false>(rocks, rocksx, move_map, rock, pos) {
                    count += 1;
                }
            }
        }
    }

    #[inline(always)]
    unsafe fn check_loop<const FIRST: bool>(
        rocks: &[RockYX],
        rocksx: &[RockXY],
        move_map: &mut [usize],
        rock: usize,
        new_rock_pos: usize,
    ) -> bool {
        let orig = move_map[..rocks.len() * 4].to_vec();

        for &mov in &move_map[..(rocks.len() + 2) * 4] {
            debug_assert!(mov < (rocks.len() + 2) * 4);
        }

        let new_rock_idx = rocks.len();
        let out_rock_idx = rocks.len() + 1;

        let new_x = (new_rock_pos % 131) as u8;
        let new_y = (new_rock_pos / 131) as u8;

        // Update move_map

        let (mut top_idx, mut top_old) = (usize::MAX, usize::MAX);
        if new_y != 0 {
            let mut idx = rocks.partition_point(|&r| r <= RockYX::new(new_x, new_y - 1));
            if idx != 0 && rocks.get_unchecked(idx - 1).y() == new_y - 1 {
                idx = idx - 1;
                *move_map.get_unchecked_mut((new_rock_idx << 2) | TOP_M) = (idx << 2) | RIGHT_M;
                let prev_move = *move_map.get_unchecked((idx << 2) | BOT_M);
                top_old = prev_move;
                if prev_move >> 2 == out_rock_idx || rocks.get_unchecked(prev_move >> 2).x() > new_x
                {
                    loop {
                        *move_map.get_unchecked_mut((idx << 2) | BOT_M) =
                            (new_rock_idx << 2) | LEFT_M;
                        if idx == 0
                            || rocks.get_unchecked(idx - 1).y() != new_y - 1
                            || *move_map.get_unchecked(((idx - 1) << 2) | BOT_M) != prev_move
                        {
                            break;
                        }
                        idx -= 1;
                    }
                    top_idx = idx;
                }
            } else {
                *move_map.get_unchecked_mut((new_rock_idx << 2) | TOP_M) =
                    (out_rock_idx << 2) | RIGHT_M;
            }
        }
        let (mut bot_idx, mut bot_old) = (usize::MAX, usize::MAX);
        if new_y != 130 - 1 {
            let mut idx = rocks.partition_point(|&r| r <= RockYX::new(new_x, new_y + 1));
            if idx != rocks.len() && rocks.get_unchecked(idx).y() == new_y + 1 {
                *move_map.get_unchecked_mut((new_rock_idx << 2) | BOT_M) = (idx << 2) | LEFT_M;
                let prev_move = *move_map.get_unchecked((idx << 2) | TOP_M);
                bot_old = prev_move;
                if prev_move >> 2 == out_rock_idx || rocks.get_unchecked(prev_move >> 2).x() < new_x
                {
                    loop {
                        *move_map.get_unchecked_mut((idx << 2) | TOP_M) =
                            (new_rock_idx << 2) | RIGHT_M;
                        if idx == rocks.len() - 1
                            || rocks.get_unchecked(idx + 1).y() != new_y + 1
                            || *move_map.get_unchecked(((idx + 1) << 2) | TOP_M) != prev_move
                        {
                            break;
                        }
                        idx += 1;
                    }
                    bot_idx = idx;
                }
            } else {
                *move_map.get_unchecked_mut((new_rock_idx << 2) | BOT_M) =
                    (out_rock_idx << 2) | LEFT_M;
            }
        }
        let (mut left_idx, mut left_old) = (usize::MAX, usize::MAX);
        if new_x != 0 {
            let mut idx = rocksx.partition_point(|&r| r <= RockXY::new(new_x - 1, new_y, 0));
            if idx != rocks.len() && rocksx.get_unchecked(idx).x() == new_x - 1 {
                let mut ridx = rocksx.get_unchecked(idx).idx();
                *move_map.get_unchecked_mut((new_rock_idx << 2) | LEFT_M) = (ridx << 2) | TOP_M;
                let prev_move = *move_map.get_unchecked((ridx << 2) | RIGHT_M);
                left_old = prev_move;
                if prev_move >> 2 == out_rock_idx || rocks.get_unchecked(prev_move >> 2).y() < new_y
                {
                    loop {
                        *move_map.get_unchecked_mut((ridx << 2) | RIGHT_M) =
                            (new_rock_idx << 2) | BOT_M;
                        if idx == rocks.len() - 1
                            || rocksx.get_unchecked(idx + 1).x() != new_x - 1
                            || *move_map
                                .get_unchecked((rocksx.get_unchecked(idx + 1).idx() << 2) | RIGHT_M)
                                != prev_move
                        {
                            break;
                        }
                        idx += 1;
                        ridx = rocksx.get_unchecked(idx).idx();
                    }
                    left_idx = idx;
                }
            } else {
                *move_map.get_unchecked_mut((new_rock_idx << 2) | LEFT_M) =
                    (out_rock_idx << 2) | TOP_M;
            }
        }
        let (mut right_idx, mut right_old) = (usize::MAX, usize::MAX);
        if new_x != 130 - 1 {
            let mut idx = rocksx.partition_point(|&r| r <= RockXY::new(new_x + 1, new_y, 0));
            if idx != 0 && rocksx.get_unchecked(idx - 1).x() == new_x + 1 {
                idx -= 1;
                let mut ridx = rocksx.get_unchecked(idx).idx();
                *move_map.get_unchecked_mut((new_rock_idx << 2) | RIGHT_M) = (ridx << 2) | BOT_M;
                let prev_move = *move_map.get_unchecked((ridx << 2) | LEFT_M);
                right_old = prev_move;
                if prev_move >> 2 == out_rock_idx || rocks.get_unchecked(prev_move >> 2).y() > new_y
                {
                    loop {
                        *move_map.get_unchecked_mut((ridx << 2) | LEFT_M) =
                            (new_rock_idx << 2) | TOP_M;
                        if idx == 0
                            || rocksx.get_unchecked(idx - 1).x() != new_x + 1
                            || *move_map
                                .get_unchecked((rocksx.get_unchecked(idx - 1).idx() << 2) | LEFT_M)
                                != prev_move
                        {
                            break;
                        }
                        idx -= 1;
                        ridx = rocksx.get_unchecked(idx).idx();
                    }
                    right_idx = idx;
                }
            } else {
                *move_map.get_unchecked_mut((new_rock_idx << 2) | RIGHT_M) =
                    (out_rock_idx << 2) | BOT_M;
            }
        }

        for &mov in &move_map[..(rocks.len() + 2) * 4] {
            debug_assert!(mov < (rocks.len() + 2) * 4);
        }

        let start_rock = if FIRST { new_rock_idx } else { rock };
        let mut pos = (start_rock << 2) | BOT_M;

        debug_assert!(pos < (rocks.len() + 2) * 4);

        let mut seen = [0u64; (1024 * 4) / 64];

        let cycle = loop {
            let seen_elem = seen.get_unchecked_mut(pos / 64);
            let seen_mask = 1 << (pos % 64);
            if *seen_elem & seen_mask != 0 {
                break pos >> 2 != out_rock_idx;
            }
            *seen_elem |= seen_mask;
            pos = *move_map.get_unchecked(pos);
        };

        // Reset move_map
        if top_idx != usize::MAX {
            while top_idx < rocks.len()
                && *move_map.get_unchecked((top_idx << 2) | BOT_M) == (new_rock_idx << 2) | LEFT_M
            {
                *move_map.get_unchecked_mut((top_idx << 2) | BOT_M) = top_old;
                top_idx += 1;
            }
        }
        if bot_idx != usize::MAX {
            while bot_idx < rocks.len()
                && *move_map.get_unchecked((bot_idx << 2) | TOP_M) == (new_rock_idx << 2) | RIGHT_M
            {
                *move_map.get_unchecked_mut((bot_idx << 2) | TOP_M) = bot_old;
                bot_idx = bot_idx.wrapping_sub(1);
            }
        }
        if left_idx != usize::MAX {
            while left_idx < rocks.len()
                && *move_map.get_unchecked((rocksx.get_unchecked(left_idx).idx() << 2) | RIGHT_M)
                    == (new_rock_idx << 2) | BOT_M
            {
                *move_map
                    .get_unchecked_mut((rocksx.get_unchecked(left_idx).idx() << 2) | RIGHT_M) =
                    left_old;
                left_idx = left_idx.wrapping_sub(1);
            }
        }
        if right_idx != usize::MAX {
            while right_idx < rocks.len()
                && *move_map.get_unchecked((rocksx.get_unchecked(right_idx).idx() << 2) | LEFT_M)
                    == (new_rock_idx << 2) | TOP_M
            {
                *move_map
                    .get_unchecked_mut((rocksx.get_unchecked(right_idx).idx() << 2) | LEFT_M) =
                    right_old;
                right_idx += 1;
            }
        }

        for &mov in &move_map[..(rocks.len() + 2) * 4] {
            debug_assert!(mov < (rocks.len() + 2) * 4);
        }

        debug_assert_eq!(&move_map[..rocks.len() * 4], orig);

        cycle
    }
}
