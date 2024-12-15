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

        let fx = fastdiv::fastmod_w((px + 100 * vx) as _) as Ty;
        let fy = fastdiv::fastmod_h((py + 100 * vy) as _) as Ty;

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
    type Ty = u8;

    #[repr(C, align(32))]
    struct Aligned<T>(T);

    let mut robots_x = Aligned([MaybeUninit::<Ty>::uninit(); 512]);
    let mut robots_y = Aligned([MaybeUninit::<Ty>::uninit(); 512]);
    let mut robots_vx = Aligned([MaybeUninit::<Ty>::uninit(); 512]);
    let mut robots_vy = Aligned([MaybeUninit::<Ty>::uninit(); 512]);
    let mut offset = 0;

    let mut ptr = input.as_ptr().wrapping_sub(1);
    let end = ptr.add(input.len());

    loop {
        ptr = ptr.add(3);
        let px = parse_pos!(ptr as Ty);
        *robots_x.0.get_unchecked_mut(offset).as_mut_ptr() = px;

        ptr = ptr.add(1);
        let py = parse_pos!(ptr as Ty);
        *robots_y.0.get_unchecked_mut(offset).as_mut_ptr() = py;

        ptr = ptr.add(3);
        let vx = parse!(ptr as Ty - W);
        *robots_vx.0.get_unchecked_mut(offset).as_mut_ptr() = vx;

        ptr = ptr.add(1);
        let vy = parse!(ptr as Ty - H);
        *robots_vy.0.get_unchecked_mut(offset).as_mut_ptr() = vy;

        offset += 1;

        if ptr == end {
            break;
        }
    }

    robots_x.0[500..].fill(MaybeUninit::new(W as Ty / 2));
    robots_vx.0[500..].fill(MaybeUninit::zeroed());
    robots_y.0[500..].fill(MaybeUninit::new(H as Ty / 2));
    robots_vy.0[500..].fill(MaybeUninit::zeroed());

    macro_rules! run_iter_reg {
        ($p:expr, $v:ident, $acc:ident, $o:literal | $s:ident) => {{
            core::arch::asm!(
                "vpaddb  {np}, {np}, ymmword ptr [{v_ptr} + {off}]",
                "vpaddb  {s}, {np}, {c1}",
                "vpminub {np}, {np}, {s}",
                "vpsadbw {s}, {np}, {c2}",
                "vpaddq  {acc}, {acc}, {s}",
                v_ptr = in(reg) $v.0.as_ptr(),
                off = const 32 * $o,
                np = inout(ymm_reg) $p,
                acc = inout(ymm_reg) $acc,
                s = out(ymm_reg) _,
                c1 = in(ymm_reg) u8x32::splat((256 - $s) as Ty),
                c2 = in(ymm_reg) u8x32::splat($s as u8 / 2),
                options(nostack, pure, readonly),
            );
        }};
    }

    macro_rules! run_iter_mem {
        ($p:expr, $v:ident, $acc:ident, $o:literal | $s:ident) => {{
            core::arch::asm!(
                "vmovdqa {np}, ymmword ptr [{p_ptr} + {off}]",
                "vpaddb  {np}, {np}, ymmword ptr [{v_ptr} + {off}]",
                "vpaddb  {s}, {np}, {c1}",
                "vpminub {np}, {np}, {s}",
                "vpsadbw {s}, {np}, {c2}",
                "vpaddq  {acc}, {acc}, {s}",
                "vmovdqa ymmword ptr [{p_ptr} + {off}], {np}",
                p_ptr = in(reg) $p.0.as_ptr(),
                v_ptr = in(reg) $v.0.as_ptr(),
                off = const 32 * $o,
                np = out(ymm_reg) _,
                acc = inout(ymm_reg) $acc,
                s = out(ymm_reg) _,
                c1 = in(ymm_reg) u8x32::splat((256 - $s) as Ty),
                c2 = in(ymm_reg) u8x32::splat($s as u8 / 2),
                options(nostack, pure, readonly),
            );
        }};
    }

    macro_rules! run_loop {
        ($p:ident, $v:ident | $s:ident) => {{
            let mut p00 = *$p.0.as_ptr().cast::<u8x32>().add(00);
            let mut p01 = *$p.0.as_ptr().cast::<u8x32>().add(01);
            let mut p02 = *$p.0.as_ptr().cast::<u8x32>().add(02);
            let mut p03 = *$p.0.as_ptr().cast::<u8x32>().add(03);
            let mut p04 = *$p.0.as_ptr().cast::<u8x32>().add(04);
            let mut p05 = *$p.0.as_ptr().cast::<u8x32>().add(05);
            let mut p06 = *$p.0.as_ptr().cast::<u8x32>().add(06);
            let mut p07 = *$p.0.as_ptr().cast::<u8x32>().add(07);
            let mut p08 = *$p.0.as_ptr().cast::<u8x32>().add(08);
            let mut p09 = *$p.0.as_ptr().cast::<u8x32>().add(09);
            let mut p10 = *$p.0.as_ptr().cast::<u8x32>().add(10);
            // let mut p11 = *$p.0.as_ptr().cast::<u8x32>().add(11);
            // let mut p12 = *$p.0.as_ptr().cast::<u8x32>().add(12);
            // let mut p13 = *$p.0.as_ptr().cast::<u8x32>().add(13);
            // let mut p14 = *$p.0.as_ptr().cast::<u8x32>().add(14);
            // let mut p15 = *$p.0.as_ptr().cast::<u8x32>().add(15);

            let mut i = 0;
            loop {
                i += 1;

                let mut acc = u64x4::splat(0);
                run_iter_reg!(p00, $v, acc, 00 | $s);
                run_iter_reg!(p01, $v, acc, 01 | $s);
                run_iter_reg!(p02, $v, acc, 02 | $s);
                run_iter_reg!(p03, $v, acc, 03 | $s);
                run_iter_reg!(p04, $v, acc, 04 | $s);
                run_iter_reg!(p05, $v, acc, 05 | $s);
                run_iter_reg!(p06, $v, acc, 06 | $s);
                run_iter_reg!(p07, $v, acc, 07 | $s);
                run_iter_reg!(p08, $v, acc, 08 | $s);
                run_iter_reg!(p09, $v, acc, 09 | $s);
                run_iter_reg!(p10, $v, acc, 10 | $s);
                run_iter_mem!($p, $v, acc, 11 | $s);
                run_iter_mem!($p, $v, acc, 12 | $s);
                run_iter_mem!($p, $v, acc, 13 | $s);
                run_iter_mem!($p, $v, acc, 14 | $s);
                run_iter_mem!($p, $v, acc, 15 | $s);

                let sum = acc.reduce_sum();

                if sum.abs_diff(500 * $s as u64 / 4) >= 1500 {
                    break i;
                }
            }
        }};
    }

    let mut i = i64::MAX;
    let j;
    let mut p = &mut robots_x;
    let mut v = &robots_vx;
    let mut c = W;

    loop {
        let n = run_loop!(p, v | c);
        if i == i64::MAX {
            i = n;
            p = &mut robots_y;
            v = &robots_vy;
            c = H;
        } else {
            j = n;
            break;
        }
    }

    (51 * (i * H + j * W) % (W * H)) as u64
}

mod fastdiv {
    #[inline(always)]
    const fn compute_m_u16(d: u16) -> u32 {
        (u32::MAX / d as u32) + 1
    }

    #[inline(always)]
    const fn mul64_u16(lowbits: u32, d: u16) -> u32 {
        (lowbits as u64 * d as u64 >> 32) as u32
    }

    #[inline(always)]
    const fn fastmod_u16(a: u16, m: u32, d: u16) -> u16 {
        let lowbits = m.wrapping_mul(a as u32);
        mul64_u16(lowbits, d) as u16
    }

    #[inline(always)]
    pub fn fastmod_w(a: u16) -> u16 {
        use super::W as D;
        const M: u32 = compute_m_u16(D as _);
        fastmod_u16(a, M, D as _)
    }

    #[inline(always)]
    pub fn fastmod_h(a: u16) -> u16 {
        use super::H as D;
        const M: u32 = compute_m_u16(D as _);
        fastmod_u16(a, M, D as _)
    }
}
