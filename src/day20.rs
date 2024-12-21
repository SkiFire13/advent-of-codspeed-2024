#![allow(unused_attributes)]
#![allow(static_mut_refs)]
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
    unsafe { inner_part1(input) }
}

#[inline(always)]
pub fn part2(input: &str) -> u64 {
    unsafe { inner_part2(input) }
}

const LEFT: usize = -1isize as usize;
const RIGHT: usize = 1isize as usize;
const UP: usize = -142isize as usize;
const DOWN: usize = 142isize as usize;

#[inline(always)]
unsafe fn find_start_end(input: &[u8]) -> (usize, usize) {
    let mut offset = 0;
    let p1 = loop {
        let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
        let mask = block.simd_ge(u8x64::splat(b'E')).to_bitmask();
        if mask != 0 {
            break offset + mask.trailing_zeros() as usize;
        }
        offset += 64;
    };

    let b = (b'E' + b'S') - *input.get_unchecked(p1);

    offset = p1 + 1;
    let p2 = loop {
        let block = u8x64::from_slice(input.get_unchecked(offset..offset + 64));
        let mask = block.simd_eq(u8x64::splat(b)).to_bitmask();
        if mask != 0 {
            break offset + mask.trailing_zeros() as usize;
        }
        offset += 64;
    };

    let (s, e) = if b == b'S' { (p1, p2) } else { (p2, p1) };

    (s, e)
}

