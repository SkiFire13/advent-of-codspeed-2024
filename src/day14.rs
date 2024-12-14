#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]
#![feature(core_intrinsics)]

use std::mem::MaybeUninit;

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
    let mut counts = [[0; 3]; 3];
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

    #[repr(C, align(8))]
    #[derive(Clone, Copy)]
    struct Robot([Ty; 4]);

    let mut robots = [MaybeUninit::<Robot>::uninit(); 500];
    let mut ptr = input.as_ptr().wrapping_sub(1);
    let end = ptr.add(input.len());
    let mut robots_ptr = robots.as_mut_ptr().cast();

    loop {
        ptr = ptr.add(3);
        let px = parse_pos!(ptr as Ty);
        ptr = ptr.add(1);
        let py = parse_pos!(ptr as Ty);
        ptr = ptr.add(3);
        let vx = parse!(ptr as Ty - W);
        ptr = ptr.add(1);
        let vy = parse!(ptr as Ty - H);

        *robots_ptr = [px, vx, py, vy];
        robots_ptr = robots_ptr.add(1);

        if ptr == end {
            break;
        }
    }

    let mut i = 0;
    loop {
        i += 1;

        let mut xcounts = [0; W as usize];
        let mut ycounts = [0; H as usize];

        let mut robots_ptr = robots.as_mut_ptr().cast::<[Ty; 4]>();
        let robots_end = robots_ptr.add(500).cast();
        loop {
            let [px, vx, py, vy] = &mut *robots_ptr;

            *px = (*px + *vx) % (W as Ty);
            *xcounts.get_unchecked_mut(*px as usize) += 1;

            *py = (*py + *vy) % (H as Ty);
            *ycounts.get_unchecked_mut(*py as usize) += 1;

            robots_ptr = robots_ptr.add(1);
            if robots_ptr == robots_end {
                break;
            }
        }

        for &xc in &xcounts {
            if xc >= 20 {
                let mut j = i;

                loop {
                    for &yc in &ycounts {
                        if yc >= 20 {
                            return (51 * (i * H + j * W) % (W * H)) as u64;
                        }
                    }

                    j += 1;

                    ycounts = [0; H as usize];

                    let mut robots_ptr = robots.as_mut_ptr().cast::<[Ty; 4]>();
                    let robots_end = robots_ptr.add(500).cast();
                    loop {
                        let [_, _, py, vy] = &mut *robots_ptr;

                        *py = (*py + *vy) % (H as Ty);
                        *ycounts.get_unchecked_mut(*py as usize) += 1;

                        robots_ptr = robots_ptr.add(1);
                        if robots_ptr == robots_end {
                            break;
                        }
                    }
                }
            }
        }

        for &yc in &ycounts {
            if yc >= 20 {
                let j = i;

                loop {
                    for &xc in &xcounts {
                        if xc >= 20 {
                            return (51 * (i * H + j * W) % (W * H)) as u64;
                        }
                    }

                    i += 1;

                    xcounts = [0; W as usize];

                    let mut robots_ptr = robots.as_mut_ptr().cast::<[Ty; 4]>();
                    let robots_end = robots_ptr.add(500).cast();
                    loop {
                        let [px, vx, _, _] = &mut *robots_ptr;

                        *px = (*px + *vx) % (W as Ty);
                        *xcounts.get_unchecked_mut(*px as usize) += 1;

                        robots_ptr = robots_ptr.add(1);
                        if robots_ptr == robots_end {
                            break;
                        }
                    }
                }
            }
        }
    }
}
