use aoc2023::read_input;

fn main() {
    let mut buf = vec![0; 0x10000];
    let input = read_input(&mut buf);

    let (a, b) = solve(input);

    println!("04a: {a}");
    println!("04b: {b}");
}

fn solve(input: &[u8]) -> (usize, usize) {
    const N: usize = 0x10;
    let mut sum1 = 0;

    let mut score = [0; 0x100];

    let mut num_cards = 0;
    let mut i = 0;
    while i < input.len() {
        while input[i] != b':' {
            i += 1;
        }
        i += 1;

        let mut parsing_number = false;
        let mut winning_numbers = [0; N];
        let mut winning_numbers_len = 0;
        let mut number = 0;
        while input[i] != b'|' {
            if let Some(digit) = into_digit(input[i]) {
                parsing_number = true;
                number *= 10;
                number += digit;
            } else if parsing_number {
                winning_numbers[winning_numbers_len] = number;
                number = 0;
                winning_numbers_len += 1;
                parsing_number = false;
            }
            i += 1;
        }
        i += 1;

        let mut hits = 0;
        loop {
            if let Some(digit) = into_digit(input[i]) {
                parsing_number = true;
                number *= 10;
                number += digit;
            } else if parsing_number {
                for winning_number in winning_numbers.iter().take(winning_numbers_len) {
                    if *winning_number == number {
                        hits += 1;
                        break;
                    }
                }
                number = 0;
                parsing_number = false;
            }
            if input[i] == b'\n' {
                i += 1;
                break;
            }
            i += 1;
        }
        score[num_cards] = hits;
        num_cards += 1;
        if hits > 0 {
            sum1 += 1 << (hits - 1);
        }
    }

    let mut sum2 = 0;
    i = num_cards;
    while i > 0 {
        i -= 1;
        let n = score[i];
        score[i] = score[i + 1..][..n].iter().sum();
        score[i] += 1;
        sum2 += score[i];
    }

    (sum1, sum2)
}

fn into_digit(byte: u8) -> Option<u8> {
    if (b'0'..=b'9').contains(&byte) {
        Some(byte - b'0')
    } else {
        None
    }
}
