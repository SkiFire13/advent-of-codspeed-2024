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
    let list0 = list0.as_mut_ptr();
    let list1 = list1.as_mut_ptr();
    let (mut len0, mut len1) = (0, 0);

    let mut count = i32x8::splat(0);

    let mut ptr = input.as_ptr();
    let end = ptr.add(input.len());
    loop {
        let b = ptr.add(4).cast::<u8x32>().read_unaligned();
        let m = b.simd_eq(u8x32::splat(b'#')).to_bitmask() as u32;

        let prevsi: *const [u32; 256];
        let prevsi_len: usize;

        core::arch::asm!(
            "cmp {c}, {dot}",

            "mov {prevsi}, {list0}",
            "mov {prevsi_len}, {len0:r}",
            "cmove {prevsi}, {list1}",
            "cmove {prevsi_len}, {len1:r}",

            "mov {lista}, {list1}",
            "mov {lena}, {len1:r}",
            "cmove {lista}, {list0}",
            "cmove {lena}, {len0:r}",
            "mov [{lista} + 4 * {lena}], {m:e}",

            "lea {new_len0}, [{len0:r} + 1]",
            "lea {new_len1}, [{len1:r} + 1]",
            "cmove {len0:r}, {new_len0}",
            "cmovne {len1:r}, {new_len1}",
            c = in(reg_byte) *ptr.add(4),
            m = in(reg) m,
            dot = const b'.',
            prevsi = out(reg) prevsi,
            prevsi_len = out(reg) prevsi_len,
            list0 = in(reg) list0,
            len0 = inout(reg) len0,
            list1 = in(reg) list1,
            len1 = inout(reg) len1,
            lista = out(reg) _,
            lena = out(reg) _,
            new_len0 = out(reg) _,
            new_len1 = out(reg) _,
        );

        {
            let mut ptr = prevsi.as_ptr().cast::<u32x8>();
            let end = prevsi.as_ptr().add(prevsi_len).cast::<u32x8>();
            let m = u32x8::splat(m);
            let z = u32x8::splat(0);

            if ptr <= end.wrapping_sub(16) {
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

            if ptr < end.wrapping_sub(1) {
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
