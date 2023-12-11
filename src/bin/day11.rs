use aoc2023::{by_sub, by_sub_mut, ix2sub, read_input, Parser};

fn main() {
    let mut buf = vec![0; 0x10000];
    let input = read_input(&mut buf);

    let (a, b) = solve(input);

    println!("11a: {a}");
    println!("11b: {b}");
}

fn solve(input: &mut [u8]) -> (usize, usize) {
    let mut parser = Parser::new(input);

    let mut stars = [0; 0x1000];
    let mut stars_len = 0;

    let mut width = 0;
    let mut height = 0;
    let mut ix = 0;
    while let Some(line) = parser.skip_line() {
        for b in line {
            if *b == b'#' {
                stars[stars_len] = ix;
                stars_len += 1;
            }

            ix += 1;
        }

        width = line.len();
        height += 1;
    }

    for row in 0..height as isize {
        let mut is_empty = true;
        for col in 0..width as isize {
            if let Some((entry, _)) = by_sub_mut(input, width, row, col) {
                is_empty &= *entry != b'#';
            }
        }

        if is_empty {
            for col in 0..width as isize {
                if let Some((entry, _)) = by_sub_mut(input, width, row, col) {
                    if *entry != b'\n' {
                        *entry = b'-';
                    }
                }
            }
        }
    }

    for col in 0..width as isize {
        let mut is_empty = true;
        for row in 0..height as isize {
            if let Some((entry, _)) = by_sub_mut(input, width, row, col) {
                is_empty &= *entry != b'#';
            }
        }

        if is_empty {
            for row in 0..width as isize {
                if let Some((entry, _)) = by_sub_mut(input, width, row, col) {
                    if *entry == b'-' {
                        *entry = b'+';
                    } else if *entry != b'\n' {
                        *entry = b'|';
                    }
                }
            }
        }
    }

    let mut sum1 = 0;
    let mut sum2 = 0;
    for star0 in stars.iter().take(stars_len) {
        for star1 in stars.iter().take(stars_len) {
            let (mut r0, mut c0) = ix2sub(width, *star0);
            let (r1, c1) = ix2sub(width, *star1);

            let mut distance0: usize = 0;
            let mut distance1: usize = 0;
            while r0 != r1 {
                r0 += (r1 - r0).signum();
                let entry = by_sub(input, width, r0, c0).unwrap().0;
                if *entry == b'-' || *entry == b'+' {
                    distance0 += 2;
                    distance1 += 1000000;
                } else {
                    distance0 += 1;
                    distance1 += 1;
                }
            }
            while c0 != c1 {
                c0 += (c1 - c0).signum();
                let entry = by_sub(input, width, r0, c0).unwrap().0;
                if *entry == b'|' || *entry == b'+' {
                    distance0 += 2;
                    distance1 += 1000000;
                } else {
                    distance0 += 1;
                    distance1 += 1;
                }
            }

            sum1 += distance0;
            sum2 += distance1;
        }
    }

    (sum1 / 2, sum2 / 2)
}
