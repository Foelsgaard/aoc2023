use aoc2023::{read_input, Parser};

fn main() {
    let mut buf = vec![0; 0x10000];
    let input = read_input(&mut buf);

    let a = solve1(input);
    let b = solve2(input);

    println!("05a: {a}");
    println!("05b: {b}");
}

fn solve1(input: &[u8]) -> usize {
    let mut parser = Parser::new(input);

    let mut maps = [[[0_usize; 3]; 0x40]; 7];
    let mut seeds = [0_usize; 0x20];
    let mut seeds_len = 0;

    parser.parse_exact(b"seeds:");
    while let Some(seed) = parser.parse_usize() {
        seeds[seeds_len] = seed;
        seeds_len += 1;
    }
    parser.skip_whitespace();

    for i in 0..7 {
        let mut map_len = 0;
        parser.skip_line();
        loop {
            if let Some([dst, src, rng]) = parser.parse_usize_n() {
                maps[i][map_len] = [dst, src, rng];
                map_len += 1;
            } else {
                break;
            }
        }
        parser.skip_whitespace();
    }

    let mut min_x = usize::MAX;

    for &(mut x) in &seeds[..seeds_len] {
        for map in maps {
            for [dst, src, rng] in map {
                if src <= x && x < src + rng {
                    x = x + dst - src;
                    break;
                }
            }
        }
        min_x = min_x.min(x);
    }

    min_x
}

fn solve2(input: &[u8]) -> usize {
    let mut parser = Parser::new(input);

    let mut maps = [[[0; 3]; 0x40]; 7];
    let mut seeds = [[0; 3]; 0x20];
    let mut seeds_len = 0;

    parser.parse_exact(b"seeds:");
    while let Some([start, len]) = parser.parse_usize_n() {
        seeds[seeds_len] = [start, len, 0];
        seeds_len += 1;
    }
    parser.skip_whitespace();

    for i in 0..7 {
        let mut map_len = 0;
        parser.skip_line();
        loop {
            if let Some([dst, src, rng]) = parser.parse_usize_n() {
                maps[i][map_len] = [dst, src, rng];
                map_len += 1;
            } else {
                break;
            }
        }
        parser.skip_whitespace();
    }

    let mut min_x = usize::MAX;

    while seeds_len > 0 {
        let [mut start, mut len, mut i] = seeds[seeds_len - 1];
        seeds_len -= 1;
        for &map in &maps[i..] {
            for [dst, src, rng] in map {
                if src <= start && start < src + rng {
                    if start + len > src + rng {
                        let new_start = src + rng;
                        let new_len = start + len - (src + rng);
                        let new_seed = [new_start, new_len, i];
                        seeds[seeds_len] = new_seed;
                        seeds_len += 1;
                        len = src + rng - start;
                        start = start + dst - src;
                    } else {
                        start = start + dst - src;
                    }
                    break;
                }
            }
            i += 1;
        }

        min_x = min_x.min(start);
    }

    min_x
}
