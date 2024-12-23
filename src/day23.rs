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

// pub fn run(input: &str) -> &'static str {
//     part2(input)
// }

#[inline(always)]
pub fn part1(input: &str) -> u64 {
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> &'static str {
    unsafe { inner_part2(input) }
}

static LUT1: [(u64, u64); 26] = {
    let mut lut = [(u64::MAX, u64::MAX); 26];

    let off = (26 * (b't' - b'a') as usize) % 64;

    let mut i = 0;
    while i < 26 {
        let mut j = 0;
        while j < i {
            if off + j < 64 {
                lut[i].0 &= !(1 << (off + j));
            } else {
                lut[i].1 &= !(1 << (off + j - 64));
            }

            j += 1;
        }

        i += 1;
    }

    lut
};

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    const L: usize = 11;
    let mut sets = [[0u64; L]; 26 * 26];

    let mut ptr = input.as_ptr();
    let end = ptr.add(input.len());
    loop {
        let b1 = *ptr.add(0) as usize - b'a' as usize;
        let b2 = *ptr.add(1) as usize - b'a' as usize;
        let b3 = *ptr.add(3) as usize - b'a' as usize;
        let b4 = *ptr.add(4) as usize - b'a' as usize;

        let n1 = 26 * b1 + b2;
        let n2 = 26 * b3 + b4;

        *sets.get_unchecked_mut(n1).get_unchecked_mut(n2 / 64) |= 1 << (n2 % 64);
        *sets.get_unchecked_mut(n2).get_unchecked_mut(n1 / 64) |= 1 << (n1 % 64);

        ptr = ptr.add(6);
        if ptr == end {
            break;
        }
    }

    let mut count = 0;
    for b2 in 0..26 {
        let i = 26 * (b't' - b'a') as usize + b2;
        let mut s = *sets.get_unchecked(i);
        let (m1, m2) = LUT1[b2];
        s[7] &= m1;
        s[8] &= m2;

        for si in 0..L {
            while s[si] != 0 {
                let j = 64 * si + s[si].trailing_zeros() as usize;
                s[si] &= !(1 << s[si].trailing_zeros());

                let s2 = *sets.get_unchecked(j);

                for sj in si..L {
                    count += (s[sj] & s2[sj]).count_ones() as u64;
                }
            }
        }
    }
    count
}

static mut PART2_OUT: [u8; 13 * 2 + 12] = [b','; 13 * 2 + 12];

#[allow(unused)]
#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> &'static str {
    let input = input.as_bytes();

    const L: usize = 11;
    let mut sets = [[0u64; L]; 26 * 26];

    let mut ptr = input.as_ptr();
    let end = ptr.add(input.len());
    loop {
        let b1 = *ptr.add(0) as usize - b'a' as usize;
        let b2 = *ptr.add(1) as usize - b'a' as usize;
        let b3 = *ptr.add(3) as usize - b'a' as usize;
        let b4 = *ptr.add(4) as usize - b'a' as usize;

        let n1 = 26 * b1 + b2;
        let n2 = 26 * b3 + b4;

        *sets.get_unchecked_mut(n1).get_unchecked_mut(n2 / 64) |= 1 << (n2 % 64);
        *sets.get_unchecked_mut(n2).get_unchecked_mut(n1 / 64) |= 1 << (n1 % 64);

        ptr = ptr.add(6);
        if ptr == end {
            break;
        }
    }

    for i in 0..26 * 26 {
        let s = *sets.get_unchecked(i);
        if s == [0; L] {
            continue;
        }

        macro_rules! handle {
            ($other:expr) => {
                'handle: {
                    let other = $other;

                    let s2 = *sets.get_unchecked(other);
                    let mut common = [0; L];
                    let mut count = 0;
                    for j in 0..L {
                        *common.get_unchecked_mut(j) = s.get_unchecked(j) & s2.get_unchecked(j);
                        count += common.get_unchecked_mut(j).count_ones();
                    }

                    if count != 11 {
                        break 'handle;
                    }

                    for i in 0..L {
                        let mut b = common[i];
                        while b != 0 {
                            let j = 64 * i + b.trailing_zeros() as usize;
                            b &= !(1 << b.trailing_zeros());

                            let s3 = *sets.get_unchecked(j);

                            let mut count = 0;
                            for k in 0..L {
                                count += (s.get_unchecked(k) & s3.get_unchecked(k)).count_ones();
                            }

                            if count != 11 {
                                break 'handle;
                            }
                        }
                    }

                    *common.get_unchecked_mut(i / 64) |= 1 << (i % 64);
                    *common.get_unchecked_mut(other / 64) |= 1 << (other % 64);
                    let mut pos = 0;
                    for i in 0..L {
                        let mut b = common[i];
                        while b != 0 {
                            let j = 64 * i + b.trailing_zeros() as usize;
                            b &= !(1 << b.trailing_zeros());

                            *PART2_OUT.get_unchecked_mut(pos) = b'a' + (j / 26) as u8;
                            *PART2_OUT.get_unchecked_mut(pos + 1) = b'a' + (j % 26) as u8;
                            pos += 3;
                        }
                    }
                    return std::str::from_utf8_unchecked(&PART2_OUT);
                }
            };
        }

        let mut j = 0;
        let mut b = *s.get_unchecked(j);

        while b == 0 {
            j += 1;
            b = *s.get_unchecked(j);
        }
        handle!(64 * j + b.trailing_zeros() as usize);

        b &= !(1 << b.trailing_zeros());

        while b == 0 {
            j += 1;
            b = *s.get_unchecked(j);
        }
        handle!(64 * j + b.trailing_zeros() as usize);
    }

    std::hint::unreachable_unchecked();
}
