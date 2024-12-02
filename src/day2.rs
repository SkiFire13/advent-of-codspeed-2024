pub fn run(input: &str) -> i64 {
    part2(input) as i64
}

pub fn part1(input: &str) -> u32 {
    let mut input = input.as_bytes().iter();

    unsafe fn read(input: &mut std::slice::Iter<u8>) -> (i8, u8) {
        let d1 = *input.next().unwrap_unchecked();
        let mut d2 = *input.next().unwrap_unchecked();

        let mut n = d1 - b'0';

        if d2 >= b'0' {
            n = 10 * n + (d2 - b'0');
            d2 = *input.next().unwrap_unchecked();
        }

        (n as i8, d2)
    }

    let mut count = 0;
    unsafe {
        while !input.as_slice().is_empty() {
            let (n1, _) = read(&mut input);
            let (n2, c2) = read(&mut input);

            let diff = n2 - n1;

            let mut prev = n2;
            let mut ctrl = c2;
            let mut valid = diff != 0 && (-3..4).contains(&diff);

            if valid {
                if diff > 0 {
                    while valid && ctrl != b'\n' {
                        let (n, c) = read(&mut input);
                        let new_diff = n - prev;
                        (prev, ctrl) = (n, c);

                        valid &= (1..4).contains(&new_diff);
                    }
                } else {
                    while valid && ctrl != b'\n' {
                        let (n, c) = read(&mut input);
                        let new_diff = n - prev;
                        (prev, ctrl) = (n, c);

                        valid &= (-3..0).contains(&new_diff);
                    }
                }
            }

            if ctrl != b'\n' {
                while *input.next().unwrap_unchecked() != b'\n' {}
            }

            if valid {
                count += 1;
            }
        }
    }

    count
}

pub fn part2(input: &str) -> u32 {
    let mut input = input.as_bytes().iter();

    unsafe fn read(input: &mut std::slice::Iter<u8>) -> (i8, u8) {
        let d1 = *input.next().unwrap_unchecked();
        let mut d2 = *input.next().unwrap_unchecked();

        let mut n = d1 - b'0';

        if d2 >= b'0' {
            n = 10 * n + (d2 - b'0');
            d2 = *input.next().unwrap_unchecked();
        }

        (n as i8, d2)
    }

    let mut count = 0;
    unsafe {
        while !input.as_slice().is_empty() {
            let (n1, _) = read(&mut input);
            let (n2, c2) = read(&mut input);

            let diff = n2 - n1;

            let mut i = 2;
            let mut prevprev = n1;
            let mut prev = n2;
            let mut ctrl = c2;
            let (mut lt_s, mut gt_s) = (0, 0);
            let (mut lt_e, mut gt_e) = (u32::MAX, u32::MAX);
            let mut lt_ppv = (1..4).contains(&diff);
            let mut gt_ppv = (-3..0).contains(&diff);

            while lt_s < lt_e && gt_s < gt_e && ctrl != b'\n' {
                let (n, c) = read(&mut input);
                let new_diff = n - prev;
                let rdiff = n - prevprev;

                if !lt_ppv {
                    lt_e = lt_e.min(if (1..4).contains(&rdiff) { i } else { i - 1 });
                }

                if !gt_ppv {
                    gt_e = gt_e.min(if (-3..0).contains(&rdiff) { i } else { i - 1 });
                }

                lt_ppv = (1..4).contains(&new_diff);
                if !lt_ppv {
                    lt_s = lt_s.max(if (1..4).contains(&rdiff) { i - 1 } else { i });
                }

                gt_ppv = (-3..0).contains(&new_diff);
                if !gt_ppv {
                    gt_s = gt_s.max(if (-3..0).contains(&rdiff) { i - 1 } else { i });
                }

                (prevprev, prev, ctrl) = (prev, n, c);
                i += 1;
            }

            while lt_s < lt_e && ctrl != b'\n' {
                let (n, c) = read(&mut input);
                let new_diff = n - prev;

                let old_lt_ppv = lt_ppv;
                lt_ppv = (1..4).contains(&new_diff);

                if !old_lt_ppv || !lt_ppv {
                    let rdiff = n - prevprev;
                    let r_ok = (1..4).contains(&rdiff);
                    if !old_lt_ppv {
                        lt_e = lt_e.min(if r_ok { i } else { i - 1 });
                    }
                    if !lt_ppv {
                        lt_s = lt_s.max(if r_ok { i - 1 } else { i });
                    }
                }

                (prevprev, prev, ctrl) = (prev, n, c);
                i += 1;
            }

            while gt_s < gt_e && ctrl != b'\n' {
                let (n, c) = read(&mut input);
                let new_diff = n - prev;

                let old_gt_ppv = gt_ppv;
                gt_ppv = (-3..0).contains(&new_diff);

                if !old_gt_ppv || !gt_ppv {
                    let rdiff = n - prevprev;
                    let r_ok = (-3..0).contains(&rdiff);
                    if !old_gt_ppv {
                        gt_e = gt_e.min(if r_ok { i } else { i - 1 });
                    }
                    if !gt_ppv {
                        gt_s = gt_s.max(if r_ok { i - 1 } else { i });
                    }
                }

                (prevprev, prev, ctrl) = (prev, n, c);
                i += 1;
            }

            if ctrl != b'\n' {
                while *input.next().unwrap_unchecked() != b'\n' {}
            } else {
                if !lt_ppv {
                    lt_e = lt_e.min(i);
                }
                if !gt_ppv {
                    gt_e = gt_e.min(i);
                }
            }

            if lt_s < lt_e || gt_s < gt_e {
                count += 1;
            }
        }
    }

    count
}
