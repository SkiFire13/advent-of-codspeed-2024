#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part1(input) as i64
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
    let input = input.as_bytes();

    #[repr(align(64))]
    struct Aligned([u32; 500]);

    static mut PREVS0: Aligned = Aligned([0; 500]);
    static mut PREVS1: Aligned = Aligned([0; 500]);

    let prevs0 = &mut PREVS0;
    let mut prevs0_len = 0;
    let prevs1 = &mut PREVS1;
    let mut prevs1_len = 0;

    let mut count = i32x8::splat(0);

    let mut ptr = input.as_ptr();
    let end = ptr.add(input.len());

    loop {
        let b = ptr.add(4).cast::<u8x32>().read_unaligned();
        let m = b.simd_eq(u8x32::splat(b'#')).to_bitmask() as u32;

        let (prevsi, prevsi_len, prevsj, prevsj_len) = if *ptr.add(4) == b'#' {
            (&*prevs0, prevs0_len, &mut *prevs1, &mut prevs1_len)
        } else {
            (&*prevs1, prevs1_len, &mut *prevs0, &mut prevs0_len)
        };

        *prevsj.0.get_unchecked_mut(*prevsj_len) = m;
        *prevsj_len += 1;

        {
            let mut ptr = prevsi.0.as_ptr().cast::<u32x8>();
            let end = prevsi.0.as_ptr().add(prevsi_len).cast::<u32x8>();

            while ptr.add(8) <= end {
                count -= ((*ptr) & u32x8::splat(m)).simd_eq(u32x8::splat(0)).to_int();
                ptr = ptr.add(1);
                count -= ((*ptr) & u32x8::splat(m)).simd_eq(u32x8::splat(0)).to_int();
                ptr = ptr.add(1);
                count -= ((*ptr) & u32x8::splat(m)).simd_eq(u32x8::splat(0)).to_int();
                ptr = ptr.add(1);
                count -= ((*ptr) & u32x8::splat(m)).simd_eq(u32x8::splat(0)).to_int();
                ptr = ptr.add(1);
                count -= ((*ptr) & u32x8::splat(m)).simd_eq(u32x8::splat(0)).to_int();
                ptr = ptr.add(1);
                count -= ((*ptr) & u32x8::splat(m)).simd_eq(u32x8::splat(0)).to_int();
                ptr = ptr.add(1);
                count -= ((*ptr) & u32x8::splat(m)).simd_eq(u32x8::splat(0)).to_int();
                ptr = ptr.add(1);
                count -= ((*ptr) & u32x8::splat(m)).simd_eq(u32x8::splat(0)).to_int();
                ptr = ptr.add(1);
            }
            while ptr.add(1) <= end {
                count -= ((*ptr) & u32x8::splat(m)).simd_eq(u32x8::splat(0)).to_int();
                ptr = ptr.add(1);
            }

            #[rustfmt::skip]
            static MASKS: [u32x8; 8] = [
                u32x8::from_array([u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,]),
                u32x8::from_array([0,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,]),
                u32x8::from_array([0,0,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,]),
                u32x8::from_array([0,0,0,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,]),
                u32x8::from_array([0,0,0,0,u32::MAX,u32::MAX,u32::MAX,u32::MAX,]),
                u32x8::from_array([0,0,0,0,0,u32::MAX,u32::MAX,u32::MAX,]),
                u32x8::from_array([0,0,0,0,0,0,u32::MAX,u32::MAX,]),
                u32x8::from_array([0,0,0,0,0,0,0,u32::MAX,]),
            ];

            let b = *ptr | MASKS[end.cast::<u32>().offset_from(ptr.cast::<u32>()) as usize];
            count -= (b & u32x8::splat(m)).simd_eq(u32x8::splat(0)).to_int();
        }

        ptr = ptr.add(43);
        if ptr >= end {
            break;
        }
    }

    count.reduce_sum() as u64
}

#[allow(unused)]
#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    0
}
