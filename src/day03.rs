pub fn solve() {
    let input: Vec<char> = std::fs::read_to_string("data/day03.txt")
        .unwrap()
        .chars()
        .collect();

    // part 1
    let santa = input
        .iter()
        .scan((0, 0), &step);
    let result = vec![(0, 0)]
        .into_iter()
        .chain(santa)
        .collect::<::std::collections::HashSet<_>>()
        .len();

    println!("part 1: {}", result);

    // part 2
    let santa = input
        .iter()
        .step_by(2)
        .scan((0, 0), &step);
    let robot = input
        .iter()
        .skip(1)
        .step_by(2)
        .scan((0, 0), &step);
    let result = vec![(0, 0)]
        .into_iter()
        .chain(santa)
        .chain(robot)
        .collect::<::std::collections::HashSet<_>>()
        .len();
    println!("part 2: {}", result);
}

fn step((x, y): &mut (i32, i32), c: &char) -> Option<(i32, i32)> {
    match c {
        '^' => *y += 1,
        '>' => *x += 1,
        'v' => *y -= 1,
        '<' => *x -= 1,
        _ => ()
    };
    Some((*x, *y))
}
