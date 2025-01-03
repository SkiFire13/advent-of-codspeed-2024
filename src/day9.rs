#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]

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
    let mut tot = 0;
    let mut left = 0;
    let mut rights = [20000; 9];
    let mut moved = [0u64; 19999usize.div_ceil(64)];

    while left < rights[9 - 1] {
        let b = *input.get_unchecked(left) as usize - b'0' as usize;
        if moved.get_unchecked(left / 2 / 64) & (1 << (left / 2 % 64)) == 0 {
            tot += (left / 2) * (b * (2 * pos + b - 1) / 2);
            *moved.get_unchecked_mut(left / 2 / 64) |= 1 << (left / 2 % 64);
        }
        pos += b;

        left += 1;

        let mut hole_size = *input.get_unchecked(left) as usize - b'0' as usize;
        while hole_size > 0 {
            let mut search = *rights.get_unchecked(hole_size - 1);

            if search - 2 < left {
                break;
            }

            let limit = i8x16::splat(b'0' as i8 + hole_size as i8);
            loop {
                search -= 32;

                let block = u8x32::from_slice(input.get_unchecked(search..search + 32));
                let (block, _) = block.deinterleave(block);
                let block = block.resize::<16>(16);
                let mask = block.cast::<i8>().simd_le(limit).to_bitmask();

                let base = (search / 2) >> 3;
                let moved = moved.as_ptr().byte_add(base).read_unaligned() >> ((search / 2) & 7);

                let mask = mask & !moved;
                if mask != 0 {
                    let j = (mask as u16).leading_zeros();
                    search = search + 30 - 2 * j as usize;
                    break;
                }

                if search < left {
                    break;
                }
            }

            for i in 1..hole_size + 1 {
                let right = rights.get_unchecked_mut(i - 1);
                *right = (*right).min(search);
            }

            if search < left {
                break;
            }

            *moved.get_unchecked_mut(search / 2 / 64) |= 1 << (search / 2 % 64);

            let b = *input.get_unchecked(search) as usize - b'0' as usize;
            tot += (search / 2) * (b * (2 * pos + b - 1) / 2);
            pos += b;
            hole_size -= b;
        }

        pos += hole_size;
        left += 1;
    }

    tot as u64
}
