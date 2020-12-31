pub fn solve() {
    let data = std::fs::read_to_string("data/day10.txt").unwrap();

    // part 1
    let mut result = data.clone();
    for _ in 0..40 {
        result = transform(&result);
    }
    println!("part 1: {}", result.len());

    // part 2
    let mut result = data;
    for _ in 0..50 {
        result = transform(&result);
    }
    println!("part 2: {}", result.len());
}

fn transform(input: &str) -> String {
    let mut result = String::new();
    let mut count = 0;
    let skip = input.chars().skip(1).chain(std::iter::once(' '));
    for (c, n) in input.chars().zip(skip) {
        count += 1;
        if c != n {
            result.push_str(&count.to_string());
            result.push(c);
            count = 0;
        }
    }
    result
}
