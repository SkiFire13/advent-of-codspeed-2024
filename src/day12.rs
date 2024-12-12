#![allow(unused_attributes)]
#![feature(portable_simd)]
#![feature(avx512_target_feature)]
#![feature(slice_ptr_get)]
#![feature(array_ptr_get)]

use std::mem::MaybeUninit;
use std::simd::prelude::*;

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

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    #[cfg(debug_assertions)]
    let real_input = input;

    let mut edges = const {
        let mut edges = [0; 141 + 141 * 141 + 32];

        let mut i = 0;
        while i < 141 {
            edges[141 + 0 * 141 + i] = 1;
            i += 1;
        }

        edges[141 + 0] += 1;

        edges
    };

    let mut array = [MaybeUninit::<u8>::uninit(); 141 + 141 * 141 + 32];
    array.get_unchecked_mut(..141).fill(MaybeUninit::new(b'\n'));
    std::ptr::copy(
        input.as_ptr(),
        array.as_mut_ptr().add(141).cast(),
        140 * 141,
    );
    array
        .get_unchecked_mut(141 + 140 * 141..)
        .fill(MaybeUninit::new(b'\n'));
    let input = &mut *((&raw mut array) as *mut [u8; 141 + 141 * 141 + 32]);

    // TODO: bitmap to which neighbours are equal -> switch table

    let mut off = 141;
    while off < 141 + 140 * 141 {
        let o1 = off;
        let o2 = off + 1;
        let o3 = off + 141;

        let b1 = u8x32::from_slice(input.get_unchecked(o1..o1 + 32));
        let b2 = u8x32::from_slice(input.get_unchecked(o2..o2 + 32));
        let b3 = u8x32::from_slice(input.get_unchecked(o3..o3 + 32));

        let t = b1.simd_ne(b2);
        let l = b1.simd_ne(b3);

        let mut s1 = i8x32::from_slice(edges.get_unchecked(o1..o1 + 32));
        s1 += t.to_int() & i8x32::splat(1);
        s1 += l.to_int() & i8x32::splat(1);
        *edges.get_unchecked_mut(o1) = s1[0];

        let mut s2 = s1.rotate_elements_left::<1>();
        s2[31] = *edges.get_unchecked(o2 + 32 - 1);
        s2 += t.to_int() & i8x32::splat(1);
        s2.copy_to_slice(edges.get_unchecked_mut(o2..o2 + 32));

        let mut s3 = i8x32::from_slice(edges.get_unchecked(o3..o3 + 32));
        s3 += l.to_int() & i8x32::splat(1);
        s3.copy_to_slice(edges.get_unchecked_mut(o3..o3 + 32));

        off += 32;
    }

    #[cfg(debug_assertions)]
    {
        let mut expected = [0; 140 * 141];
        for y in 0..140 {
            for x in 0..140 {
                let c = real_input[141 * y + x];
                let mut n = 0;
                if x == 0 || real_input[141 * y + (x - 1)] != c {
                    n += 1;
                }
                if x == 140 || real_input[141 * y + (x + 1)] != c {
                    n += 1;
                }
                if y == 0 || real_input[141 * (y - 1) + x] != c {
                    n += 1;
                }
                if y == 139 || real_input[141 * (y + 1) + x] != c {
                    n += 1;
                }
                expected[141 * y + x] = n;
            }
        }

        for y in 0..140 {
            for x in 0..140 {
                assert_eq!(
                    edges[141 + 141 * y + x],
                    expected[141 * y + x],
                    "x={x} y={y}"
                );
            }
        }
    }

    let mut tot = 0;
    let mut stack = [MaybeUninit::uninit(); 128];
    let mut stack_len = 0;
    let mut off = 141;

    while off < 141 + 140 * 141 {
        let Some(idx) = u8x32::from_slice(input.get_unchecked(off..off + 32))
            .simd_ne(u8x32::splat(b'\n'))
            .first_set()
        else {
            off += 32;
            continue;
        };
        off += idx;

        let c = *input.get_unchecked(off);
        *input.get_unchecked_mut(off) = b'\n';

        let mut curr_off = off;
        let mut area = 0;
        let mut perimeter = 0;

        loop {
            area += 1;
            perimeter += *edges.get_unchecked(curr_off) as u8 as u64;

            macro_rules! handle {
                ($off:expr) => {{
                    let new_off = $off;
                    if *input.get_unchecked(new_off) == c {
                        *input.get_unchecked_mut(new_off) = b'\n';
                        *stack.get_unchecked_mut(stack_len).as_mut_ptr() = new_off;
                        stack_len += 1;
                    }
                }};
            }

            handle!(curr_off + 1);
            handle!(curr_off + 141);
            handle!(curr_off - 141);
            let new_off = curr_off - 1;
            if *input.get_unchecked(new_off) == c {
                *input.get_unchecked_mut(new_off) = b'\n';
                curr_off = new_off;
                continue;
            }

            if stack_len == 0 {
                break;
            }
            stack_len -= 1;
            curr_off = *stack.get_unchecked(stack_len).as_ptr();
        }

        tot += area * perimeter;

        off += 1;
    }

    tot
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let input = input.as_bytes();

    #[cfg(debug_assertions)]
    let real_input = input;

    let mut corners = [0; 141 + 141 * 141 + 32];
    corners[141 + 0] += 1;

    let mut array = [MaybeUninit::<u8>::uninit(); 141 + 141 * 141 + 32];
    array.get_unchecked_mut(..141).fill(MaybeUninit::new(b'\n'));
    std::ptr::copy(
        input.as_ptr(),
        array.as_mut_ptr().add(141).cast(),
        140 * 141,
    );
    array
        .get_unchecked_mut(141 + 140 * 141..)
        .fill(MaybeUninit::new(b'\n'));
    let input = &mut *((&raw mut array) as *mut [u8; 141 + 141 * 141 + 32]);

    // TODO: bitmap to which neighbours are equal -> switch table

    let mut off = 0;
    while off < 141 + 140 * 141 {
        let o1 = off;
        let o2 = off + 1;
        let o3 = off + 141;
        let o4 = off + 1 + 141;

        let b1 = u8x32::from_slice(input.get_unchecked(o1..o1 + 32));
        let b2 = u8x32::from_slice(input.get_unchecked(o2..o2 + 32));
        let b3 = u8x32::from_slice(input.get_unchecked(o3..o3 + 32));
        let b4 = u8x32::from_slice(input.get_unchecked(o4..o4 + 32));

        let t = b1.simd_ne(b2);
        let b = b3.simd_ne(b4);
        let l = b1.simd_ne(b3);
        let r = b2.simd_ne(b4);
        let d1 = b1.simd_ne(b4);
        let d2 = b2.simd_ne(b3);

        let mut s1 = i8x32::from_slice(corners.get_unchecked(o1..o1 + 32));
        s1 += (t.simd_eq(l) & (d1 | t)).to_int() & i8x32::splat(1);
        *corners.get_unchecked_mut(o1) = s1[0];

        let mut s2 = s1.rotate_elements_left::<1>();
        s2[31] = *corners.get_unchecked(o2 + 32 - 1);
        s2 += (t.simd_eq(r) & (d2 | t)).to_int() & i8x32::splat(1);
        s2.copy_to_slice(corners.get_unchecked_mut(o2..o2 + 32));

        let mut s3 = i8x32::from_slice(corners.get_unchecked(o3..o3 + 32));
        s3 += (b.simd_eq(l) & (d2 | b)).to_int() & i8x32::splat(1);
        *corners.get_unchecked_mut(o3) = s3[0];

        let mut s4 = s3.rotate_elements_left::<1>();
        s4[31] = *corners.get_unchecked(o4 + 32 - 1);
        s4 += (b.simd_eq(r) & (d1 | b)).to_int() & i8x32::splat(1);
        s4.copy_to_slice(corners.get_unchecked_mut(o4..o4 + 32));

        off += 32;
    }

    #[cfg(debug_assertions)]
    {
        let mut expected = [0; 140 * 141];
        for y in 0..140 {
            for x in 0..140 {
                let c = real_input[141 * y + x];
                let mut n = 0;

                let t = (y != 0).then(|| real_input[141 * (y - 1) + x]);
                let b = (y != 139).then(|| real_input[141 * (y + 1) + x]);
                let l = (x != 0).then(|| real_input[141 * y + (x - 1)]);
                let r = (x != 139).then(|| real_input[141 * y + (x + 1)]);

                let tl = (y != 0 && x != 0).then(|| real_input[141 * (y - 1) + (x - 1)]);
                let tr = (y != 0 && x != 139).then(|| real_input[141 * (y - 1) + (x + 1)]);
                let bl = (y != 139 && x != 0).then(|| real_input[141 * (y + 1) + (x - 1)]);
                let br = (y != 139 && x != 139).then(|| real_input[141 * (y + 1) + (x + 1)]);

                n += (t != Some(c) && l != Some(c)) as i8;
                n += (t != Some(c) && r != Some(c)) as i8;
                n += (b != Some(c) && l != Some(c)) as i8;
                n += (b != Some(c) && r != Some(c)) as i8;

                n += (t == Some(c) && l == Some(c) && tl != Some(c)) as i8;
                n += (t == Some(c) && r == Some(c) && tr != Some(c)) as i8;
                n += (b == Some(c) && l == Some(c) && bl != Some(c)) as i8;
                n += (b == Some(c) && r == Some(c) && br != Some(c)) as i8;

                expected[141 * y + x] = n;
            }
        }

        for y in 0..140 {
            for x in 0..140 {
                assert_eq!(
                    corners[141 + 141 * y + x],
                    expected[141 * y + x],
                    "got={}, expected={}, x={x}, y={y}",
                    corners[141 + 141 * y + x],
                    expected[141 * y + x],
                );
            }
        }
    }

    let mut tot = 0;
    let mut stack = [MaybeUninit::uninit(); 128];
    let mut stack_len = 0;
    let mut off = 141;

    while off < 141 + 140 * 141 {
        let Some(idx) = u8x32::from_slice(input.get_unchecked(off..off + 32))
            .simd_ne(u8x32::splat(b'\n'))
            .first_set()
        else {
            off += 32;
            continue;
        };
        off += idx;

        let c = *input.get_unchecked(off);
        *input.get_unchecked_mut(off) = b'\n';

        let mut curr_off = off;
        let mut area = 0;
        let mut perimeter = 0;

        loop {
            area += 1;
            perimeter += *corners.get_unchecked(curr_off) as u8 as u64;

            macro_rules! handle {
                ($off:expr) => {{
                    let new_off = $off;
                    if *input.get_unchecked(new_off) == c {
                        *input.get_unchecked_mut(new_off) = b'\n';
                        *stack.get_unchecked_mut(stack_len).as_mut_ptr() = new_off;
                        stack_len += 1;
                    }
                }};
            }

            handle!(curr_off + 1);
            handle!(curr_off + 141);
            handle!(curr_off - 141);
            let new_off = curr_off - 1;
            if *input.get_unchecked(new_off) == c {
                *input.get_unchecked_mut(new_off) = b'\n';
                curr_off = new_off;
                continue;
            }

            if stack_len == 0 {
                break;
            }
            stack_len -= 1;
            curr_off = *stack.get_unchecked(stack_len).as_ptr();
        }

        tot += area * perimeter;

        off += 1;
    }

    tot
}
