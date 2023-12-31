use aoc2023::{read_input, Parser};

fn main() {
    let mut buf = vec![0; 0x10000];
    let input = read_input(&mut buf);

    let (a, b) = solve(input);

    println!("09a: {a}");
    println!("09b: {b}");
}

fn solve(input: &[u8]) -> (isize, isize) {
    let mut parser = Parser::new(input);

    let mut table = [[0_isize; 0x20]; 0x400];

    let mut sum1 = 0;
    let mut sum2 = 0;
    while let Some(line) = parser.skip_line() {
        let mut line_parser = Parser::new(line);
        let mut n = 0;
        while let Some(x) = line_parser.parse_isize() {
            table[0][n] = x;
            n += 1;
        }

        if n == 0 {
            continue;
        }

        let mut i = 1;
        loop {
            let mut all_zeros = true;
            for j in 0..n - i {
                let x = table[i - 1][j + 1] - table[i - 1][j];
                table[i][j] = x;
                all_zeros &= x == 0;
            }
            if all_zeros {
                table[i][n - i] = 0;
                break;
            }

            i += 1;
        }

        let mut x = 0;
        for j in 1..=i {
            x = table[i - j + 1][n - i + j - 1] + table[i - j][n - i + j - 1];
            table[i - j][n - i + j] = x;
        }

        sum1 += x;

        x = 0;
        for j in 1..=i {
            x = -table[i - j + 1][n - i + j - 1] + table[i - j][0];
            table[i - j][n - i + j] = x;
        }

        sum2 += x;
    }

    (sum1, sum2)
}
