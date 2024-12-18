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

    const MAX: usize = 3450;
    let mut levels_list = [MaybeUninit::uninit(); MAX];
    let mut levels = [MAX as u16; 73 * 73];

    let mut ptr = input.as_ptr();
    let end_ptr = ptr.add(input.len());
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

        let pos = 73 * (y + 1) + (x + 1);
        *levels.get_unchecked_mut(pos) = n;
        *levels_list.get_unchecked_mut(n as usize).as_mut_ptr() = pos as u16;

        n += 1;
        if ptr == end_ptr {
            debug_assert_eq!(n, 3450);
            break;
        }
    }

    let mut seen = const {
        let mut seen = [0u64; (73 * 73 + 63) / 64];
        let mut i = 0;
        while i < 73 {
            let a = [(0, i), (i, 0), (72, i), (i, 72)];
            let mut j = 0;
            while j < 4 {
                let (x, y) = a[j];
                let idx = 73 * y + x;
                seen[idx / 64] |= 1 << (idx % 64);
                j += 1;
            }

            i += 1;
        }
        seen
    };

    const START: usize = 73 * (0 + 1) + (0 + 1);
    const END: usize = 73 * (70 + 1) + (70 + 1);

    let mut stack = [MaybeUninit::<u16>::uninit(); 1024];
    let mut stack_len = 0;

    *stack[stack_len].as_mut_ptr() = START as u16;
    stack_len += 1;
    *seen.get_unchecked_mut(START / 64) |= 1 << (START % 64);

    let mut queue = [MaybeUninit::<u16>::uninit(); 1024];
    let mut queue_len = 0;

    let mut min = 3450;

    loop {
        stack_len -= 1;
        let pos = *stack.get_unchecked(stack_len).as_ptr() as usize;

        debug_assert!(*levels.get_unchecked(pos) >= min);

        if pos == END {
            break;
        }

        const LEFT: usize = -1isize as usize;
        const RIGHT: usize = 1;
        const UP: usize = -73isize as usize;
        const DOWN: usize = 73;
        for dir in [LEFT, RIGHT, UP, DOWN] {
            let new_pos = pos.wrapping_add(dir);
            let g = seen.get_unchecked_mut(new_pos / 64);
            if *g & (1 << (new_pos % 64)) == 0 {
                *g |= 1 << (new_pos % 64);

                let level = *levels.get_unchecked(new_pos);
                if level >= min {
                    *stack.get_unchecked_mut(stack_len).as_mut_ptr() = new_pos as u16;
                    stack_len += 1;
                } else {
                    *queue.get_unchecked_mut(queue_len).as_mut_ptr() = level;
                    queue_len += 1;
                    bheap::push(std::slice::from_raw_parts_mut(
                        queue.as_mut_ptr().cast::<u16>(),
                        queue_len,
                    ));
                }
            }
        }

        if stack_len == 0 {
            debug_assert!(queue_len > 0);
            let level = *queue.get_unchecked(0).as_ptr();
            bheap::pop(std::slice::from_raw_parts_mut(
                queue.as_mut_ptr().cast::<u16>(),
                queue_len,
            ));
            queue_len -= 1;

            *stack[0].as_mut_ptr() = *levels_list.get_unchecked(level as usize).as_ptr();
            stack_len = 1;

            debug_assert!(level < min);
            min = level;
        }
    }

    let pos = *levels_list.get_unchecked(min as usize).as_ptr();
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

mod bheap {
    #[inline(always)]
    pub unsafe fn pop<T: Copy + Ord>(heap: &mut [T]) {
        if heap.len() > 1 {
            // len = len - 1
            //
            // sift_down_to_bottom(0)

            let start = 0;
            let end = heap.len() - 1;

            let hole = *heap.get_unchecked(heap.len() - 1);
            let mut hole_pos = start;
            let mut child = 2 * hole_pos + 1;

            while child <= end.saturating_sub(2) {
                child += (*heap.get_unchecked(child) <= *heap.get_unchecked(child + 1)) as usize;

                *heap.get_unchecked_mut(hole_pos) = *heap.get_unchecked(child);
                hole_pos = child;

                child = 2 * hole_pos + 1;
            }

            if child == end - 1 {
                *heap.get_unchecked_mut(hole_pos) = *heap.get_unchecked(child);
                hole_pos = child;
            }

            // sift_up(start, hole_pos)
            while hole_pos > start {
                let parent = (hole_pos - 1) / 2;

                if hole <= *heap.get_unchecked(parent) {
                    break;
                }

                *heap.get_unchecked_mut(hole_pos) = *heap.get_unchecked(parent);
                hole_pos = parent;
            }

            *heap.get_unchecked_mut(hole_pos) = hole;
        }
    }

    #[inline(always)]
    pub unsafe fn push<T: Copy + Ord>(heap: &mut [T]) {
        // sift_up(0, heap.len() - 1)
        let start = 0;
        let pos = heap.len() - 1;

        let hole = *heap.get_unchecked(pos);
        let mut hole_pos = pos;

        while hole_pos > start {
            let parent = (hole_pos - 1) / 2;

            if hole <= *heap.get_unchecked(parent) {
                break;
            }

            *heap.get_unchecked_mut(hole_pos) = *heap.get_unchecked(parent);
            hole_pos = parent;
        }

        *heap.get_unchecked_mut(hole_pos) = hole;
    }
}
