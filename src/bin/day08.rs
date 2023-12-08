use aoc2023::{read_input, Parser};

fn main() {
    let mut buf = vec![0; 0x10000];
    let input = read_input(&mut buf);

    let a = solve1(input);
    let b = solve2(input);

    println!("08a: {a}");
    println!("08b: {b}");
}

fn solve1(input: &[u8]) -> usize {
    let mut parser = Parser::new(input);
    let mut map = [[0_u16; 2]; 0x8000];

    let directions = parser.parse_word().unwrap();
    parser.skip_whitespace();

    while let Some([src, left, right]) = parse_line(&mut parser) {
        map[id2ix(src)] = [id2ix(left) as u16, id2ix(right) as u16];
    }

    let mut n = 0;
    let mut pos = 0;
    for dir in core::iter::repeat(directions).flat_map(|ds| ds.iter()) {
        pos = match dir {
            b'L' => map[pos as usize][0],
            b'R' => map[pos as usize][1],
            _ => unreachable!(),
        };

        n += 1;

        if pos == id2ix(b"ZZZ") as u16 {
            break;
        }
    }

    n
}

fn solve2(input: &[u8]) -> usize {
    let mut parser = Parser::new(input);
    let mut map = [[0_u16; 2]; 0x8000];

    let directions = parser.parse_word().unwrap();
    parser.skip_whitespace();

    let mut poss = [0_u16; 0x400];
    let mut poss_len = 0;

    while let Some([src, left, right]) = parse_line(&mut parser) {
        let src_ix = id2ix(src);
        if src_ix % 32 == 0 {
            poss[poss_len] = src_ix as u16;
            poss_len += 1;
        }
        map[src_ix] = [id2ix(left) as u16, id2ix(right) as u16];
    }

    let mut ns = [[0; 2]; 0x400];

    for (pos, ntry) in poss.iter_mut().zip(ns.iter_mut()).take(poss_len) {
        let mut cycle_start = 0;
        let mut cycle_len = 0;
        let mut in_cycle = false;
        for dir in core::iter::repeat(directions).flat_map(|ds| ds.iter()) {
            *pos = match dir {
                b'L' => map[*pos as usize][0],
                b'R' => map[*pos as usize][1],
                _ => unreachable!(),
            };

            if in_cycle {
                cycle_len += 1;
                if *pos % 32 == 25 {
                    // cycle_start always equals cycle_len in the data
                    *ntry = [cycle_start, cycle_len];
                    break;
                }
            } else {
                cycle_start += 1;
                if *pos % 32 == 25 {
                    in_cycle = true;
                }
            }
        }
    }

    ns.iter()
        .copied()
        .take(poss_len)
        .map(|[_, n]| n)
        .reduce(lcm)
        .unwrap()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        let r = a % b;
        a = b;
        b = r;
    }

    a
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn id2ix(id: &[u8]) -> usize {
    let mut ix = 0;
    for b in id {
        ix *= 32;
        ix += (b - b'A') as usize;
    }

    ix
}

fn parse_line<'m>(parser: &mut Parser<'m>) -> Option<[&'m [u8]; 3]> {
    let src = parser.skip_n(3)?;
    parser.parse_exact(b" = (")?;
    let left = parser.skip_n(3)?;
    parser.skip_n(2)?;
    let right = parser.skip_n(3)?;
    parser.skip_line()?;
    Some([src, left, right])
}
