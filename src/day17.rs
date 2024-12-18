#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

// pub fn run(input: &str) -> i64 {
//     part2(input) as i64
// }

pub fn run(input: &str) -> &'static str {
    part1(input)
}

#[inline(always)]
pub fn part1(input: &str) -> &'static str {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

#[allow(long_running_const_eval)]
static LUT: [u64; (6 * 6 * 6) * (8 * 8 * 8 * 8)] =
    unsafe { std::mem::transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/d17p2.lut"))) };
// [0; (6 * 6 * 6) * (8 * 8 * 8 * 8)];

static mut PART1_OUTPUT: [u8; 2 * 9] = [b','; 2 * 9];

#[inline(always)]
fn parse8(n: u64) -> u64 {
    use std::num::Wrapping as W;

    let mut n = W(n);
    let mask = W(0xFF | (0xFF << 32));
    let mul1 = W(100 + (1000000 << 32));
    let mul2 = W(1 + (10000 << 32));

    n -= W(u64::from_ne_bytes([b'0'; 8]));
    n = (n * W(10)) + (n >> 8);
    n = (((n & mask) * mul1) + (((n >> 16) & mask) * mul2)) >> 32;

    n.0 as u64
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> &'static str {
    let input = input.as_bytes();

    let mut a = parse8(input.as_ptr().add(12).cast::<u64>().read_unaligned());

    let mut ptr = input.as_ptr().add(input.len()).sub(26);
    let xor1 = *ptr - b'0';
    ptr = ptr.add(6);
    let mut xor2_ptr = std::ptr::null();
    let mut four_ptr = std::ptr::null();
    for _ in 0..3 {
        if *ptr == b'1' {
            xor2_ptr = ptr;
        }
        if *ptr == b'4' {
            four_ptr = ptr;
        }
        ptr = ptr.add(4);
    }

    let mut xor2 = 0;
    let mut xor3 = 0;
    if xor2_ptr < four_ptr {
        xor2 = *xor2_ptr.add(2) - b'0';
    } else {
        xor3 = *xor2_ptr.add(2) - b'0';
    }

    let mut out_len = 0;

    loop {
        let mut b = (a & 0b111) as u8;
        b ^= xor1;

        let c = a >> b;
        b ^= xor2;
        b ^= c as u8;
        b ^= xor3;
        a >>= 3;

        *PART1_OUTPUT.get_unchecked_mut(out_len) = (b & 0b111) + b'0';
        out_len += 2;

        if a == 0 {
            break;
        }
    }

    std::str::from_utf8_unchecked(PART1_OUTPUT.get_unchecked(..out_len - 1))
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let ptr = input.as_bytes().as_ptr();

    let mut offset = 0;

    offset = 0 * offset + (*ptr.add(65) as usize - b'0' as usize);
    offset = 6 * offset + (*ptr.add(71) as usize - b'0' as usize);
    offset = 8 * offset + (*ptr.add(73) as usize - b'0' as usize);
    offset = 6 * offset + (*ptr.add(75) as usize - b'0' as usize);
    offset = 8 * offset + (*ptr.add(77) as usize - b'0' as usize);
    offset = 6 * offset + (*ptr.add(79) as usize - b'0' as usize);
    offset = 8 * offset + (*ptr.add(81) as usize - b'0' as usize);

    *LUT.get_unchecked(offset)
}
