use aoc2023::{read_input, Parser};

fn main() {
    let mut buf = vec![0; 0x10000];
    let input = read_input(&mut buf);

    let a = solve::<2>(input);
    let b = solve::<1000000>(input);

    println!("11a: {a}");
    println!("11b: {b}");
}

fn solve<const K: usize>(input: &mut [u8]) -> usize {
    let mut parser = Parser::new(input);

    let mut rowsum = [K; 0x100];
    let mut colsum = [K; 0x100];

    let mut stars = [(0, 0); 0x1000];
    let mut stars_len = 0;

    let mut width = 0;
    let mut height = 0;
    let mut ix = 0;
    while let Some(line) = parser.skip_line() {
        width = line.len();
        for b in line {
            let row = ix / width;
            let col = ix % width;

            if *b == b'#' {
                stars[stars_len] = (row, col);
                stars_len += 1;
                rowsum[row] = 1;
                colsum[col] = 1;
            }

            ix += 1;
        }

        height += 1;
    }

    for i in 1..width {
        rowsum[i] += rowsum[i - 1];
    }
    for i in 1..height {
        colsum[i] += colsum[i - 1];
    }

    let mut sum = 0;

    for (n, (r0, c0)) in stars.iter().enumerate().take(stars_len) {
        for (r1, c1) in stars.iter().skip(n + 1).take(stars_len - n - 1) {
            sum += rowsum[*r0.max(r1)] - rowsum[*r0.min(r1)];
            sum += colsum[*c0.max(c1)] - colsum[*c0.min(c1)];
        }
    }

    sum
}
