use aoc2023::{read_input, Parser};

fn main() {
    let mut buf = vec![0; 0x10000];
    let input = read_input(&mut buf);

    let a = solve(false, input);
    let b = solve(true, input);

    println!("12a: {a}");
    println!("12b: {b}");
}

fn solve(folded: bool, input: &mut [u8]) -> usize {
    let mut parser = Parser::new(input);

    let debug = false;
    let mut num_invalids = 0;
    let test = false;
    let mut sum = 0;
    while let Some(line) = parser.skip_line() {
        let mut line_parser = Parser::new(line);

        let mut statuses = line_parser.parse_word().unwrap();
        let mut groups = [0; 0x100];
        let mut groups_len = 0;
        while let Some(group) = line_parser.parse_usize() {
            groups[groups_len] = group;
            groups_len += 1;
            line_parser.next_byte();
        }

        let mut actual = [0; 0x100];
        if folded {
            let mut actual_len = 0;

            for _ in 0..4 {
                for status in statuses.iter() {
                    actual[actual_len] = *status;
                    actual_len += 1;
                }
                actual[actual_len] = b'?';
                actual_len += 1;
            }

            for status in statuses.iter() {
                actual[actual_len] = *status;
                actual_len += 1;
            }

            statuses = &actual[..actual_len];

            let n = groups_len;
            for _ in 0..4 {
                for i in 0..n {
                    groups[groups_len] = groups[i];
                    groups_len += 1;
                }
            }
        }

        if debug {
            for status in statuses {
                print!("{}", *status as char);
            }

            print!(" ");

            for group in groups.iter().take(groups_len) {
                print!("{group} ");
            }

            println!();
        }

        let mut perms = 0;
        let mut test_perms = 0;

        if test {
            let entropy = statuses.iter().filter(|s| **s == b'?').count();

            'perm_loop: for mut perm in 0..(1 << entropy) {
                let mut i = 0;
                let mut end = 0;

                let mut perm0 = perm;

                while end < statuses.len() {
                    while let Some(s) = statuses.get(end) {
                        if *s == b'.' {
                            end += 1;
                        } else if *s == b'?' && perm & 1 == 0 {
                            perm >>= 1;
                            end += 1;
                        } else {
                            break;
                        }
                    }

                    let mut size = 0;
                    while let Some(s) = statuses.get(end) {
                        if *s == b'#' {
                            size += 1;
                            end += 1;
                        } else if *s == b'?' && perm & 1 != 0 {
                            perm >>= 1;
                            size += 1;
                            end += 1;
                        } else {
                            break;
                        }
                    }

                    if i == groups_len {
                        if size == 0 {
                            break;
                        } else {
                            continue 'perm_loop;
                        }
                    }

                    if size != groups[i] {
                        continue 'perm_loop;
                    }

                    i += 1;
                }

                if i == groups_len && end == statuses.len() {
                    if debug {
                        for s in statuses.iter() {
                            match s {
                                b'?' => {
                                    if perm0 & 1 == 0 {
                                        print!(".");
                                    } else {
                                        print!("#");
                                    }
                                    perm0 >>= 1;
                                }
                                _ => print!("{}", *s as char),
                            }
                        }
                        println!();
                    }
                    test_perms += 1;
                }
            }
        }

        let mut stack = [(1, 0, 0, 0); 0x1000];
        let mut stack_len = 1;

        while stack_len > 0 {
            stack_len -= 1;
            let state = stack[stack_len];
            let (factor, offset, curr, carry) = state;

            let mut end = offset;
            while let Some(b'.') = statuses.get(end) {
                end += 1;
            }

            if offset < end && 0 < carry {
                continue;
            }

            if end == statuses.len() {
                if carry == 0 && curr == groups_len {
                    if debug {
                        println!("!!!");
                    }
                    perms += factor;
                } else {
                }
                continue;
            }

            let mut prefix = carry;
            while let Some(b'#') = statuses.get(end) {
                prefix += 1;
                end += 1;
            }

            if groups[curr] < prefix {
                continue;
            }

            let mut overhang = 0;
            let mut next = curr;
            if 0 < prefix {
                overhang = groups[curr] - prefix;
                next += 1;
            }

            let mut suffix = 0;
            while let Some(b'?') = statuses.get(end) {
                suffix += 1;
                end += 1;
            }

            if debug {
                let mut rem = statuses.len();
                for _ in 0..end - prefix - suffix {
                    rem -= 1;
                    print!(" ");
                }

                for _ in 0..prefix {
                    rem -= 1;
                    print!("+");
                }

                for _ in 0..suffix {
                    rem -= 1;
                    print!("-");
                }

                for _ in 0..rem {
                    print!(" ");
                }

                print!(" ");

                for _ in 0..curr {
                    print!("  ");
                }

                println!("*");
            }

            if suffix == 0 {
                stack[stack_len] = (factor, end, next, overhang);
                stack_len += 1;
                continue;
            }

            if prefix + suffix < groups[curr] {
                if prefix == 0 {
                    let mut new_carry = 0;
                    while new_carry <= suffix {
                        stack[stack_len] = (factor, end, curr, prefix + new_carry);
                        stack_len += 1;
                        new_carry += 1;
                    }
                } else {
                    stack[stack_len] = (factor, end, curr, prefix + suffix);
                    stack_len += 1;
                }
                continue;
            }

            let open = statuses.get(end).map_or(false, |s| *s == b'#');
            let mut unknowns = suffix - overhang;

            if open && unknowns == 0 {
                continue;
            }

            if next != curr {
                unknowns = unknowns.saturating_sub(1);
            }

            if unknowns == 0 {
                stack[stack_len] = (factor, end, next, 0);
                stack_len += 1;
                continue;
            }

            let mut k = 0;
            let mut n = unknowns;
            if !open {
                n += 1;
            }

            while k <= n && next <= groups_len {
                if open && next < groups_len {
                    let mut new_carry = groups[next].min(n) + 1;
                    while new_carry > 0 {
                        new_carry = new_carry.saturating_sub(1);
                        let new_factor = factor * n_choose_k(n - new_carry, k);
                        stack[stack_len] = (new_factor, end, next, new_carry);
                        stack_len += 1;
                    }
                } else {
                    let new_factor = factor * n_choose_k(n, k);
                    stack[stack_len] = (new_factor, end, next, 0);
                    stack_len += 1;
                }

                k += 1;
                n = n.saturating_sub(groups[next]);
                next += 1;
            }
        }

        if debug {
            print!(": {perms} arrangements");
            if test && perms != test_perms {
                num_invalids += 1;
                println!(" <- INVALID (actually {test_perms} arrangements)");
            }
            println!();
        }

        sum += perms;
    }

    if debug && test {
        println!("number of invalids: {num_invalids}");
    }

    sum
}

fn n_choose_k(n: usize, mut k: usize) -> usize {
    if n < k {
        return 0;
    }

    if k == 0 || k == n {
        return 1;
    }

    k = k.min(n - k);
    let mut c = 1;

    for i in 0..k {
        c = c * (n - i) / (i + 1);
    }

    c
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_n_choose_k() {
        assert_eq!(n_choose_k(4, 2), 6);
    }
}