#[inline(always)]
unsafe fn next(input: &[u8], prev: usize, curr: usize) -> usize {
    let mut next = curr.wrapping_add(LEFT);
    for d in [RIGHT, UP, DOWN] {
        let cand = curr.wrapping_add(d);
        if *input.get_unchecked(cand) != b'#' && cand != prev {
            next = curr.wrapping_add(d);
        }
    }
    next
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part1(input: &str) -> u64 {
    let input = input.as_bytes();

    let (s, e) = find_start_end(input);

    let mut count = 0;
    let mut prev = 0;
    let mut curr = s;
    let mut n = u16::MAX - 100;

    let mut seen = [0; 142 * 143];

    loop {
        *seen.get_unchecked_mut(curr + 142) = n;

        let next = next(input, prev, curr);
        prev = curr;
        curr = next;
        n -= 1;

        for d in [LEFT, RIGHT, UP, DOWN] {
            if *seen.get_unchecked((curr + 142).wrapping_add(d).wrapping_add(d)) > n + 100 {
                count += 1;
            }
        }

        if curr == e {
            break;
        }
    }

    count
}

#[target_feature(enable = "popcnt,avx2,ssse3,bmi1,bmi2,lzcnt")]
#[cfg_attr(avx512_available, target_feature(enable = "avx512vl"))]
unsafe fn inner_part2(input: &str) -> u64 {
    let input = input.as_bytes();

    let (s, e) = find_start_end(input);

    #[inline(always)]
    unsafe fn next(
        input: &[u8],
        prev: usize,
        cx: usize,
        cy: usize,
        curr: usize,
    ) -> (usize, usize, usize) {
        let (mut nx, mut ny, mut nc) = (cx - 1, cy, curr.wrapping_add(LEFT));
        for (dx, dy, d) in [(1, 0, RIGHT), (0, -1, UP), (0, 1, DOWN)] {
            let cand = curr.wrapping_add(d);
            if *input.get_unchecked(cand) != b'#' && cand != prev {
                nx = cx.wrapping_add_signed(dx);
                ny = cy.wrapping_add_signed(dy);
                nc = cand;
            }
        }
        (nx, ny, nc)
    }

    let mut count = 0;
    let mut prev = 0;
    let (mut cx, mut cy, mut curr) = (s % 142, s / 142, s);
    let mut cost = 0;

    let mut queue = [MaybeUninit::uninit(); 128];
    let mut queue_head = 0;
    let mut queue_tail = 0;

    for _ in 0..100 {
        *queue.get_unchecked_mut(queue_head).as_mut_ptr() = (cx, cy, cost);
        queue_head += 1;

        (prev, (cx, cy, curr)) = (curr, next(input, prev, cx, cy, curr));
        cost += 1;
    }

    for i in 2..=20 {
        *queue.get_unchecked_mut(queue_head).as_mut_ptr() = (cx, cy, cost);
        queue_head += 1;

        (prev, (cx, cy, curr)) = (curr, next(input, prev, cx, cy, curr));
        cost += 1;

        let (cx, cy) = (curr % 142, curr / 142);
        for j in queue_tail..queue_tail + i {
            let (px, py, pcost) = *queue.get_unchecked(j & 127).as_ptr();
            if pcost + cx.abs_diff(px) + cy.abs_diff(py) + 100 <= cost {
                count += 1;
            }
        }
    }

    const MASK: u16 = 1 << 15;
    const EMPTY: u16 = MASK | (142 * 142);

    let mut qtree = [MaybeUninit::<[u16; 5]>::uninit(); 8192];
    let mut qtree_len = 0;
    let mut qtree_root = EMPTY;

    #[cfg(debug_assertions)]
    use std::collections::HashSet;

    #[cfg(debug_assertions)]
    let mut set = HashSet::new();

    macro_rules! xy_to_wz {
        ($x:expr, $y:expr) => {
            (20 + 142 + $x - $y, 20 + $x + $y)
        };
    }

    loop {
        *queue.get_unchecked_mut(queue_head & 127).as_mut_ptr() = (cx, cy, cost);
        queue_head += 1;
        debug_assert_eq!(queue_head - queue_tail, 120);

        (prev, (cx, cy, curr)) = (curr, next(input, prev, cx, cy, curr));
        cost += 1;

        // TODO: Store (ox << 16) | oy in queue?
        let (ox, oy, _) = *queue.get_unchecked(queue_tail & 127).as_ptr();
        queue_tail += 1;
        let (mut ow, mut oz) = xy_to_wz!(ox, oy);

        let (cx, cy) = (curr % 142, curr / 142);
        for j in queue_tail..queue_tail + 19 {
            let (px, py, pcost) = *queue.get_unchecked(j & 127).as_ptr();
            if pcost + cx.abs_diff(px) + cy.abs_diff(py) + 100 <= cost {
                count += 1;
            }
        }

        #[cfg(debug_assertions)]
        set.insert((ow, oz));

        let mut node = &mut qtree_root;
        let mut size = 512;
        while *node & MASK == 0 {
            size /= 2;
            debug_assert!(ow < 2 * size);
            debug_assert!(oz < 2 * size);
            // TODO: pext? + merged ow/oz
            let dow = ow >= size;
            let doz = oz >= size;
            ow &= size - 1;
            oz &= size - 1;
            let ptr = &mut *qtree.get_unchecked_mut(*node as usize).as_mut_ptr();
            *ptr.get_unchecked_mut(4) += 1;
            node = ptr.get_unchecked_mut(2 * doz as usize + dow as usize);
        }
        if *node != EMPTY {
            let cnode = *node;
            let cpos = (cnode & !MASK) as usize;
            let (px, py) = (cpos % 142, cpos / 142);
            let (mut pw, mut pz) = xy_to_wz!(px, py);
            (pw, pz) = (pw & (size - 1), pz & (size - 1));
            loop {
                let new_node = qtree_len;
                qtree_len += 1;
                *node = new_node as u16;
                *qtree.get_unchecked_mut(new_node).as_mut_ptr() = [EMPTY, EMPTY, EMPTY, EMPTY, 2];
                let ptr = &mut *qtree.get_unchecked_mut(new_node).as_mut_ptr();

                size /= 2;

                debug_assert!(pw < 2 * size);
                debug_assert!(pz < 2 * size);
                let dpw = pw >= size;
                let dpz = pz >= size;
                pw &= size - 1;
                pz &= size - 1;
                *ptr.get_unchecked_mut(2 * dpz as usize + dpw as usize) = cnode;

                debug_assert!(ow < 2 * size);
                debug_assert!(oz < 2 * size);
                let dow = ow >= size;
                let doz = oz >= size;
                ow &= size - 1;
                oz &= size - 1;
                node = ptr.get_unchecked_mut(2 * doz as usize + dow as usize);

                if *node == EMPTY {
                    break;
                }
            }
        }
        *node = MASK | (142 * oy + ox) as u16;

        #[cfg(debug_assertions)]
        if queue_tail >= 2 {
            assert_eq!(
                qtree[qtree_root as usize].assume_init_ref()[4] as usize,
                queue_tail
            );
        }

        #[cfg(debug_assertions)]
        if curr == e {
            let mut new_seen = HashSet::new();
            check_qtree(&set, &qtree, qtree_root, &mut new_seen, 0, 0, 512);
            assert_eq!(new_seen, set);

            unsafe fn check_qtree(
                set: &HashSet<(usize, usize)>,
                qtree: &[MaybeUninit<[u16; 5]>],
                node: u16,
                new_seen: &mut HashSet<(usize, usize)>,
                bw: usize,
                bz: usize,
                size: usize,
            ) -> usize {
                if node & MASK == 0 {
                    let n = &*qtree[node as usize].as_ptr();
                    let mut tot = 0;
                    let size = size / 2;
                    tot += check_qtree(set, qtree, n[0], new_seen, bw, bz, size);
                    tot += check_qtree(set, qtree, n[1], new_seen, bw + size, bz, size);
                    tot += check_qtree(set, qtree, n[2], new_seen, bw, bz + size, size);
                    tot += check_qtree(set, qtree, n[3], new_seen, bw + size, bz + size, size);
                    assert_eq!(tot, n[4] as usize);
                    tot
                } else if node != EMPTY {
                    let n = (node & !MASK) as usize;
                    let (x, y) = (n % 142, n / 142);
                    let (w, z) = xy_to_wz!(x, y);
                    assert!(set.contains(&(w, z)));
                    assert!(new_seen.insert((w, z)));
                    assert!(bw <= w);
                    assert!(w < bw + size);
                    assert!(bz <= z);
                    assert!(z < bz + size);
                    1
                } else {
                    0
                }
            }
        }

        let (x, y) = (curr % 142, curr / 142);
        let (w, z) = xy_to_wz!(x as u16, y as u16);
        let (minw, maxw) = (w - 20, w + 20);
        let (minz, maxz) = (z - 20, z + 20);

        let mut stack = [MaybeUninit::uninit(); 32];
        let mut stack_len = 1;
        *stack.get_unchecked_mut(0).as_mut_ptr() = (qtree_root, 0u16, 0u16, 512u16);
        loop {
            stack_len -= 1;
            let (node, bw, bz, mut size) = *stack.get_unchecked(stack_len).as_ptr();

            if node & MASK != 0 {
                let npos = (node & !MASK) as usize;
                let (nx, ny) = (npos % 142, npos / 142);
                let (nw, nz) = xy_to_wz!(nx as u16, ny as u16);
                // TODO: wrapping_sub check
                if minw <= nw && nw <= maxw && minz <= nz && nz <= maxz {
                    debug_assert!(npos < 142 * 141, "{node}");
                    debug_assert!(nx.abs_diff(x) + ny.abs_diff(y) <= 20);
                    count += 1;
                }
            } else {
                let node = &*qtree.get_unchecked(node as usize).as_ptr();

                if minw <= bw && bw + size <= maxw && minz <= bz && bz + size <= maxz {
                    count += *node.get_unchecked(4) as u64;
                } else {
                    macro_rules! handle {
                        ($bw:expr, $bz:expr, $dw:expr, $dz:expr) => {{
                            let (bw, bz) = ($bw + size * $dw, $bz + size * $dz);
                            let new_node = *node.get_unchecked(2 * $dz + $dw);
                            if bw <= maxw
                                && bw + size >= minw
                                && bz <= maxz
                                && bz + size >= minz
                                && new_node != EMPTY
                            {
                                *stack.get_unchecked_mut(stack_len).as_mut_ptr() =
                                    (new_node, bw, bz, size);
                                stack_len += 1;
                            }
                        }};
                    }
                    size /= 2;
                    handle!(bw, bz, 0, 0);
                    handle!(bw, bz, 1, 0);
                    handle!(bw, bz, 0, 1);
                    handle!(bw, bz, 1, 1);
                }
            }

            if stack_len == 0 {
                break;
            }
        }

        if curr == e {
            break;
        }
    }

    count
}
