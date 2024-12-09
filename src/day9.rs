#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]

use std::mem::MaybeUninit;

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

    'outer: while idf < idb {
        let len = (*input.get_unchecked(idf) - b'0') as usize;
        tot += (idf / 2) * (len * (2 * pos + len - 1) / 2);
        pos += len;

        idf += 1;

        let mut fill_len = (*input.get_unchecked(idf) - b'0') as usize;
        loop {
            if fill_len >= idb_len {
                tot += (idb / 2) * (idb_len * (2 * pos + idb_len - 1) / 2);
                pos += idb_len;
                fill_len -= idb_len;
                idb -= 2;
                idb_len = (*input.get_unchecked(idb) - b'0') as usize;
            } else {
                tot += (idb / 2) * (fill_len * (2 * pos + fill_len - 1) / 2);
                pos += fill_len;
                idb_len -= fill_len;
                break;
            }
            if idb < idf {
                break 'outer;
            }
        }

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
    let mut poss = MaybeUninit::<[u32; 19999]>::uninit();
    let mut queues = MaybeUninit::<[[u16; 1250]; 10]>::uninit();
    let mut queues_len = [1; 10];

    for i in 0..10 {
        *queues.get_mut(i).get_mut(0).as_mut_ptr() = u16::MAX;
    }

    for i in 0..19999 {
        let b = *input.get_unchecked(i) - b'0';
        *poss.get_mut(i).as_mut_ptr() = pos;
        pos += b as u32;
    }

    for i in (0..9999).rev() {
        let i = 2 * i + 1;
        let b = *input.get_unchecked(i) - b'0';
        let len = queues_len.get_unchecked_mut(b as usize);
        let queue = queues.get_mut(b as usize);
        *queue.get_mut(*len).as_mut_ptr() = i as u16;
        *len += 1;
    }

    let mut tot = 0;

    for i in (0..10000).rev() {
        let i = 2 * i;
        let b = *input.get_unchecked(i) - b'0';
        let mut min_j = u16::MAX;
        let mut min_h = 0;
        for h in (0..10).rev() {
            if h >= b {
                let j = *queues
                    .get(h as usize)
                    .get(queues_len[h as usize] - 1)
                    .as_ptr();
                if j < min_j {
                    min_j = j;
                    min_h = h;
                }
            }
        }

        if (min_j as usize) < i {
            *queues_len.get_unchecked_mut(min_h as usize) -= 1;

            if min_h != b {
                let len = queues_len.get_unchecked_mut((min_h - b) as usize);
                let queue = queues.get_mut((min_h - b) as usize);
                let mut pos = *len;
                while *queue.get(pos - 1).as_ptr() < min_j {
                    pos -= 1;
                }
                let ptr = queue.as_mut_ptr().cast::<u16>();
                std::ptr::copy(ptr.add(pos), ptr.add(pos + 1), *len - pos);
                *queue.get_mut(pos).as_mut_ptr() = min_j;
                *len += 1;
            }

            let pos = *poss.get(min_j as usize).as_ptr() as usize;
            let len = b as usize;
            tot += (i / 2) * (len * (2 * pos + len - 1) / 2);
            *poss.get_mut(min_j as usize).as_mut_ptr() += b as u32;
        } else {
            let pos = *poss.get(i).as_ptr() as usize;
            let len = b as usize;
            tot += (i / 2) * (len * (2 * pos + len - 1) / 2);
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
