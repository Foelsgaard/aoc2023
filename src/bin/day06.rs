use aoc2023::{read_input, Parser};

fn main() {
    let mut buf = vec![0; 0x10000];
    let input = read_input(&mut buf);

    let a = solve1(input);
    let b = solve2(input);

    println!("06a: {a}");
    println!("06b: {b}");
}

fn solve1(input: &[u8]) -> usize {
    const N: usize = 4;

    let mut parser = Parser::new(input);

    let times = {
        parser.parse_exact(b"Time:");
        parser.parse_usize_n::<N>().unwrap()
    };

    parser.skip_whitespace();

    let distances = {
        parser.parse_exact(b"Distance:");
        parser.parse_usize_n::<N>().unwrap()
    };

    let mut product = 1;
    for (time, distance) in times.iter().zip(distances) {
        let mut sum = 0;
        for t0 in 0..*time {
            if (time - t0) * t0 > distance {
                sum += 1;
            }
        }
        product *= sum;
    }

    product
}

fn solve2(input: &[u8]) -> usize {
    let mut parser = Parser::new(input);

    parser.parse_exact(b"Time:");
    let mut time = 0;
    while let Some(digit) = parser.parse_digit() {
        time *= 10;
        time += (digit - b'0') as usize;
    }

    parser.skip_whitespace();

    parser.parse_exact(b"Distance:");
    let mut distance = 0;
    while let Some(digit) = parser.parse_digit() {
        distance *= 10;
        distance += (digit - b'0') as usize;
    }

    let mut sum = 0;
    for t0 in 0..time {
        if (time - t0) * t0 > distance {
            sum += 1;
        }
    }

    sum
}
