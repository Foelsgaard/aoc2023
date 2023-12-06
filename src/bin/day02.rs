use aoc2023::read_input;

fn main() {
    let mut buf = vec![0; 0x10000];
    let input = std::str::from_utf8(read_input(&mut buf)).unwrap();

    let (a, b) = solve(input);

    println!("01a: {a}");
    println!("01b: {b}");
}

fn solve(input: &str) -> (usize, usize) {
    let sum1 = input.lines().flat_map(validate).sum();
    let sum2 = input.lines().map(power).sum();

    (sum1, sum2)
}

fn validate(line: &str) -> Option<usize> {
    let (a, b) = line.split_once(": ").unwrap();
    let id = a.strip_prefix("Game ").unwrap().parse().ok().unwrap();

    for c in b.split("; ") {
        for d in c.split(", ") {
            let (e, f) = d.split_once(' ').unwrap();
            let amount: usize = e.parse().ok().unwrap();
            let max = match f {
                "red" => 12,
                "green" => 13,
                "blue" => 14,
                _ => panic!(),
            };

            if max < amount {
                return None;
            }
        }
    }

    Some(id)
}

fn power(line: &str) -> usize {
    let (_, b) = line.split_once(": ").unwrap();

    let mut max = [0; 3];

    for c in b.split("; ") {
        for d in c.split(", ") {
            let (e, f) = d.split_once(' ').unwrap();
            let amount: usize = e.parse().ok().unwrap();
            match f {
                "red" => max[0] = max[0].max(amount),
                "green" => max[1] = max[1].max(amount),
                "blue" => max[2] = max[2].max(amount),
                _ => panic!(),
            }
        }
    }

    max.iter().product()
}
