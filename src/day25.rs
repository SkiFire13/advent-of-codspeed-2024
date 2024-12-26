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

    #[repr(C, align(64))]
    struct Data {
        list0: [u32x8; 256 / 8],
        list1: [u32x8; 256 / 8],
    }

    static mut DATA: Data = Data {
        list0: [u32x8::from_array([u32::MAX; 8]); 256 / 8],
        list1: [u32x8::from_array([u32::MAX; 8]); 256 / 8],
    };
    let data = &mut DATA;
    let mut len0 = 0;
    let mut len1 = 0;

    let mut ptr = input.as_ptr().add(4);
    let end = ptr.add(43 * 500);
    loop {
        let b = ptr.cast::<u8x32>().read_unaligned();
        let m = b.simd_eq(u8x32::splat(b'#')).to_bitmask() as u32;

        if *ptr == b'#' {
            *data.list0.as_mut_ptr().cast::<u32>().add(len0) = m;
            len0 += 1;
        } else {
            *data.list1.as_mut_ptr().cast::<u32>().add(len1) = m;
            len1 += 1;
        }

        ptr = ptr.add(43);
        if ptr == end {
            break;
        }
    }

    let mut count = i32x8::splat(0);
    for i in 0..250 {
        let m = *data.list0.as_ptr().cast::<u32>().add(i);
        core::arch::asm!(
            "vpand    {x}, {m}, ymmword ptr [{l} + 0]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 32]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 64]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 96]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 128]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 160]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 192]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 224]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 256]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 288]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 320]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 352]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 384]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 416]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 448]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 480]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 512]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 544]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 576]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 608]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 640]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 672]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 704]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 736]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 768]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 800]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 832]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 864]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 896]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 928]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 960]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            "vpand    {x}, {m}, ymmword ptr [{l} + 992]",
            "vpcmpeqd {x}, {x}, {z}",
            "vpsubd   {c}, {c}, {x}",
            l = in(reg) data.list1.as_ptr(),
            m = in(ymm_reg) u32x8::splat(m),
            z = in(ymm_reg) u32x8::splat(0),
            c = inout(ymm_reg) count,
            x = out(ymm_reg) _,
            options(nostack, pure, readonly),
        )
    }

    count.reduce_sum() as u64
}
