use aoc2023::{read_input, Parser};

fn main() {
    let mut buf = vec![0; 0x10000];
    let input = read_input(&mut buf);

    let a = solve1(input);
    let b = solve2(input);

    println!("03a: {a}");
    println!("03b: {b}");
}

fn solve1(input: &[u8]) -> usize {
    let mut parser = Parser::new(input);

    let row_len = parser.peek(|p| p.skip_line()).unwrap().len();
    let mut sum = 0;

    while !parser.done() {
        parser.skip_matching(b'.');

        let start = parser.index();
        if let Some(number) = parser.parse_usize() {
            let end = parser.index();

            let mut part_number = false;
            'outer: for ix in start..end {
                let (row, col) = ix2sub(row_len, ix);
                for drow in [-1, 0, 1] {
                    for dcol in [-1, 0, 1] {
                        if let Some(b) = by_sub(input, row_len, row + drow, col + dcol) {
                            if !isdigit(b) && *b != b'.' && *b != b'\n' {
                                part_number = true;
                                break 'outer;
                            }
                        }
                    }
                }
            }

            if part_number {
                sum += number;
            }
        } else if let Some(_) = parser.next() {
        }
    }

    sum
}

fn solve2(input: &[u8]) -> usize {
    let mut sum = 0;

    let mut row_len = 0;
    for b in input {
        row_len += 1;
        if *b == b'\n' {
            break;
        }
    }

    for (ix, b) in input.iter().enumerate() {
        if *b != b'*' {
            continue;
        }

        let (row, col) = ix2sub(row_len, ix);
        let mut filter = [false; 9];
        let mut gear_ratio = 1;
        let mut ratio_count = 0;

        for drow in [-1, 0, 1] {
            for dcol in [-1, 0, 1] {
                if let Some(entry) = by_sub_mut(&mut filter, 3, 1 + drow, 1 + dcol) {
                    if *entry {
                        continue;
                    }
                    *entry = true;
                }

                if drow == 0 && dcol == 0 {
                    continue;
                }

                if let Some(true) = by_sub(input, row_len, row + drow, col + dcol).map(isdigit) {
                    let mut a = dcol;
                    let mut b = dcol;
                    while let Some(true) =
                        by_sub(input, row_len, row + drow, col + a - 1).map(isdigit)
                    {
                        a -= 1;
                    }
                    while let Some(true) =
                        by_sub(input, row_len, row + drow, col + b + 1).map(isdigit)
                    {
                        b += 1;
                    }

                    let mut number = 0;
                    for offset in a..=b {
                        if let Some(digit) = by_sub(input, row_len, row + drow, col + offset) {
                            number *= 10;
                            number += (*digit - b'0') as usize;
                        }

                        if let Some(entry) = by_sub_mut(&mut filter, 3, 1 + drow, 1 + offset) {
                            *entry = true;
                        }
                    }

                    gear_ratio *= number;
                    ratio_count += 1;
                    if ratio_count == 2 {
                        sum += gear_ratio;
                    }
                }
            }
        }
    }
    sum
}

fn ix2sub(row_len: usize, ix: usize) -> (isize, isize) {
    let row = (ix / row_len) as isize;
    let col = (ix % row_len) as isize;

    (row, col)
}

fn sub2ix(row_len: usize, row: isize, col: isize) -> Option<usize> {
    if row < 0 || row_len <= (col as usize) || col < 0 {
        return None;
    }
    Some((row * (row_len as isize) + col) as usize)
}

fn isdigit(byte: &u8) -> bool {
    b'0' <= *byte && *byte <= b'9'
}

fn by_sub<T>(ar: &[T], row_len: usize, row: isize, col: isize) -> Option<&T> {
    sub2ix(row_len, row, col).and_then(|ix| ar.get(ix))
}

fn by_sub_mut<T>(ar: &mut [T], row_len: usize, row: isize, col: isize) -> Option<&mut T> {
    sub2ix(row_len, row, col).and_then(|ix| ar.get_mut(ix))
}
