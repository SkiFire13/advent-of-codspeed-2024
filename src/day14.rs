#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

use std::mem::MaybeUninit;
use std::simd::prelude::*;

pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

#[inline(always)]
pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) as u64 }
}

#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

macro_rules! parse_pos {
    ($ptr:ident as $ty:ty) => {{
        let mut n = *$ptr as $ty - b'0' as $ty;
        $ptr = $ptr.add(1);
        if *$ptr as $ty >= b'0' as $ty {
            n = 10 * n + *$ptr as $ty - b'0' as $ty;
            $ptr = $ptr.add(1);
            if *$ptr as $ty >= b'0' as $ty {
                n = 10 * n + *$ptr as $ty - b'0' as $ty;
                $ptr = $ptr.add(1);
            }
        }
        n
    }};
}

macro_rules! parse {
    ($ptr:ident as $ty:ident - $m:expr) => {{
        if *$ptr == b'-' {
            $ptr = $ptr.add(1);
            $m as $ty - parse_pos!($ptr as $ty)
        } else {
            parse_pos!($ptr as $ty)
        }
    }};
}

const W: i64 = 101;
const H: i64 = 103;

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let mut counts = [[0; 2]; 2];
    let mut ptr = input.as_ptr().wrapping_sub(1);
    let end = ptr.add(input.len());

    type Ty = u32;

    loop {
        ptr = ptr.add(3);
        let px = parse_pos!(ptr as Ty);
        ptr = ptr.add(1);
        let py = parse_pos!(ptr as Ty);
        ptr = ptr.add(3);
        let vx = parse!(ptr as Ty - W);
        ptr = ptr.add(1);
        let vy = parse!(ptr as Ty - H);

        let fx = (px + 100 * vx) % (W as Ty);
        let fy = (py + 100 * vy) % (H as Ty);

        if fx != W as Ty / 2 && fy != H as Ty / 2 {
            counts[(fx < W as Ty / 2) as usize][(fy < H as Ty / 2) as usize] += 1;
        }

        if ptr == end {
            break;
        }
    }

    counts[0][0] * counts[0][1] * counts[1][0] * counts[1][1]
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    type Ty = u16;

    #[repr(C, align(4))]
    #[derive(Clone, Copy)]
    struct RobotZ([Ty; 2]);

    let mut robots_x = [MaybeUninit::<RobotZ>::uninit(); 500];
    let mut robots_y = [MaybeUninit::<RobotZ>::uninit(); 500];
    let mut robots_x_ptr = robots_x.as_mut_ptr().cast();
    let mut robots_y_ptr = robots_y.as_mut_ptr().cast();

    let mut ptr = input.as_ptr().wrapping_sub(1);
    let end = ptr.add(input.len());

    loop {
        ptr = ptr.add(3);
        let px = parse_pos!(ptr as Ty);
        ptr = ptr.add(1);
        let py = parse_pos!(ptr as Ty);
        ptr = ptr.add(3);
        let vx = parse!(ptr as Ty - W);
        ptr = ptr.add(1);
        let vy = parse!(ptr as Ty - H);

        *robots_x_ptr = [px, vx];
        robots_x_ptr = robots_x_ptr.add(1);

        *robots_y_ptr = [py, vy];
        robots_y_ptr = robots_y_ptr.add(1);

        if ptr == end {
            break;
        }
    }

    #[inline(always)]
    unsafe fn check20(a: &[u8; 128]) -> bool {
        let m = u8x32::splat(20);
        let b1 = u8x32::from_slice(a.get_unchecked(..32)).simd_gt(m);
        let b2 = u8x32::from_slice(a.get_unchecked(32..64)).simd_gt(m);
        let b3 = u8x32::from_slice(a.get_unchecked(64..96)).simd_gt(m);
        let b4 = u8x32::from_slice(a.get_unchecked(96..128)).simd_gt(m);
        (b1 | b2 | b3 | b4).any()
    }

    let mut i = 0;
    loop {
        i += 1;

        let mut counts = [0u8; W.next_multiple_of(64) as usize];

        let mut robots_ptr = robots_x.as_mut_ptr().cast::<[Ty; 2]>();
        let robots_end = robots_ptr.add(500);
        loop {
            let [p, v] = &mut *robots_ptr;

            *p = (*p + *v) % (W as Ty);
            *counts.get_unchecked_mut(*p as usize) += 1;

            robots_ptr = robots_ptr.add(1);
            if robots_ptr == robots_end {
                break;
            }
        }

        if check20(&counts) {
            break;
        }
    }

    let mut j = 0;
    loop {
        j += 1;

        let mut counts = [0; H.next_multiple_of(64) as usize];

        let mut robots_ptr = robots_y.as_mut_ptr().cast::<[Ty; 2]>();
        let robots_end = robots_ptr.add(500).cast();
        loop {
            let [p, v] = &mut *robots_ptr;

            *p = (*p + *v) % (H as Ty);
            *counts.get_unchecked_mut(*p as usize) += 1;

            robots_ptr = robots_ptr.add(1);
            if robots_ptr == robots_end {
                break;
            }
        }

        if check20(&counts) {
            break;
        }
    }

    (51 * (i * H + j * W) % (W * H)) as u64
}
