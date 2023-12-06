#![cfg_attr(test, feature(test))]
#[cfg(test)]
extern crate test;

use aoc2023::read_input;

fn main() {
    let mut buf = vec![0; 0x10000];
    let input = read_input(&mut buf);

    let a = solve::<true>(input);
    let b = solve::<false>(input);

    println!("01a: {a}");
    println!("01b: {b}");
}

static TABLE: &[(&[u8], usize)] = &[
    (b"1", 1),
    (b"2", 2),
    (b"3", 3),
    (b"4", 4),
    (b"5", 5),
    (b"6", 6),
    (b"7", 7),
    (b"8", 8),
    (b"9", 9),
    (b"one", 1),
    (b"two", 2),
    (b"three", 3),
    (b"four", 4),
    (b"five", 5),
    (b"six", 6),
    (b"seven", 7),
    (b"eight", 8),
    (b"nine", 9),
];

fn solve<const DIGITS_ONLY: bool>(input: &[u8]) -> usize {
    let mut sum = 0;
    let mut first = 0;
    let mut last = 0;
    let mut first_found = false;
    let mut i = 0;
    'outer: while i < input.len() {
        let rest = &input[i..];
        i += 1;
        let table = if DIGITS_ONLY { &TABLE[0..9] } else { TABLE };
        for (pattern, value) in table {
            if Some(pattern) == rest.get(..pattern.len()).as_ref() {
                last = *value;
                if !first_found {
                    first = last;
                    first_found = true;
                }
                continue 'outer;
            }
        }

        match rest[0] {
            b'\n' => {
                sum += first * 10 + last;
                first_found = false;
                continue;
            }
            _ => continue,
        }
    }
    sum
}
