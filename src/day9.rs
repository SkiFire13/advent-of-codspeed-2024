#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]

use std::mem::MaybeUninit;

pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
}

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
        let new_pos = pos + len;
        tot += (idf / 2) * ((new_pos * (new_pos - 1) / 2) - (pos * (pos.wrapping_sub(1)) / 2));
        pos = new_pos;

        idf += 1;

        let mut fill_len = (*input.get_unchecked(idf) - b'0') as usize;
        loop {
            if fill_len >= idb_len {
                let new_pos = pos + idb_len;
                tot += (idb / 2) * ((new_pos * (new_pos - 1) / 2) - (pos * (pos - 1) / 2));
                pos = new_pos;
                fill_len -= idb_len;
                idb -= 2;
                idb_len = (*input.get_unchecked(idb) - b'0') as usize;
            } else {
                let new_pos = pos + fill_len;
                tot += (idb / 2) * ((new_pos * (new_pos - 1) / 2) - (pos * (pos - 1) / 2));
                pos = new_pos;
                idb_len -= fill_len;
                break;
            }
            if idb < idf {
                break 'outer;
            }
        }

        idf += 1;
    }

    let new_pos = pos + idb_len;
    tot += (idb / 2) * ((new_pos * (new_pos - 1) / 2) - (pos * (pos - 1) / 2));

    tot as u64
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let input = input.as_bytes();

    let mut pos = 0;
    let mut poss = MaybeUninit::<[u32; 19999]>::uninit();
    let mut heaps = MaybeUninit::<[[u16; 1250]; 10]>::uninit();
    let mut heaps_len = [1; 10];

    for i in 0..10 {
        *heaps.get_mut(i).get_mut(0).as_mut_ptr() = u16::MAX;
    }

    for i in 0..19999 {
        let b = *input.get_unchecked(i) - b'0';
        *poss.get_mut(i).as_mut_ptr() = pos;
        pos += b as u32;

        if i % 2 == 1 {
            let len = heaps_len.get_unchecked_mut(b as usize);
            let heap = heaps.get_mut(b as usize);
            *heap.get_mut(*len).as_mut_ptr() = i as u16 / 2;
            *len += 1;
        }
    }

    for i in 0..10 {
        let len = heaps_len[i];
        bheap::heapify(
            &mut *heaps
                .get_mut(i)
                .as_mut_ptr()
                .as_mut_slice()
                .get_unchecked_mut(..len),
        );
    }

    let mut tot = 0;

    for i in (0..10000).rev() {
        let b = *input.get_unchecked(2 * i) - b'0';
        let mut min_j = u16::MAX;
        let mut min_h = 0;
        for h in (0..10).rev() {
            if h >= b {
                let j = *heaps.get(h as usize).get(0).as_ptr();
                if j < min_j {
                    min_j = j;
                    min_h = h;
                }
            }
        }

        if (min_j as usize) < i {
            let len = heaps_len.get_unchecked_mut(min_h as usize);
            let heap = heaps.get_mut(min_h as usize);
            bheap::pop(&mut *heap.as_mut_ptr().as_mut_slice().get_unchecked_mut(..*len));
            *len -= 1;

            if min_h != b {
                let len = heaps_len.get_unchecked_mut((min_h - b) as usize);
                let heap = heaps.get_mut((min_h - b) as usize);
                *heap.get_mut(*len).as_mut_ptr() = min_j;
                *len += 1;
                bheap::push(&mut *heap.as_mut_ptr().as_mut_slice().get_unchecked_mut(..*len));
            }

            let pos = *poss.get(1 + 2 * min_j as usize).as_ptr() as usize;
            let new_pos = pos + b as usize;
            tot += i * ((new_pos * (new_pos - 1) / 2) - (pos * (pos - 1) / 2));
            *poss.get_mut(1 + 2 * min_j as usize).as_mut_ptr() += b as u32;
        } else {
            let pos = *poss.get(2 * i).as_ptr() as usize;
            let new_pos = pos + b as usize;
            tot += i * ((new_pos * (new_pos - 1) / 2) - (pos * (pos.wrapping_sub(1)) / 2));
        }
    }

    tot as u64
}

trait MUHelper<T> {
    unsafe fn get(&self, i: usize) -> &MaybeUninit<T>;
    unsafe fn get_mut(&mut self, i: usize) -> &mut MaybeUninit<T>;
}
impl<T, const N: usize> MUHelper<T> for MaybeUninit<[T; N]> {
    unsafe fn get(&self, i: usize) -> &MaybeUninit<T> {
        &*self.as_ptr().cast::<MaybeUninit<T>>().add(i)
    }

    unsafe fn get_mut(&mut self, i: usize) -> &mut MaybeUninit<T> {
        &mut *self.as_mut_ptr().cast::<MaybeUninit<T>>().add(i)
    }
}

mod bheap {
    #[inline(always)]
    pub unsafe fn heapify<T: Copy + Ord>(heap: &mut [T]) {
        let mut n = heap.len() / 2;
        'outer: while n > 0 {
            n -= 1;

            // sift_down(n) => sift_down_range(n, len)
            let end = heap.len();
            let hole = *heap.get_unchecked(n);
            let mut hole_pos = n;
            let mut child = 2 * hole_pos + 1;

            while child <= end.saturating_sub(2) {
                child += (heap.get_unchecked(child) >= heap.get_unchecked(child + 1)) as usize;

                if hole <= *heap.get_unchecked(child) {
                    *heap.get_unchecked_mut(hole_pos) = hole;
                    continue 'outer;
                }

                *heap.get_unchecked_mut(hole_pos) = *heap.get_unchecked(child);
                hole_pos = child;
                child = 2 * hole_pos + 1;
            }

            if child == end - 1 && hole > *heap.get_unchecked(child) {
                *heap.get_unchecked_mut(hole_pos) = *heap.get_unchecked(child);
                hole_pos = child;
            }

            *heap.get_unchecked_mut(hole_pos) = hole;
        }
    }

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
