#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]

pub fn run(input: &str) -> i64 {
    part2(input) as i64
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

    while idf < idb {
        let len = (*input.get_unchecked(idf) - b'0') as usize;
        let new_pos = pos + len;
        tot += (idf / 2) * ((new_pos * (new_pos - 1) / 2) - (pos * (pos.wrapping_sub(1)) / 2));
        pos = new_pos;

        idf += 1;
        if idf < idb {
            let mut fill_len = (*input.get_unchecked(idf) - b'0') as usize;

            while fill_len != 0 && idf < idb {
                let len = std::cmp::min(fill_len, idb_len);
                let new_pos = pos + len;
                tot += (idb / 2) * ((new_pos * (new_pos - 1) / 2) - (pos * (pos - 1) / 2));
                pos = new_pos;
                idb_len -= len;
                fill_len -= len;
                if idb_len == 0 {
                    idb -= 2;
                    idb_len = (*input.get_unchecked(idb) - b'0') as usize;
                }
            }

            idf += 1;
        }
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
    // TODO: MaybeUninit
    let mut poss = [0; 19999];
    let mut heaps = [[0; 1250]; 10];
    let mut heaps_len = [1; 10];

    for i in 0..10 {
        heaps[i][0] = u16::MAX;
    }

    for i in 0..19999 {
        let b = *input.get_unchecked(i) - b'0';
        *poss.get_unchecked_mut(i) = pos;
        pos += b as u32;

        if i % 2 == 1 {
            let len = heaps_len.get_unchecked_mut(b as usize);
            let heap = heaps.get_unchecked_mut(b as usize);
            *heap.get_unchecked_mut(*len) = i as u16 / 2;
            *len += 1;
        }
    }

    for i in 0..10 {
        let len = heaps_len[i];
        bheap::heapify(heaps[i].get_unchecked_mut(..len));
    }

    let mut tot = 0;

    for i in (0..10000).rev() {
        let b = *input.get_unchecked(2 * i) - b'0';
        let mut min_j = u16::MAX;
        let mut min_h = 0;
        for h in (0..10).rev() {
            if h >= b {
                let j = *heaps.get_unchecked(h as usize).get_unchecked(0);
                if j < min_j {
                    min_j = j;
                    min_h = h;
                }
            }
        }

        if (min_j as usize) < i {
            let len = heaps_len.get_unchecked_mut(min_h as usize);
            let heap = heaps.get_unchecked_mut(min_h as usize);
            bheap::pop(heap.get_unchecked_mut(..*len));
            *len -= 1;

            if min_h != b {
                let len = heaps_len.get_unchecked_mut((min_h - b) as usize);
                let heap = heaps.get_unchecked_mut((min_h - b) as usize);
                *heap.get_unchecked_mut(*len) = min_j;
                *len += 1;
                bheap::push(heap.get_unchecked_mut(..*len));
            }

            let pos = *poss.get_unchecked_mut(1 + 2 * min_j as usize) as usize;
            let new_pos = pos + b as usize;
            tot += i * ((new_pos * (new_pos - 1) / 2) - (pos * (pos - 1) / 2));
            *poss.get_unchecked_mut(1 + 2 * min_j as usize) += b as u32;
        } else {
            let pos = *poss.get_unchecked(2 * i) as usize;
            let new_pos = pos + b as usize;
            tot += i * ((new_pos * (new_pos - 1) / 2) - (pos * (pos.wrapping_sub(1)) / 2));
        }
    }

    tot as u64
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
