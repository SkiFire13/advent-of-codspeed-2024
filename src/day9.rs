#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]
#![feature(fn_align)]

use std::arch::x86_64::*;
use std::mem::MaybeUninit;
use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

#[inline(always)]
pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let mut input = input.as_bytes();
    if input[input.len() - 1] == b'\n' {
        input = &input[..input.len() - 1];
    }

    let mut idf = 0;
    let mut idb = input.len() - 1;
    let mut idb_len = (*input.get_unchecked(idb) - b'0') as usize;

    let mut tot = 0;
    let mut pos = 0;

    'outer: while idf < idb {
        let len = (*input.get_unchecked(idf) - b'0') as usize;
        tot += (idf / 2) * (len * (2 * pos + len - 1) / 2);
        pos += len;

        idf += 1;

        let mut fill_len = (*input.get_unchecked(idf) - b'0') as usize;
        while fill_len >= idb_len {
            tot += (idb / 2) * (idb_len * (2 * pos + idb_len - 1) / 2);
            pos += idb_len;
            fill_len -= idb_len;
            idb -= 2;
            idb_len = (*input.get_unchecked(idb) - b'0') as usize;
            if idb < idf {
                break 'outer;
            }
        }

        tot += (idb / 2) * (fill_len * (2 * pos + fill_len - 1) / 2);
        pos += fill_len;
        idb_len -= fill_len;

        idf += 1;
    }

    if idf == idb {
        tot += (idb / 2) * (idb_len * (2 * pos + idb_len - 1) / 2);
    }

    tot as u64
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let input = input.as_bytes();

    let mut pos = 0;
    let mut poss = MaybeUninit::<[u16; 5700]>::uninit();
    let mut queues = MaybeUninit::<[[u16; 750]; 10]>::uninit();
    let mut queues_len = [1; 10];

    for i in 0..10 {
        *queues.get_mut(i).get_mut(0).as_mut_ptr() = u16::MAX;
    }

    let mut i = 19999;
    let mut j = 9999;
    loop {
        i -= 1;

        let b = *input.get_unchecked(i) - b'0';
        pos += b as u16;

        if j == 5700 {
            break;
        }

        i -= 1;
        j -= 1;

        let b = *input.get_unchecked(i) - b'0';
        pos += b as u16;
    }
    let posj_base = pos;
    pos = 0;
    loop {
        i -= 1;
        j -= 1;

        let b = *input.get_unchecked(i) - b'0';
        pos += b as u16;

        *poss.get_mut(j).as_mut_ptr() = pos;

        let len = queues_len.get_unchecked_mut(b as usize);
        let queue = queues.get_mut(b as usize);
        *queue.get_mut(*len).as_mut_ptr() = j as u16;
        *len += 1;

        i -= 1;

        let b = *input.get_unchecked(i) - b'0';
        pos += b as u16;

        if i == 0 {
            break;
        }
    }

    queues_len[0] = 1;
    let mut mins = [u16::MAX; 16];

    mins[0] = *queues.get(0).get(0).as_ptr();
    for i in 1..10 {
        mins[i] = *queues.get(i).get(queues_len[i] - 1).as_ptr();
    }

    macro_rules! queue_push {
        ($i:expr, $elem:expr) => {{
            let i = $i as usize;
            let elem = $elem;
            let len = queues_len.get_unchecked_mut(i);
            let queue = queues.get_mut(i);
            let mut pos = *len;
            while *queue.get(pos - 1).as_ptr() < elem {
                pos -= 1;
            }
            let ptr = queue.as_mut_ptr().cast::<u16>();
            std::ptr::copy(ptr.add(pos), ptr.add(pos + 1), *len - pos);
            *queue.get_mut(pos).as_mut_ptr() = elem;
            *len += 1;

            let min = mins.get_unchecked_mut(i);
            *min = (*min).min(elem);
        }};
    }

    macro_rules! bheap_push {
        ($elem:expr) => {{
            let elem = $elem;
            let len = queues_len.get_unchecked_mut(0);
            let heap = queues.get_mut(0);
            *heap.get_mut(*len).as_mut_ptr() = elem;
            *len += 1;
            bheap::push(&mut *heap.as_mut_ptr().as_mut_slice().get_unchecked_mut(..*len));

            let min = mins.get_unchecked_mut(0);
            *min = (*min).min(elem);
        }};
    }

    macro_rules! queue_pop {
        ($i:expr) => {{
            let i = $i as usize;
            let len = queues_len.get_unchecked_mut(i);
            *len -= 1;

            let min = mins.get_unchecked_mut(i);
            *min = *queues.get(i).get(*len - 1).as_ptr();
        }};
    }

    macro_rules! bheap_pop {
        () => {{
            let len = queues_len.get_unchecked_mut(0);
            let heap = queues.get_mut(0);
            bheap::pop(&mut *heap.as_mut_ptr().as_mut_slice().get_unchecked_mut(..*len));
            *len -= 1;

            *mins.get_unchecked_mut(0) = *heap.get(0).as_ptr();
        }};
    }

    const MASKS: [u16x16; 10] = {
        let mut masks = [[0; 16]; 10];

        let mut i = 2;
        while i < 10 {
            let mut j = 0;
            while j < i {
                masks[i][j] = u16::MAX;
                j += 1;
            }

            i += 1;
        }

        unsafe { std::mem::transmute(masks) }
    };

    macro_rules! minpos {
        ($b:expr) => {{
            let b = $b as usize;

            let mins = u16x16::from_array(mins) | *MASKS.get_unchecked(b);

            let res = u16x8::from(_mm_minpos_epu16(u16x8::from_slice(mins.as_array()).into()));
            let mut min = res[0];
            let mut pos = res[1];

            if mins[8] < min {
                min = mins[8];
                pos = 8;
            }

            if mins[9] < min {
                min = mins[9];
                pos = 9;
            }

            (min, pos as usize)
        }};
    }

    let mut tot = 0;

    let mut max_b;

    let mut i = 19998;
    let mut posi = pos as usize + posj_base as usize;
    let total_pos = pos as usize;
    loop {
        let b = *input.get_unchecked(i) as usize - b'0' as usize;

        posi -= b;

        let (min_j, min_h) = minpos!(b);

        if min_j as usize >= i / 2 {
            let len = b as usize;
            tot += (i / 2) * (len * (2 * posi + len - 1) / 2);

            max_b = b;

            i -= 2;
            posi -= (*input.get_unchecked(i + 1) - b'0') as usize;
            break;
        } else if min_h == 0 {
            bheap_pop!();

            let pos = total_pos - *poss.get(min_j as usize).as_ptr() as usize;
            let len = b as usize;
            tot += (i / 2) * (len * (2 * pos + len - 1) / 2);
            *poss.get_mut(min_j as usize).as_mut_ptr() -= b as u16;
        } else {
            queue_pop!(min_h);

            if min_h - b == 1 {
                bheap_push!(min_j);
            } else if min_h != b {
                queue_push!(min_h - b, min_j);
            }

            let pos = total_pos - *poss.get(min_j as usize).as_ptr() as usize;
            let len = b as usize;
            tot += (i / 2) * (len * (2 * pos + len - 1) / 2);
            *poss.get_mut(min_j as usize).as_mut_ptr() -= b as u16;
        }

        i -= 2;
        posi -= (*input.get_unchecked(i + 1) - b'0') as usize;
    }

    loop {
        let b = *input.get_unchecked(i) as usize - b'0' as usize;

        posi -= b as usize;

        if b < max_b {
            let (min_j, min_h) = minpos!(b);

            if min_j as usize >= i / 2 {
                let len = b as usize;
                tot += (i / 2) * (len * (2 * posi + len - 1) / 2);
                max_b = b;

                if b == 1 {
                    while i != 0 {
                        i -= 2;
                        posi -= (*input.get_unchecked(i + 1) - b'0') as usize;
                        let b = *input.get_unchecked(i) - b'0';
                        posi -= b as usize;
                        let len = b as usize;
                        tot += (i / 2) * (len * (2 * posi + len - 1) / 2);
                    }
                    break;
                }
            } else if min_h == 0 {
                bheap_pop!();

                let pos = total_pos - *poss.get(min_j as usize).as_ptr() as usize;
                let len = b as usize;
                tot += (i / 2) * (len * (2 * pos + len - 1) / 2);
                *poss.get_mut(min_j as usize).as_mut_ptr() -= b as u16;
            } else {
                queue_pop!(min_h);

                if min_h - b == 1 {
                    bheap_push!(min_j);
                } else if min_h != b {
                    queue_push!(min_h - b, min_j);
                }

                let pos = total_pos - *poss.get(min_j as usize).as_ptr() as usize;
                let len = b as usize;
                tot += (i / 2) * (len * (2 * pos + len - 1) / 2);
                *poss.get_mut(min_j as usize).as_mut_ptr() -= b as u16;
            }
        } else {
            let len = b as usize;
            tot += (i / 2) * (len * (2 * posi + len - 1) / 2);
        }

        i -= 2;
        posi -= (*input.get_unchecked(i + 1) - b'0') as usize;
    }

    tot as u64
}

trait MUHelper<T> {
    unsafe fn get(&self, i: usize) -> &MaybeUninit<T>;
    unsafe fn get_mut(&mut self, i: usize) -> &mut MaybeUninit<T>;
}
impl<T, const N: usize> MUHelper<T> for MaybeUninit<[T; N]> {
    unsafe fn get(&self, i: usize) -> &MaybeUninit<T> {
        &*self.as_ptr().as_slice().get_unchecked(i).cast()
    }

    unsafe fn get_mut(&mut self, i: usize) -> &mut MaybeUninit<T> {
        &mut *self.as_mut_ptr().as_mut_slice().get_unchecked_mut(i).cast()
    }
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
                child += (*heap.get_unchecked(child) >= *heap.get_unchecked(child + 1)) as usize;

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

                if hole >= *heap.get_unchecked(parent) {
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

            if hole >= *heap.get_unchecked(parent) {
                break;
            }

            *heap.get_unchecked_mut(hole_pos) = *heap.get_unchecked(parent);
            hole_pos = parent;
        }

        *heap.get_unchecked_mut(hole_pos) = hole;
    }
}
