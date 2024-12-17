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
static LUT: [u64; (4 * 4 * 4 * 4) * (8 * 8 * 8)] =
    unsafe { std::mem::transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/d17p2.lut"))) };

static LUT2: [usize; 6] = {
    let mut lut = [0; 6];
    lut[0] = 0;
    lut[1] = 1;
    lut[4] = 2;
    lut[5] = 3;
    lut
};

static mut PART1_OUTPUT: [u8; 2 * 9] = [b','; 2 * 9];

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> &'static str {
    let input = input.as_bytes();
    let mut ptr = input.as_ptr().add(input.len()).sub(26);

    let xor1 = *ptr - b'0';
    ptr = ptr.add(6);

    let mut xor2ptr = std::ptr::null();
    let mut c_last = true;
    for _ in 0..3 {
        if *ptr == b'1' {
            xor2ptr = ptr;
            c_last = false;
        } else if *ptr == b'4' {
            c_last = true;
        }
        ptr = ptr.add(4);
    }
    let xor2 = *xor2ptr.add(2) - b'0';

    let mut ptr = input.as_ptr().add(12);
    let mut a = *ptr as u64 - b'0' as u64;
    ptr = ptr.add(1);
    while *ptr != b'\n' {
        a = 10 * a + *ptr as u64 - b'0' as u64;
        ptr = ptr.add(1);
    }

    let mut out_len = 0;

    if c_last {
        while a != 0 {
            let mut b = (a & 0b111) as u8;
            b ^= xor1;

            let c = a >> b;
            b ^= xor2;
            b ^= c as u8;
            a >>= 3;

            *PART1_OUTPUT.get_unchecked_mut(out_len) = (b & 0b111) + b'0';
            out_len += 2;
        }
    } else {
        while a != 0 {
            let mut b = (a & 0b111) as u8;
            b ^= xor1;

            let c = a >> b;
            b ^= c as u8;
            b ^= xor2;
            a >>= 3;

            *PART1_OUTPUT.get_unchecked_mut(out_len) = (b & 0b111) + b'0';
            out_len += 2;
        }
    }

    std::str::from_utf8_unchecked(PART1_OUTPUT.get_unchecked(..out_len - 1))
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let input = input.as_bytes();
    let mut ptr = input.as_ptr().add(input.len()).sub(26);

    let xor1 = *ptr - b'0';
    ptr = ptr.add(6);

    let mut offset = 0;
    let mut xor2 = std::ptr::null();
    let mut four = std::ptr::null();
    for _ in 0..4 {
        offset = 4 * offset + *LUT2.get_unchecked(*ptr as usize - b'0' as usize);
        if *ptr == b'1' {
            xor2 = ptr;
        }
        if *ptr == b'4' {
            four = ptr;
        }
        ptr = ptr.add(4);
    }

    let xor2 = *xor2.add(2) - b'0';
    let four = *four.add(2) - b'0';

    *LUT.get_unchecked(
        (8 * 8 * 8) * offset + 64 * xor1 as usize + 8 * xor2 as usize + four as usize,
    )
}
