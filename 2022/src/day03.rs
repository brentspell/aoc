use std::collections::HashMap;

pub fn solve() {
    let data = std::fs::read_to_string("data/day03.txt").unwrap();
    let sacks: Vec<_> = parse(&data);

    // part 1
    println!("part 1: {}", part1(&sacks));

    // part 2
    println!("part 2: {}", part2(&sacks));
}

fn parse(string: &str) -> Vec<Vec<u32>> {
    string
        .lines()
        .map(|l| l.chars().map(|c| PRIORITIES[&c]).collect())
        .collect()
}

fn part1(sacks: &[Vec<u32>]) -> u32 {
    sacks
        .iter()
        .map(|s| {
            let h: usize = s.len() / 2;
            let lhs = s[..h].iter().fold(0_u64, |a, &x| a | (1 << x));
            let rhs = s[h..].iter().fold(0_u64, |a, &x| a | (1 << x));
            (lhs & rhs).trailing_zeros()
        })
        .sum()
}

fn part2(sacks: &[Vec<u32>]) -> u32 {
    sacks
        .chunks(3)
        .map(|s| {
            s.iter()
                .map(|s| s.iter().fold(0, |a, &x| a | (1 << x)))
                .fold(u64::MAX, |a, x| a & x)
                .trailing_zeros()
        })
        .sum()
}

lazy_static! {
    static ref PRIORITIES: HashMap<char, u32> =
        "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars()
            .zip(1..=52)
            .collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        vJrwpWtwJgWrhcsFMMfFFhFp\n\
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
        PmmdzqPrVvPwwTWBwg\n\
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
        ttgJtRGJQctTZtZT\n\
        CrZsJsPPZsGzwwsLwLmpwMDw\n\
    ";

    lazy_static! {
        static ref SACKS: Vec<Vec<u32>> = parse(DATA);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&SACKS), 157);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&SACKS), 70);
    }
}
