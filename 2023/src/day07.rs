use itertools::Itertools;
use std::collections::HashMap;

pub fn solve() {
    let data = std::fs::read_to_string("data/day07.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    data.lines()
        .map(|line| {
            // parse the hand line into a hand and a bid
            let mut s = line.split_whitespace();
            let h = s.next().unwrap();
            let b: u64 = s.next().unwrap().parse().unwrap();

            // calculate suit frequencies
            let c: Vec<_> = counts(&h.chars().collect::<Vec<_>>())
                .values()
                .copied()
                .sorted()
                .collect();

            // convert frequencies to hand types
            let t = match c.as_slice() {
                [5] => 6,
                [1, 4] => 5,
                [2, 3] => 4,
                [1, 1, 3] => 3,
                [1, 2, 2] => 2,
                [1, 1, 1, 2] => 1,
                _ => 0,
            };

            // map suits to ordinals
            let h: Vec<_> = h
                .chars()
                .map(|c| match c {
                    'T' => 10,
                    'J' => 11,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => c.to_digit(10).unwrap() as u64,
                })
                .collect();

            (t, h, b)
        })
        .sorted()
        .enumerate()
        .map(|(i, (_, _, b))| b * (i as u64 + 1))
        .sum()
}

fn part2(data: &str) -> u64 {
    data.lines()
        .map(|line| {
            // parse the hand line into a hand and a bid
            let mut s = line.split_whitespace();
            let h = s.next().unwrap();
            let b: u64 = s.next().unwrap().parse().unwrap();

            // calculate suit frequencies and get the number of jokers
            let c = counts(&h.chars().collect::<Vec<_>>());
            let v: Vec<_> = c.values().sorted().collect();

            // convert frequences to hand types, now considering jokers
            let t = match (v.as_slice(), c.get(&'J').unwrap_or(&0)) {
                // 5-of-a-kind
                ([5], _) => 6,
                // 4-of-a-kind
                ([1, 4], 0) => 5,
                ([1, 4], _) => 6, // => 5-of-a-kind
                // full house
                ([2, 3], 0) => 4,
                ([2, 3], _) => 6, // => 5-of-a-kind
                // 3-of-a-kind
                ([1, 1, 3], 0) => 3,
                ([1, 1, 3], _) => 5, // => 4-of-a-kind
                // two pair
                ([1, 2, 2], 0) => 2,
                ([1, 2, 2], 1) => 4, // => 3-of-a-kind
                ([1, 2, 2], 2) => 5, // => 4-of-a-kind
                // one pair
                ([1, 1, 1, 2], 0) => 1,
                ([1, 1, 1, 2], _) => 3, // => 3-of-a-kind
                // high card
                (_, 1) => 1, // => pair
                _ => 0,
            };

            // map suits to ordinals (joker is now 0)
            let h: Vec<_> = h
                .chars()
                .map(|c| match c {
                    'T' => 10,
                    'J' => 0,
                    'Q' => 12,
                    'K' => 13,
                    'A' => 14,
                    _ => c.to_digit(10).unwrap() as u64,
                })
                .collect();

            (t, h, b)
        })
        .sorted()
        .enumerate()
        .map(|(i, (_, _, b))| b * (i as u64 + 1))
        .sum()
}

fn counts<T: Copy + Eq + std::hash::Hash>(s: &[T]) -> HashMap<T, u64> {
    let mut c: HashMap<T, u64> = HashMap::new();
    for x in s {
        match c.get_mut(x) {
            Some(v) => {
                *v += 1;
            }
            None => {
                c.insert(*x, 1);
            }
        }
    }
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        32T3K 765\n\
        T55J5 684\n\
        KK677 28\n\
        KTJJT 220\n\
        QQQJA 483\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 5905);
    }
}
