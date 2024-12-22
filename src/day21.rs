#![allow(unused_attributes)]
#![allow(static_mut_refs)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]
#![feature(int_roundings)]

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

static LUT1: [u64; 1 << 24] =
    unsafe { std::mem::transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/d21p1.lut"))) };

static LUT2: [u64; 1 << 24] =
    unsafe { std::mem::transmute(*include_bytes!(concat!(env!("OUT_DIR"), "/d21p2.lut"))) };

#[inline(always)]
unsafe fn solve(input: &[u8], lut: &[u64; 1 << 24]) -> u64 {
    let mut tot;
    core::arch::asm!(
        "mov    {r1}, qword ptr[{input}]",
        "mov    {r2}, qword ptr[{input} + 10]",
        "movabs {rmask}, {cmask}",
        "pext   {r1}, {r1}, {rmask}",
        "pext   {r2}, {r2}, {rmask}",
        "mov    {r1}, qword ptr [{lut} + 8*{r1}]",
        "add    {r1}, qword ptr [{lut} + 8*{r2}]",
        "mov    {r2:e}, dword ptr [{input} + 20]",
        "pext   {r2:e}, {r2:e}, {rmask:e}",
        "add    {r1}, qword ptr [{lut} + 8*{r2}]",
        input = in(reg) input.as_ptr(),
        lut = in(reg) lut,
        r1 = out(reg) tot,
        r2 = out(reg) _,
        rmask = out(reg) _,
        cmask = const u64::from_ne_bytes([0b1111, 0b1111, 0b1111, 0, 0, 0b1111, 0b1111, 0b1111]),
        options(nostack, pure, readonly)
    );
    tot
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    solve(input.as_bytes(), &LUT1)
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    solve(input.as_bytes(), &LUT2)
}
