use aoc2023::{read_input, Parser};

fn main() {
    let mut buf = vec![0; 0x10000];
    let input = read_input(&mut buf);

    let (a, b) = solve(input);

    println!("10a: {a}");
    println!("10b: {b}");
}

fn solve(input: &mut [u8]) -> (usize, usize) {
    const GRID_LEN: usize = 0x200;

    let mut grid = [0_u8; GRID_LEN * GRID_LEN];

    let row_len = Parser::new(input).skip_line().unwrap().len();
    let start = input.iter().position(|b| *b == b'S').unwrap();

    let left_horz = matches!(input.get(start - 1), Some(b'-') | Some(b'L') | Some(b'F'));
    let right_horz = matches!(input.get(start + 1), Some(b'-') | Some(b'J') | Some(b'7'));
    let up_vert = matches!(
        input.get(start - row_len),
        Some(b'|') | Some(b'F') | Some(b'7')
    );
    let down_vert = matches!(
        input.get(start + row_len),
        Some(b'|') | Some(b'L') | Some(b'J')
    );

    let (start_row, start_col) = ix2sub(row_len, start);

    let mut ixs;
    *by_sub_mut(&mut grid, GRID_LEN, start_row * 2 + 1, start_col * 2 + 1)
        .unwrap()
        .0 = 1;
    if left_horz && right_horz {
        input[start] = b'-';
        *by_sub_mut(&mut grid, GRID_LEN, start_row * 2 + 1, start_col * 2)
            .unwrap()
            .0 = 1;
        *by_sub_mut(&mut grid, GRID_LEN, start_row * 2 + 1, start_col * 2 + 2)
            .unwrap()
            .0 = 1;
        ixs = [start - 1, start + 1];
    } else if up_vert && down_vert {
        input[start] = b'|';
        *by_sub_mut(&mut grid, GRID_LEN, start_row * 2, start_col * 2 + 1)
            .unwrap()
            .0 = 1;
        *by_sub_mut(&mut grid, GRID_LEN, start_row * 2 + 2, start_col * 2 + 1)
            .unwrap()
            .0 = 1;
        ixs = [start - row_len, start + row_len];
    } else if up_vert && right_horz {
        input[start] = b'F';
        *by_sub_mut(&mut grid, GRID_LEN, start_row * 2, start_col * 2 + 1)
            .unwrap()
            .0 = 1;
        *by_sub_mut(&mut grid, GRID_LEN, start_row * 2 + 1, start_col * 2 + 2)
            .unwrap()
            .0 = 1;
        ixs = [start - row_len, start + 1];
    } else if down_vert && right_horz {
        input[start] = b'L';
        *by_sub_mut(&mut grid, GRID_LEN, start_row * 2 + 2, start_col * 2 + 1)
            .unwrap()
            .0 = 1;
        *by_sub_mut(&mut grid, GRID_LEN, start_row * 2 + 1, start_col * 2 + 2)
            .unwrap()
            .0 = 1;
        ixs = [start + row_len, start + 1];
    } else if up_vert && left_horz {
        input[start] = b'J';
        *by_sub_mut(&mut grid, GRID_LEN, start_row * 2, start_col * 2 + 1)
            .unwrap()
            .0 = 1;
        *by_sub_mut(&mut grid, GRID_LEN, start_row * 2 + 1, start_col * 2)
            .unwrap()
            .0 = 1;
        ixs = [start - row_len, start - 1];
    } else if down_vert && left_horz {
        input[start] = b'7';
        *by_sub_mut(&mut grid, GRID_LEN, start_row * 2 + 2, start_col * 2 + 1)
            .unwrap()
            .0 = 1;
        *by_sub_mut(&mut grid, GRID_LEN, start_row * 2 + 1, start_col * 2)
            .unwrap()
            .0 = 1;
        ixs = [start + row_len, start - 1];
    } else {
        dbg!(left_horz, right_horz, up_vert, down_vert);
        panic!();
    }

    let mut prevs = [start, start];

    let mut sum1 = 1;
    loop {
        sum1 += 1;
        for (ix, prev) in ixs.iter_mut().zip(prevs.iter_mut()) {
            let (row, col) = ix2sub(row_len, *ix);

            let current = input[*ix];

            let ds: [(isize, isize); 2] = match current {
                b'|' => [(-1, 0), (1, 0)],
                b'-' => [(0, -1), (0, 1)],
                b'L' => [(-1, 0), (0, 1)],
                b'J' => [(-1, 0), (0, -1)],
                b'7' => [(0, -1), (1, 0)],
                b'F' => [(0, 1), (1, 0)],
                b => panic!("unexpected character {}", b as char),
            };

            'outer: for (drow, dcol) in ds {
                if let Some(next) = sub2ix(row_len, row + drow, col + dcol) {
                    if next == *prev {
                        continue;
                    }

                    match input.get(next) {
                        Some(b'|') if drow != 0 && dcol == 0 => {}
                        Some(b'-') if drow == 0 && dcol != 0 => {}
                        Some(b'L') if drow == 1 || dcol == -1 => {}
                        Some(b'J') if drow == 1 || dcol == 1 => {}
                        Some(b'7') if drow == -1 || dcol == 1 => {}
                        Some(b'F') if drow == -1 || dcol == -1 => {}
                        Some(b'.') | Some(b'#') | None => continue,
                        Some(b) => panic!("unexpected character {}", *b as char),
                    }
                    input[*prev] = b'#';
                    *prev = *ix;
                    *ix = next;

                    *by_sub_mut(&mut grid, GRID_LEN, row * 2 + 1, col * 2 + 1)
                        .unwrap()
                        .0 = 1;
                    *by_sub_mut(
                        &mut grid,
                        GRID_LEN,
                        (row + drow) * 2 + 1,
                        (col + dcol) * 2 + 1,
                    )
                    .unwrap()
                    .0 = 1;
                    *by_sub_mut(&mut grid, GRID_LEN, row * 2 + drow + 1, col * 2 + dcol + 1)
                        .unwrap()
                        .0 = 1;

                    break 'outer;
                }
            }
        }

        if ixs[0] == ixs[1] {
            break;
        }
    }

    let mut stack = [0; 0x20000];
    let mut stack_len = 1;

    while stack_len > 0 {
        let current = stack[stack_len - 1];
        stack_len -= 1;

        let (row, col) = ix2sub(GRID_LEN, current);

        for (drow, dcol) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let Some((next, ix)) = by_sub_mut(&mut grid, GRID_LEN, row + drow, col + dcol) {
                if *next == 0 {
                    *next = 2;
                    stack[stack_len] = ix;
                    stack_len += 1;
                }
            }
        }
    }

    let mut sum2 = 0;

    for (ix, b) in input.iter().enumerate() {
        let (row, col) = ix2sub(row_len, ix);
        if *b == b'\n' {
            continue;
        }

        if let Some((0, _)) = by_sub(&grid, GRID_LEN, row * 2 + 1, col * 2 + 1) {
            sum2 += 1;
        }
    }

    (sum1, sum2)
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

fn by_sub<T>(ar: &[T], row_len: usize, row: isize, col: isize) -> Option<(&T, usize)> {
    sub2ix(row_len, row, col).and_then(|ix| Some((ar.get(ix)?, ix)))
}

fn by_sub_mut<T>(ar: &mut [T], row_len: usize, row: isize, col: isize) -> Option<(&mut T, usize)> {
    sub2ix(row_len, row, col).and_then(|ix| Some((ar.get_mut(ix)?, ix)))
}
