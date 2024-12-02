#[inline(always)]
fn parse8(n: u64) -> u32 {
    use std::num::Wrapping as W;

    let mut n = W(n);
    let mask = W(0xFF | (0xFF << 32));
    let mul1 = W(100 + (1000000 << 32));
    let mul2 = W(1 + (10000 << 32));

    n -= W(u64::from_ne_bytes([b'0'; 8]));
    n = (n * W(10)) + (n >> 8);
    n = (((n & mask) * mul1) + (((n >> 16) & mask) * mul2)) >> 32;

    n.0 as u32
}

#[inline(always)]
fn parse5top(n: u64) -> u32 {
    parse8((n << 24) | 0x0000000000303030)
}

#[inline(always)]
fn parse5bottom(n: u64) -> u32 {
    parse8((n & 0xFFFFFFFFFF000000) | 0x0000000000303030)
}

#[inline(always)]
fn read_u64(s: &[u8]) -> u64 {
    u64::from_ne_bytes(s.try_into().unwrap())
}

pub fn run(input: &str) -> i64 {
    part1(input) as i64
}

pub fn part1(input: &str) -> u32 {
    let mut left = [0; 1000];
    let mut right = [0; 1000];

    for (i, line) in input.as_bytes().chunks_exact(14).enumerate() {
        let l = parse5top(read_u64(&line[..8]));
        let r = parse5bottom(read_u64(&line[5..13]));

        left[i] = l;
        right[i] = r;
    }

    left.sort_unstable();
    right.sort_unstable();

    std::iter::zip(&left, &right)
        .map(|(&l, &r)| u32::abs_diff(l, r))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut counts = [0u16; 100_000];
    let mut tot = 0;

    for line in input.as_bytes().chunks_exact(14) {
        let l = parse5top(read_u64(&line[..8]));
        if counts[l as usize] != u16::MAX {
            tot += l * counts[l as usize] as u32;
            counts[l as usize] = u16::MAX;
        }

        let r = parse5bottom(read_u64(&line[5..13]));
        if counts[r as usize] == u16::MAX {
            tot += r;
        } else {
            counts[r as usize] += 1;
        }
    }
    tot
}
