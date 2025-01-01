#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

use std::arch::x86_64::*;

pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

#[inline(always)]
pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
    // super::day19par::part1(input)
}

#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
    // super::day19par::part2(input)
}

static LUT: [usize; 128] = {
    let mut lut = [usize::MAX; 128];
    lut[b'r' as usize] = 0;
    lut[b'g' as usize] = 1;
    lut[b'b' as usize] = 2;
    lut[b'u' as usize] = 3;
    lut[b'w' as usize] = 4;
    lut
};

#[allow(unused)]
#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    let mut tries = [[0u16; 5]; 1024];
    let mut tries_end = [false; 1024];
    let mut tries_len = 1;

    let mut ptr = input.as_ptr();
    loop {
        let n = ptr.cast::<u64>().read_unaligned();
        let mask = _pext_u64(n, u64::from_ne_bytes([0b00001000; 8]) | (1 << 62));
        let len = mask.trailing_zeros();
        std::hint::assert_unchecked(len > 0 && len <= 8);
        let end = ptr.add(len as usize);

        let mut trie = 0;
        loop {
            // let i = _pext_u64(*ptr as u64 + 13, 0b10010010) - 1;
            let i = *LUT.get_unchecked(*ptr as usize);

            let mut next = *tries.get_unchecked(trie).get_unchecked(i as usize);
            if next == 0 {
                next = tries_len;
                tries_len += 1;
            }
            *tries.get_unchecked_mut(trie).get_unchecked_mut(i as usize) = next;
            trie = next as usize;

            ptr = ptr.add(1);
            if ptr == end {
                break;
            }
        }

        *tries_end.get_unchecked_mut(trie) = true;

        ptr = ptr.add(2);
        if *ptr.sub(2) == b'\n' {
            break;
        }
    }

    let mut queue;
    let mut to_see;

    let mut count = 0;
    let end = input.as_ptr().add(input.len());

    loop {
        queue = 1u64;
        to_see = u64::MAX;

        let base_ptr = ptr;
        loop {
            let pos = 63 - (queue & to_see).leading_zeros();
            to_see &= !(1 << pos);

            let mut ptr = base_ptr.add(pos as usize);
            let mut trie = 0;

            loop {
                let i = *LUT.get_unchecked(*ptr as usize);

                trie = *tries.get_unchecked(trie).get_unchecked(i) as usize;
                if trie == 0 {
                    break;
                }

                ptr = ptr.add(1);

                let b = *tries_end.get_unchecked(trie) as u64;
                queue |= b << ptr.offset_from(base_ptr) as u64;

                if *ptr == b'\n' {
                    break;
                }
            }

            if *ptr == b'\n' && *tries_end.get_unchecked(trie) {
                count += 1;
                break;
            }

            if queue & to_see == 0 {
                break;
            }
        }

        while *ptr != b'\n' {
            ptr = ptr.add(1);
        }
        ptr = ptr.add(1);

        if ptr == end {
            break;
        }
    }

    count
}

#[allow(unused)]
#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let input = input.as_bytes();

    let mut tries = [[0u16; 5]; 1024];
    let mut tries_end = [false; 1024];
    let mut tries_len = 1;

    let mut ptr = input.as_ptr();
    loop {
        let n = ptr.cast::<u64>().read_unaligned();
        let mask = _pext_u64(n, u64::from_ne_bytes([0b00001000; 8]) | (1 << 62));
        let len = mask.trailing_zeros();
        let end = ptr.add(len as usize);

        let mut trie = 0;
        loop {
            // let i = _pext_u64(*ptr as u64 + 13, 0b10010010) - 1;
            let i = *LUT.get_unchecked(*ptr as usize);

            let mut next = *tries.get_unchecked(trie).get_unchecked(i as usize);
            if next == 0 {
                next = tries_len;
                tries_len += 1;
            }
            *tries.get_unchecked_mut(trie).get_unchecked_mut(i as usize) = next;
            trie = next as usize;

            ptr = ptr.add(1);
            if ptr == end {
                break;
            }
        }

        *tries_end.get_unchecked_mut(trie) = true;

        ptr = ptr.add(2);
        if *ptr.sub(2) == b'\n' {
            break;
        }
    }

    let mut queue;

    let mut count = 0;
    let end = input.as_ptr().add(input.len());

    loop {
        queue = [0; 64];
        queue[0] = 1;
        let mut pos = 0;
        let mut outer_ptr = ptr;
        let base_ptr = ptr;

        loop {
            let n = *queue.get_unchecked(pos);

            if n != 0 {
                let mut ptr = outer_ptr;
                let mut trie = 0;

                loop {
                    let i = *LUT.get_unchecked(*ptr as usize);

                    trie = *tries.get_unchecked(trie).get_unchecked(i) as usize;
                    if trie == 0 {
                        break;
                    }

                    ptr = ptr.add(1);

                    if *tries_end.get_unchecked(trie) {
                        *queue.get_unchecked_mut(ptr.offset_from(base_ptr) as usize) += n;
                    }

                    if *ptr == b'\n' {
                        break;
                    }
                }
            }

            pos += 1;
            outer_ptr = outer_ptr.add(1);

            if *outer_ptr == b'\n' {
                count += *queue.get_unchecked(pos);
                break;
            }
        }

        ptr = outer_ptr.add(1);

        if ptr == end {
            break;
        }
    }

    count
}
