#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]
#![feature(fn_align)]

use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

#[inline(always)]
#[repr(align(64))]
pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(_input: &str) -> u64 {
    0
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
#[repr(align(64))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    #[repr(align(64))]
    struct Aligned([u32; 512]);
    static mut PREVS: Aligned = Aligned([0; 512]);
    let (list0, list1) = PREVS.0.split_at_mut(256);
    let (mut len0, mut len1) = (0, 0);

    let mut count = i32x8::splat(0);

    let mut ptr = input.as_ptr();
    let end = ptr.add(input.len());
    loop {
        let b = ptr.add(4).cast::<u8x32>().read_unaligned();
        let m = b.simd_eq(u8x32::splat(b'#')).to_bitmask() as u32;

        let (prevsi, prevsi_len, prevsj, prevsj_len) = if *ptr.add(4) == b'#' {
            (&*list0, len0, &mut *list1, &mut len1)
        } else {
            (&*list1, len1, &mut *list0, &mut len0)
        };

        *prevsj.get_unchecked_mut(*prevsj_len) = m;
        *prevsj_len += 1;

        {
            let mut ptr = prevsi.as_ptr().cast::<u32x8>();
            let end = prevsi.as_ptr().add(prevsi_len).cast::<u32x8>();
            let m = u32x8::splat(m);
            let z = u32x8::splat(0);

            while ptr <= end.wrapping_sub(16) {
                count -= ((*ptr.add(0)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(1)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(2)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(3)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(4)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(5)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(6)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(7)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(8)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(9)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(10)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(11)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(12)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(13)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(14)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(15)) & m).simd_eq(z).to_int();
                ptr = ptr.add(16);
            }

            if ptr <= end.wrapping_sub(8) {
                count -= ((*ptr.add(0)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(1)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(2)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(3)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(4)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(5)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(6)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(7)) & m).simd_eq(z).to_int();
                ptr = ptr.add(8);
            }

            if ptr <= end.wrapping_sub(4) {
                count -= ((*ptr.add(0)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(1)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(2)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(3)) & m).simd_eq(z).to_int();
                ptr = ptr.add(4);
            }

            if ptr <= end.wrapping_sub(2) {
                count -= ((*ptr.add(0)) & m).simd_eq(z).to_int();
                count -= ((*ptr.add(1)) & m).simd_eq(z).to_int();
                ptr = ptr.add(2);
            }

            while ptr < end.wrapping_sub(1) {
                count -= (*ptr & m).simd_eq(z).to_int();
                ptr = ptr.add(1);
            }

            #[rustfmt::skip]
            static MASKS: [u32x8; 9] = {
                let mut masks = [[u32::MAX; 8]; 9];
                let mut i = 0;
                while i < 9 {
                    let mut j = 0;
                    while j < i {
                        masks[i][j] = 0;
                        j += 1;
                    }
                    i += 1;
                }
                unsafe { std::mem::transmute(masks) }
            };
            let rem = end.cast::<u32>().offset_from(ptr.cast::<u32>()) as usize;
            let b = *ptr | *MASKS.get_unchecked(rem);
            count -= (b & m).simd_eq(z).to_int();
        }

        ptr = ptr.wrapping_add(43);
        if ptr >= end {
            break;
        }
    }

    count.reduce_sum() as u64
}
