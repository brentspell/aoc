pub fn solve() {
    let input: Vec<i32> = std::fs::read_to_string("data/day01.txt")
        .unwrap()
        .chars()
        .map(|c| match c { '(' => 1, ')' => -1, _ => 0 })
        .collect();

    // part 1
    let result: i32 = input.iter().sum();
    println!("part 1: {}", result);

    // part 2
    let result = input
        .iter()
        .scan(0, |a, x| { *a = *a + x; Some(*a) })
        .take_while(|&x| x >= 0)
        .count()
        + 1;
    println!("part 2: {}", result);
}
