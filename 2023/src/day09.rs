pub fn solve() {
    let data = std::fs::read_to_string("data/day09.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> i64 {
    data.lines()
        .map(parse)
        .map(diff)
        .map(|d| d.iter().map(|x| x[x.len() - 1]).sum::<i64>())
        .sum()
}

fn part2(data: &str) -> i64 {
    data.lines()
        .map(parse)
        .map(diff)
        .map(|d| d.iter().rev().fold(0, |a, x| x[0] - a))
        .sum()
}

fn parse(s: &str) -> Vec<i64> {
    s.split_whitespace()
        .map(|v| v.parse::<i64>().unwrap())
        .collect()
}

fn diff(x: Vec<i64>) -> Vec<Vec<i64>> {
    let mut xs = vec![x];

    loop {
        let x = &xs[xs.len() - 1];
        if x.iter().all(|&v| v == 0) {
            break;
        }
        xs.push(x.windows(2).map(|x| x[1] - x[0]).collect());
    }

    xs
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        0 3 6 9 12 15\n\
        1 3 6 10 15 21\n\
        10 13 16 21 30 45\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 2);
    }
}
