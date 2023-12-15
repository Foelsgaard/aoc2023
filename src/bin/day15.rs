use aoc2023::read_input;

fn main() {
    let mut buf = vec![0; 0x10000];
    let input = read_input(&mut buf);

    let a = solve1(input);
    let b = solve2(input);

    println!("15a: {a}");
    println!("15b: {b}");
}

fn solve1(input: &mut [u8]) -> usize {
    use core::num::Wrapping;

    let mut sum = 0;
    let mut val = Wrapping(0_u8);
    for b in &input[..input.len() - 1] {
        match b {
            b',' => {
                sum += val.0 as usize;
                val -= val;
            }
            _ => {
                val += *b;
                val *= 17;
            }
        }
    }
    sum += val.0 as usize;
    val -= val;

    sum
}

fn solve2(input: &mut [u8]) -> usize {
    use core::num::Wrapping;

    let mut table_label = [0; 0x10000];
    let mut table_focal = [0; 0x10000];
    let mut table_len = [0; 0x100];

    let mut hash = Wrapping(0_u8);
    let mut label = 0;

    let mut bytes = input.iter();
    while let Some(b) = bytes.next() {
        match b {
            b'=' => {
                let i = hash.0 as usize;
                let mut j = 0;
                while j < table_len[i] && table_label[i * 0x100 + j] != label {
                    j += 1;
                }
                table_len[i] = table_len[i].max(j + 1);
                let focal = *bytes.next().unwrap() - b'0';
                table_focal[i * 0x100 + j] = focal;
                table_label[i * 0x100 + j] = label;
                hash -= hash;
                label = 0;
            }
            b'-' => {
                let i = hash.0 as usize;
                let mut j = 0;
                while j < table_len[i] && table_label[i * 0x100 + j] != label {
                    j += 1;
                }

                if j < table_len[i] {
                    table_len[i] -= 1;
                }

                while j < table_len[i] {
                    table_label[i * 0x100 + j] = table_label[i * 0x100 + j + 1];
                    table_focal[i * 0x100 + j] = table_focal[i * 0x100 + j + 1];
                    j += 1;
                }

                hash -= hash;
                label = 0;
            }
            b',' => {}
            _ => {
                hash += *b;
                hash *= 17;
                label <<= 8;
                label |= *b as u64;
            }
        }
    }

    let mut sum = 0;
    for i in 0..0x100 {
        for j in 0..table_len[i] {
            sum += (1 + i) * (1 + j) * (table_focal[i * 0x100 + j] as usize);
        }
    }
    sum
}
