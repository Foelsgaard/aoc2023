use aoc2023::{read_input, Parser};

fn main() {
    let mut buf = vec![0; 0x10000];
    let input = read_input(&mut buf);

    let a = solve::<false>(input);
    let b = solve::<true>(input);

    println!("07a: {a}");
    println!("07b: {b}");
}

fn solve<const JOKERS: bool>(input: &[u8]) -> usize {
    let mut parser = Parser::new(input);

    let mut bids = [(0, 0); 0x400];
    let mut bids_len = 0;

    while let Some((hand, bid)) = parser.parse_word().zip(parser.parse_usize()) {
        let value = hand_value::<JOKERS>(hand);
        bids[bids_len] = (value, bid);
        bids_len += 1;
        parser.skip_whitespace();
    }

    bids[..bids_len].sort_by(|(v0, _), (v1, _)| v0.cmp(v1));
    let mut sum = 0;
    for (i, bid) in bids.iter().enumerate().take(bids_len) {
        sum += bid.1 * (i + 1);
    }

    sum
}

fn hand_value<const JOKERS: bool>(hand: &[u8]) -> usize {
    let table = count_cards::<JOKERS>(hand);

    let mut value = if JOKERS {
        let jokers = table[0];

        // five of a kind
        if table[1..].iter().any(|entry| *entry + jokers == 5) {
            7

        // four of a kind
        } else if table[1..].iter().any(|entry| *entry + jokers == 4) {
            6

        // full house
        } else if jokers == 2 && table[1..].iter().any(|entry| *entry == 2)
            || jokers == 1 && table[1..].iter().filter(|entry| **entry == 2).count() == 2
            || jokers == 0
                && table[1..].iter().any(|entry| *entry == 3)
                && table[1..].iter().any(|entry| *entry == 2)
        {
            5

        // three of a kind
        } else if table[1..].iter().any(|entry| *entry + jokers == 3) {
            4
        // two pair
        } else if jokers == 1 && table[1..].iter().any(|entry| *entry == 2)
            || jokers == 0 && table[1..].iter().filter(|entry| **entry == 2).count() == 2
        {
            3
        // one pair
        } else if jokers == 1 || table[1..].iter().filter(|entry| **entry == 2).count() == 1 {
            2
        } else {
            1
        }
    } else {
        // five of a kind
        if table.iter().any(|entry| *entry == 5) {
            7

        // four of a kind
        } else if table.iter().any(|entry| *entry == 4) {
            6

        // full house
        } else if table.iter().any(|entry| *entry == 3) && table.iter().any(|entry| *entry == 2) {
            5

        // three of a kind
        } else if table.iter().any(|entry| *entry == 3) {
            4
        // two pair
        } else if table.iter().filter(|entry| **entry == 2).count() == 2 {
            3
        // one pair
        } else if table.iter().filter(|entry| **entry == 2).count() == 1 {
            2
        } else {
            1
        }
    };

    for card in hand {
        value *= 100;
        value += card_value::<JOKERS>(*card);
    }
    value
}

fn count_cards<const JOKERS: bool>(hand: &[u8]) -> [usize; 13] {
    let mut table = [0; 13];
    for card in hand {
        table[card_value::<JOKERS>(*card) - 1] += 1;
    }

    table
}

fn card_value<const JOKERS: bool>(card: u8) -> usize {
    if JOKERS {
        match card {
            b'A' => 13,
            b'K' => 12,
            b'Q' => 11,
            b'T' => 10,
            b'9' => 9,
            b'8' => 8,
            b'7' => 7,
            b'6' => 6,
            b'5' => 5,
            b'4' => 4,
            b'3' => 3,
            b'2' => 2,
            b'J' => 1,
            _ => panic!("invalid card {}", card as char),
        }
    } else {
        match card {
            b'A' => 13,
            b'K' => 12,
            b'Q' => 11,
            b'J' => 10,
            b'T' => 9,
            b'9' => 8,
            b'8' => 7,
            b'7' => 6,
            b'6' => 5,
            b'5' => 4,
            b'4' => 3,
            b'3' => 2,
            b'2' => 1,
            _ => panic!("invalid card {}", card as char),
        }
    }
}
