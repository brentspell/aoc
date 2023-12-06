pub fn solve() {
    let data = std::fs::read_to_string("data/day06.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    let lines: Vec<Vec<u64>> = data
        .lines()
        .map(|l| {
            l.split_whitespace()
                .skip(1)
                .map(|s| s.parse::<u64>().unwrap())
                .collect()
        })
        .collect();

    let times = &lines[0];
    let dists = &lines[1];
    times
        .iter()
        .zip(dists.iter())
        .map(|(&t, &d)| (1..t).filter(|&v| v * (t - v) > d).count() as u64)
        .product()
}

fn part2(data: &str) -> u64 {
    let lines: Vec<u64> = data
        .replace(' ', "")
        .lines()
        .map(|l| l.split(':').nth(1).unwrap().parse().unwrap())
        .collect();

    let t = lines[0];
    let d = lines[1];
    (1..t).filter(|&v| v * (t - v) > d).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        Time:      7  15   30\n\
        Distance:  9  40  200\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 71503);
    }
}
