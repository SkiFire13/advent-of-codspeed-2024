#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

use std::mem::MaybeUninit;

// pub fn run(input: &str) -> i64 {
//     part1(input) as i64
// }

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

    const MAX: usize = 3450;
    let mut levels_list = [MaybeUninit::uninit(); MAX];
    let mut levels = const {
        let mut levels = [0; 73 * 73];
        let mut i = 0;
        while i < 73 {
            let a = [(0, i), (i, 0), (72, i), (i, 72)];
            let mut j = 0;
            while j < 4 {
                let (x, y) = a[j];
                levels[73 * y + x] = u16::MAX;
                j += 1;
            }
            i += 1;
        }
        levels
    };

    let mut ptr = input.as_ptr();
    let end_ptr = ptr.add(input.len());
    let mut n = MAX as u16;

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

        let pos = 73 * (y + 1) + (x + 1);
        *levels.get_unchecked_mut(pos) = n;
        *levels_list.get_unchecked_mut(n as usize - 1).as_mut_ptr() = pos as u16;

        n -= 1;

        if ptr == end_ptr {
            debug_assert_eq!(n, 0);
            break;
        }
    }

    const START: usize = 73 * (0 + 1) + (0 + 1);
    const END: usize = 73 * (70 + 1) + (70 + 1);

    let mut stack = [MaybeUninit::<u16>::uninit(); 32];
    let mut stack_len = 0;

    *stack[stack_len].as_mut_ptr() = START as u16;
    stack_len += 1;
    *levels.get_unchecked_mut(START) = u16::MAX;

    let mut max = 0;

    loop {
        stack_len -= 1;
        let pos = *stack.get_unchecked(stack_len).as_ptr() as usize;

        debug_assert!(*levels.get_unchecked(pos) == u16::MAX);

        if pos == END {
            break;
        }

        const LEFT: usize = -1isize as usize;
        const RIGHT: usize = 1;
        const UP: usize = -73isize as usize;
        const DOWN: usize = 73;
        for dir in [LEFT, RIGHT, UP, DOWN] {
            let new_pos = pos.wrapping_add(dir);

            let level = levels.get_unchecked_mut(new_pos);
            if *level <= max {
                *stack.get_unchecked_mut(stack_len).as_mut_ptr() = new_pos as u16;
                stack_len += 1;
            }
            *level = u16::MAX;
        }

        if stack_len == 0 {
            loop {
                max += 1;
                let pos = *levels_list.get_unchecked(max as usize - 1).as_ptr() as usize;
                if *levels.get_unchecked(pos) == u16::MAX {
                    *stack[0].as_mut_ptr() = pos as u16;
                    stack_len = 1;
                    break;
                }
            }
        }
    }

    let pos = *levels_list.get_unchecked(max as usize - 1).as_ptr();
    let (x, y) = ((pos % 73) - 1, (pos / 73) - 1);

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
