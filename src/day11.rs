#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]

pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

#[inline(always)]
pub fn part1(input: &str) -> u32 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

macro_rules! solve {
    ($input:ident, $lut:literal, $ty:ident) => {{
        let lut = include_bytes!(concat!(env!("OUT_DIR"), $lut))
            .as_ptr()
            .cast::<$ty>();
        let mut iter = $input.as_bytes().iter();
        let mut tot = 0;

        while !iter.as_slice().is_empty() {
            let mut n = (iter.as_slice().get_unchecked(0) - b'0') as usize;
            iter = iter.as_slice().get_unchecked(1..).iter();
            loop {
                let d = iter.as_slice().get_unchecked(0).wrapping_sub(b'0');
                iter = iter.as_slice().get_unchecked(1..).iter();
                if d >= 10 {
                    break;
                }
                n = 10 * n + d as usize;
            }

            tot += lut.add(n).read_unaligned();
        }

        tot
    }};
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u32 {
    solve!(input, "/d11p1.lut", u32)
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    solve!(input, "/d11p2.lut", u64)
}
