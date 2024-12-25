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
            let m = u32x8::splat(m);
            let z = u32x8::splat(0);

            while ptr.add(8) <= end {
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

            while ptr < end.sub(1) {
                count -= (*ptr & m).simd_eq(z).to_int();
                ptr = ptr.add(1);
            }

            #[rustfmt::skip]
            static MASKS: [u32x8; 9] = [
                u32x8::from_array([u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,]),
                u32x8::from_array([0,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,]),
                u32x8::from_array([0,0,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,]),
                u32x8::from_array([0,0,0,u32::MAX,u32::MAX,u32::MAX,u32::MAX,u32::MAX,]),
                u32x8::from_array([0,0,0,0,u32::MAX,u32::MAX,u32::MAX,u32::MAX,]),
                u32x8::from_array([0,0,0,0,0,u32::MAX,u32::MAX,u32::MAX,]),
                u32x8::from_array([0,0,0,0,0,0,u32::MAX,u32::MAX,]),
                u32x8::from_array([0,0,0,0,0,0,0,u32::MAX,]),
                u32x8::from_array([0,0,0,0,0,0,0,0]),
            ];
            let rem = end.cast::<u32>().offset_from(ptr.cast::<u32>()) as usize;
            let b = *ptr | *MASKS.get_unchecked(rem);
            count -= (b & m).simd_eq(z).to_int();
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
