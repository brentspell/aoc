pub fn solve() {
    let data = std::fs::read_to_string("data/day01.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    // part 1
    println!("part 1: {}", part1(&lines));

    // part 2
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[&str]) -> u64 {
    let mut d = 50;
    let mut n = 0;
    for line in lines {
        // apply the rotation
        d += parse(line);

        // floored modulo 100
        d = floormod(d, 100);

        // count the number of times we land on 0
        if d == 0 {
            n += 1;
        }
    }

    n
}

fn part2(lines: &[&str]) -> u64 {
    let mut d = 50;
    let mut n = 0;
    for line in lines {
        // save the previous position and apply the rotation
        let p = d;
        d += parse(line);

        // count the number of zero crossings
        // add an additional crossing for -100 < d <= 0 when not starting at 0
        n += (d / 100).unsigned_abs();
        if d <= 0 && p > 0 {
            n += 1;
        }

        // floored modulo 100
        d = floormod(d, 100);
    }

    n
}

fn parse(line: &str) -> i64 {
    (match &line[..1] {
        "L" => -1,
        "R" => 1,
        _ => panic!("invalid direction"),
    }) * line[1..].parse::<i64>().expect("invalid distance")
}

fn floormod(a: i64, b: i64) -> i64 {
    let r = a % b;
    if r < 0 { r + b } else { r }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &[&str] = &[
        "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
    ];

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 6);
    }
}
