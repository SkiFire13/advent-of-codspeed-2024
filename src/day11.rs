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

#[allow(long_running_const_eval)]
const fn solve_rec(i: usize, j: usize, lut: &[[u64; 1000]; 76]) -> u64 {
    if i == 0 {
        return 1;
    } else if j < 1000 {
        return lut[i][j];
    } else if j == 0 {
        return lut[i - 1][1];
    } else if j.ilog10() % 2 == 1 {
        let pow10 = 10usize.pow((j.ilog10() + 1) / 2);
        return solve_rec(i - 1, j / pow10, lut) + solve_rec(i - 1, j % pow10, lut);
    } else {
        return solve_rec(i - 1, j * 2024, lut);
    }
}

#[allow(long_running_const_eval)]
static LUT: [[u64; 1000]; 76] = {
    let mut lut = [[0; 1000]; 76];

    let mut j = 0;
    while j < 1000 {
        lut[0][j] = 1;
        j += 1;
    }

    let mut i = 1;
    while i < 76 {
        let mut j = 0;
        while j < 1000 {
            if j == 0 {
                lut[i][j] = lut[i - 1][1];
            } else if j.ilog10() % 2 == 1 {
                let pow10 = 10usize.pow((j.ilog10() + 1) / 2);
                lut[i][j] = lut[i - 1][j / pow10] + lut[i - 1][j % pow10];
            } else {
                lut[i][j] = solve_rec(i - 1, j * 2024, &lut);
            }

            j += 1;
        }

        i += 1;
    }

    lut
};

unsafe fn solve(input: &[u8], iters: usize) -> u64 {
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

        tot += solve_rec(iters, n, &LUT);
    }

    tot
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    solve(input.as_bytes(), 25)
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    solve(input.as_bytes(), 75)
}
