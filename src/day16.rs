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

const LEFT_ID: DirId = DirId(0b00);
const RIGHT_ID: DirId = DirId(0b11);
const UP_ID: DirId = DirId(0b10);
const DOWN_ID: DirId = DirId(0b01);

static DIR_MAP: [usize; 4] = {
    let mut dirs = [0; 4];
    dirs[LEFT_ID.idx()] = LEFT;
    dirs[RIGHT_ID.idx()] = RIGHT;
    dirs[UP_ID.idx()] = UP;
    dirs[DOWN_ID.idx()] = DOWN;
    dirs
};

#[derive(Copy, Clone)]
#[repr(transparent)]
struct DirId(u16);

impl DirId {
    const fn parity(self) -> usize {
        (self.0 & 1) as usize
    }
    const fn kind(self) -> u16 {
        (self.0 ^ (self.0 >> 1)) & 1
    }
    const fn invert(self) -> DirId {
        Self(self.0 ^ 0b11)
    }
    const fn perp1(self) -> DirId {
        Self(self.0 ^ 0b01)
    }
    const fn perp2(self) -> DirId {
        Self(self.0 ^ 0b10)
    }
    const fn idx(self) -> usize {
        self.0 as usize
    }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u32 {
    let input = input.as_bytes();

    let mut ids = [u16::MAX; 141 * 142];
    ids[START as usize] = 0;
    ids[END as usize] = 2;
    let mut next_id = 4;

    let mut moves = [MaybeUninit::<[(u16, u8, u8); 2]>::uninit(); 3000];
    moves[0] = MaybeUninit::new([(u16::MAX, 0, 0); 2]);
    moves[1] = MaybeUninit::new([(u16::MAX, 0, 0); 2]);
    moves[2] = MaybeUninit::new([(u16::MAX, 0, 0); 2]);
    moves[3] = MaybeUninit::new([(u16::MAX, 0, 0); 2]);

    let mut queue = [MaybeUninit::uninit(); 256];
    *queue.get_unchecked_mut(0).as_mut_ptr() = (START, RIGHT_ID, 0);
    *queue.get_unchecked_mut(1).as_mut_ptr() = (START, UP_ID, 1);
    let mut queue_len = 2;

    'queue: while queue_len != 0 {
        queue_len -= 1;
        let (pos, start_dir_id, start_id) = *queue.get_unchecked(queue_len).as_ptr();

        if moves[start_id as usize].assume_init()[start_dir_id.parity() as usize].0 != u16::MAX {
            continue;
        }

        let mut pos = pos as usize;
        let mut dir_id = start_dir_id;
        let mut turns = 0;
        let mut cells = 0;

        let mut dir = *DIR_MAP.get_unchecked(dir_id.idx());
        let mut dir1 = *DIR_MAP.get_unchecked(dir_id.perp1().idx());
        let mut dir2 = *DIR_MAP.get_unchecked(dir_id.perp2().idx());

        debug_assert_ne!(
            input[pos.wrapping_add(dir)] as char,
            '#',
            "{} {}, {} {}, {}",
            pos % 142,
            pos / 142,
            pos.wrapping_add(dir) % 142,
            pos.wrapping_add(dir) / 142,
            dir as isize
        );

        'inner: loop {
            pos = pos.wrapping_add(dir);
            cells += 1;

            let cont = *input.get_unchecked(pos.wrapping_add(dir)) != b'#';
            let cont1 = *input.get_unchecked(pos.wrapping_add(dir1)) != b'#';
            let cont2 = *input.get_unchecked(pos.wrapping_add(dir2)) != b'#';

            debug_assert_ne!(input[pos] as char, '#', "{} {}", pos % 142, pos / 142);

            if !cont1 && !cont2 {
                if cont {
                    // go straight
                    continue 'inner;
                } else {
                    // deadend
                    continue 'queue;
                }
            } else if cont || (cont1 && cont2) {
                // new node

                let mut dest_id = *ids.get_unchecked(pos);
                if dest_id == u16::MAX {
                    dest_id = next_id | dir_id.kind();

                    *ids.get_unchecked_mut(pos) = next_id;
                    *moves.get_unchecked_mut(next_id as usize).as_mut_ptr() = [(u16::MAX, 0, 0); 2];
                    *moves.get_unchecked_mut(next_id as usize + 1).as_mut_ptr() =
                        [(u16::MAX, 0, 0); 2];

                    debug_assert!(dest_id == next_id || dest_id == next_id + 1);

                    next_id += 2;

                    let m = &*moves.get_unchecked(dest_id as usize).as_ptr();
                    if cont {
                        debug_assert_eq!(m.get_unchecked(dir_id.invert().parity()).0, u16::MAX);
                        *queue.get_unchecked_mut(queue_len).as_mut_ptr() =
                            (pos as u32, dir_id, dest_id);
                        queue_len += 1;
                    }

                    let m = &*moves.get_unchecked(dest_id as usize ^ 1).as_ptr();

                    let dir1_id = dir_id.perp1();
                    if cont1 && m.get_unchecked(dir1_id.parity()).0 == u16::MAX {
                        *queue.get_unchecked_mut(queue_len).as_mut_ptr() =
                            (pos as u32, dir1_id, dest_id ^ 1);
                        queue_len += 1;
                    }

                    let dir2_id = dir_id.perp2();
                    if cont2 && m.get_unchecked(dir2_id.parity()).0 == u16::MAX {
                        *queue.get_unchecked_mut(queue_len).as_mut_ptr() =
                            (pos as u32, dir2_id, dest_id ^ 1);
                        queue_len += 1;
                    }
                }

                *(*moves.get_unchecked_mut(start_id as usize).as_mut_ptr())
                    .get_unchecked_mut(start_dir_id.parity()) = (dest_id, turns, cells);
                *(*moves.get_unchecked_mut(dest_id as usize).as_mut_ptr())
                    .get_unchecked_mut(dir_id.invert().parity()) = (start_id, turns, cells);

                continue 'queue;
            } else {
                // turn

                dir_id = if cont1 {
                    dir_id.perp1()
                } else {
                    dir_id.perp2()
                };
                dir = *DIR_MAP.get_unchecked(dir_id.idx());
                dir1 = *DIR_MAP.get_unchecked(dir_id.perp1().idx());
                dir2 = *DIR_MAP.get_unchecked(dir_id.perp2().idx());
                turns += 1;

                continue 'inner;
            }
        }
    }

    0
}
