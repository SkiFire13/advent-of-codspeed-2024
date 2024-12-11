#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]

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

unsafe fn solve(input: &[u8], lut: &[u8]) -> u64 {
    let mut iter = input.iter();
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

        tot += u64::from_ne_bytes(lut.get_unchecked(8 * n..8 * n + 8).try_into().unwrap());
    }

    tot
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    solve(
        input.as_bytes(),
        include_bytes!(concat!(env!("OUT_DIR"), "/d11p1.lut")),
    )
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    solve(
        input.as_bytes(),
        include_bytes!(concat!(env!("OUT_DIR"), "/d11p2.lut")),
    )
}
