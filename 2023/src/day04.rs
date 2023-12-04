use std::collections::{HashMap, HashSet};

pub fn solve() {
    let data = std::fs::read_to_string("data/day04.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    // part 1
    println!("part 1: {}", part1(&lines));

    // part 2
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[&str]) -> u32 {
    lines
        .iter()
        .map(|&line| {
            // parse the card line
            let s: Vec<_> = line[7..].split('|').collect();

            // compute the number of matches as before
            let win: HashSet<_> = s[0].split_whitespace().collect();
            let own: HashSet<_> = s[1].split_whitespace().collect();
            let matches = win.intersection(&own).count() as u32;

            // calculate the points for the current card
            if matches == 0 {
                0
            } else {
                2_u32.pow(matches - 1)
            }
        })
        .sum()
}

fn part2(lines: &[&str]) -> u32 {
    // maintain the current number of repetitions of each line
    let mut counts: HashMap<u32, u32> = HashMap::new();

    lines
        .iter()
        .map(|&line| {
            // parse the card line
            let s: Vec<_> = line[5..].split(':').collect();
            let i = s[0].trim().parse().unwrap();
            let s: Vec<_> = s[1].split('|').collect();

            // compute the number of matches as before
            let win: HashSet<_> = s[0].split_whitespace().collect();
            let own: HashSet<_> = s[1].split_whitespace().collect();
            let matches = win.intersection(&own).count() as u32;

            // add n instances of every line after the current line
            let n = *counts.get(&i).unwrap_or(&1);
            for j in 0..matches {
                *counts.entry(i + 1 + j).or_insert(1) += n;
            }

            n
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &[&str] = &[
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
        "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
        "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
        "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
        "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
        "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
    ];

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 30);
    }
}
