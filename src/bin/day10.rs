use aoc2023::{read_input, Parser};

fn main() {
    let mut buf = vec![0; 0x10000];
    let input = read_input(&mut buf);

    let (a, b) = solve(input);

    println!("10a: {a}");
    println!("10b: {b}");
}

fn solve(input: &mut [u8]) -> (isize, isize) {
    let row_len = Parser::new(input).skip_line().unwrap().len();
    let start = input.iter().position(|b| *b == b'S').unwrap();

    let left = input
        .get(start - 1)
        .map_or(false, |b| matches!(b, b'-' | b'L' | b'F'));
    let right = input
        .get(start + 1)
        .map_or(false, |b| matches!(b, b'-' | b'J' | b'7'));
    let up = input
        .get(start - row_len)
        .map_or(false, |b| matches!(b, b'|' | b'F' | b'7'));
    let down = input
        .get(start + row_len)
        .map_or(false, |b| matches!(b, b'|' | b'L' | b'J'));

    let width = row_len as isize;

    let connections = [
        (left, [-1, 0]),
        (right, [1, 0]),
        (up, [0, -1]),
        (down, [0, 1]),
    ];

    let [mut dx, mut dy] = connections
        .into_iter()
        .find_map(|(connected, dir)| connected.then_some(dir))
        .unwrap();

    let mut perimeter = 0;
    let mut area = 0;

    let [x0, y0] = [(start as isize) % width, (start as isize) / width];
    let [mut x, mut y] = [x0, y0];

    loop {
        let [x1, y1] = [x + dx, y + dy];

        // Shoelace formula
        area += y * x1 - y1 * x;

        perimeter += 1;

        let next = y1 * width + x1;
        match input[next as usize] {
            b'L' | b'7' => [dx, dy] = [dy, dx],
            b'J' | b'F' => [dx, dy] = [-dy, -dx],
            _ => {}
        }

        [x, y] = [x1, y1];

        if [x, y] == [x0, y0] {
            break;
        }
    }

    // Shoelace gives twice the area and includes the perimeter
    //
    // For some reason I don't fully understand, this always gives a result
    // that is one less than the correct amount, so we add one in the end..
    // This works for every test I've thrown against it.
    area = (area.abs() - perimeter) / 2 + 1;

    (perimeter / 2, area)
}
