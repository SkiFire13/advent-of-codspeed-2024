#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

// pub fn run(input: &str) -> i64 {
//     part1(input) as i64
// }

use std::mem::MaybeUninit;

pub fn run(input: &str) -> &'static str {
    part2(input)
}

#[inline(always)]
pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> &'static str {
    unsafe { inner_part2(input) }
}

static mut PART2_OUTPUT: [u8; 2 * 9] = [b','; 2 * 9];

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    let mut grid = const {
        let mut grid = [0u64; (73 * 73 + 63) / 64];
        let mut i = 0;
        while i < 73 {
            let a = [(0, i), (i, 0), (72, i), (i, 72)];
            let mut j = 0;
            while j < 4 {
                let (x, y) = a[j];
                let idx = 73 * y + x;
                grid[idx / 64] |= 1 << (idx % 64);
                j += 1;
            }

            i += 1;
        }
        grid
    };

    let mut ptr = input.as_ptr();
    let mut n = 0;

    loop {
        let mut x = (*ptr as usize) - (b'0' as usize);
        ptr = ptr.add(1);
        if *ptr != b',' {
            x = 10 * x + (*ptr as usize) - (b'0' as usize);
            ptr = ptr.add(1);
        }

        ptr = ptr.add(1);

        let mut y = (*ptr as usize) - (b'0' as usize);
        ptr = ptr.add(1);
        if *ptr != b'\n' {
            y = 10 * y + (*ptr as usize) - (b'0' as usize);
            ptr = ptr.add(1);
        }
        ptr = ptr.add(1);

        let idx = 73 * (y + 1) + (x + 1);
        *grid.get_unchecked_mut(idx / 64) |= 1 << (idx % 64);

        n += 1;
        if n == 1024 {
            break;
        }
    }

    let mut queue1 = [MaybeUninit::uninit(); 256];
    let mut queue2 = [MaybeUninit::uninit(); 256];
    let mut queue1_len = 0;
    let mut queue2_len = 0;

    const START: usize = 73 * (0 + 1) + (0 + 1);
    const END: usize = 73 * (70 + 1) + (70 + 1);

    *queue1[queue1_len].as_mut_ptr() = START;
    queue1_len += 1;
    *grid.get_unchecked_mut(START / 64) |= 1 << (START % 64);

    let mut cost = 0;

    loop {
        macro_rules! exhaust_queue {
            ($queue1:ident, $queue1_len:ident, $queue2:ident, $queue2_len:ident) => {
                loop {
                    $queue1_len -= 1;
                    let pos = *$queue1.get_unchecked($queue1_len).as_ptr();

                    if pos == END {
                        return cost;
                    }

                    const LEFT: usize = -1isize as usize;
                    const RIGHT: usize = 1;
                    const UP: usize = -73isize as usize;
                    const DOWN: usize = 73;
                    for dir in [LEFT, RIGHT, UP, DOWN] {
                        let new_pos = pos.wrapping_add(dir);
                        let g = grid.get_unchecked_mut(new_pos / 64);
                        if *g & (1 << (new_pos % 64)) == 0 {
                            *g |= 1 << (new_pos % 64);
                            *$queue2.get_unchecked_mut($queue2_len).as_mut_ptr() = new_pos;
                            $queue2_len += 1;
                        }
                    }

                    if $queue1_len == 0 {
                        break;
                    }
                }
                cost += 1;
            };
        }

        exhaust_queue!(queue1, queue1_len, queue2, queue2_len);
        exhaust_queue!(queue2, queue2_len, queue1, queue1_len);
    }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> &'static str {
    let input = input.as_bytes();

    const LEFT_ROOT: usize = 73 * 1 + 0;
    const RIGHT_ROOT: usize = 73 * 0 + 1;
    const PARENT_MASK: u16 = 1 << 15;

    let mut uf = const {
        let mut uf = [0u16; 73 * 73];

        uf[LEFT_ROOT] = u16::MAX >> 2;
        uf[RIGHT_ROOT] = u16::MAX >> 3;

        let mut i = 2;
        while i < 73 {
            uf[73 * i + 0] = LEFT_ROOT as u16 | PARENT_MASK;
            uf[73 * 0 + i] = RIGHT_ROOT as u16 | PARENT_MASK;
            i += 1;
        }
        let mut i = 1;
        while i < 72 {
            uf[73 * 72 + i] = LEFT_ROOT as u16 | PARENT_MASK;
            uf[73 * i + 72] = RIGHT_ROOT as u16 | PARENT_MASK;
            i += 1;
        }

        uf
    };

    let mut ptr = input.as_ptr();

    let (x, y) = 'outer: loop {
        let mut x = (*ptr as usize) - (b'0' as usize);
        ptr = ptr.add(1);
        if *ptr != b',' {
            x = 10 * x + (*ptr as usize) - (b'0' as usize);
            ptr = ptr.add(1);
        }

        ptr = ptr.add(1);

        let mut y = (*ptr as usize) - (b'0' as usize);
        ptr = ptr.add(1);
        if *ptr != b'\n' {
            y = 10 * y + (*ptr as usize) - (b'0' as usize);
            ptr = ptr.add(1);
        }
        ptr = ptr.add(1);

        let pos = 73 * (y + 1) + (x + 1);
        let mut this_root = pos as u16;
        *uf.get_unchecked_mut(this_root as usize) = 1;
        for dir in [-73isize - 1, -73, -73 + 1, -1, 1, 73 - 1, 73, 73 + 1] {
            let new_pos = pos.wrapping_add(dir as usize);
            if *uf.get_unchecked(new_pos) != 0 {
                let mut root = new_pos as u16;
                while *uf.get_unchecked(root as usize) & PARENT_MASK != 0 {
                    root = *uf.get_unchecked(root as usize) & !PARENT_MASK;
                }
                if root != this_root {
                    if *uf.get_unchecked(root as usize) < *uf.get_unchecked(this_root as usize) {
                        *uf.get_unchecked_mut(this_root as usize) +=
                            *uf.get_unchecked(root as usize);
                        *uf.get_unchecked_mut(root as usize) = this_root | PARENT_MASK;
                    } else {
                        *uf.get_unchecked_mut(root as usize) +=
                            *uf.get_unchecked(this_root as usize);
                        *uf.get_unchecked_mut(this_root as usize) = root | PARENT_MASK;
                        this_root = root;
                    }
                    if *uf.get_unchecked(RIGHT_ROOT) == LEFT_ROOT as u16 | PARENT_MASK {
                        break 'outer (x, y);
                    }
                }
            }
        }
    };

    let mut out_len = 0;

    if x >= 10 {
        *PART2_OUTPUT.get_unchecked_mut(out_len) = b'0' + (x / 10) as u8;
        out_len += 1;
        *PART2_OUTPUT.get_unchecked_mut(out_len) = b'0' + (x % 10) as u8;
        out_len += 1;
    } else {
        *PART2_OUTPUT.get_unchecked_mut(out_len) = b'0' + x as u8;
        out_len += 1;
    }

    *PART2_OUTPUT.get_unchecked_mut(out_len) = b',';
    out_len += 1;

    if y >= 10 {
        *PART2_OUTPUT.get_unchecked_mut(out_len) = b'0' + (y / 10) as u8;
        out_len += 1;
        *PART2_OUTPUT.get_unchecked_mut(out_len) = b'0' + (y % 10) as u8;
        out_len += 1;
    } else {
        *PART2_OUTPUT.get_unchecked_mut(out_len) = b'0' + y as u8;
        out_len += 1;
    }

    std::str::from_utf8_unchecked(PART2_OUTPUT.get_unchecked(..out_len))
}
