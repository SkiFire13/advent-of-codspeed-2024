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

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u32 {
    type Ty = u32;

    let mut tot = 0;

    let lut = include_bytes!(concat!(env!("OUT_DIR"), "/d11p1.lut"))
        .as_ptr()
        .cast::<Ty>();

    #[rustfmt::skip]
    core::arch::asm!(
    "2:",
        "movzx   {n}, byte ptr [{ptr}]",
        "movzx   {d}, byte ptr [{ptr} + 1]",
        "sub     {n}, {b0}",
        "sub     {d}, {b0}",
        "add     {ptr}, 2",
        "cmp     {d}, 9",
        "ja      4f",
    "3:",
        "lea     {n}, [{n} + 4*{n}]",
        "lea     {n}, [{d} + 2*{n}]",
        "movzx   {d}, byte ptr [{ptr}]",
        "sub     {d}, {b0}",
        "inc     {ptr}",
        "cmp     {d}, 10",
        "jb      3b",
    "4:",
        "add     {tot:e}, dword ptr [{lut} + {s}*{n}]",
        "cmp     {ptr}, {end}",
        "jne     2b",
        ptr = in(reg) input.as_ptr(),
        end = in(reg) input.as_ptr().add(input.len()),
        lut = in(reg) lut,
        tot = inout(reg) tot,
        n = out(reg) _,
        d = out(reg) _,
        s = const std::mem::size_of::<Ty>(),
        b0 = const b'0' as u64
    );

    tot
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    type Ty = u64;

    let mut tot = 0;

    let lut = include_bytes!(concat!(env!("OUT_DIR"), "/d11p2.lut"))
        .as_ptr()
        .cast::<Ty>();

    #[rustfmt::skip]
    core::arch::asm!(
    "2:",
        "movzx   {n}, byte ptr [{ptr}]",
        "movzx   {d}, byte ptr [{ptr} + 1]",
        "sub     {n}, {b0}",
        "sub     {d}, {b0}",
        "add     {ptr}, 2",
        "cmp     {d}, 9",
        "ja      4f",
    "3:",
        "lea     {n}, [{n} + 4*{n}]",
        "lea     {n}, [{d} + 2*{n}]",
        "movzx   {d}, byte ptr [{ptr}]",
        "sub     {d}, {b0}",
        "inc     {ptr}",
        "cmp     {d}, 10",
        "jb      3b",
    "4:",
        "add     {tot}, qword ptr [{lut} + {s}*{n}]",
        "cmp     {ptr}, {end}",
        "jne     2b",
        ptr = in(reg) input.as_ptr(),
        end = in(reg) input.as_ptr().add(input.len()),
        lut = in(reg) lut,
        tot = inout(reg) tot,
        n = out(reg) _,
        d = out(reg) _,
        s = const std::mem::size_of::<Ty>(),
        b0 = const b'0' as u64
    );

    tot
}
