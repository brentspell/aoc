use regex::Regex;
use std::collections::HashMap;

pub fn solve() {
    let data = std::fs::read_to_string("data/day01.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    // part 1
    println!("part 1: {}", part1(&lines));

    // part 2
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[&str]) -> u32 {
    lazy_static! {
        static ref REGEX: Regex = Regex::new("^[1-9]").unwrap();
    }

    process(&REGEX, lines)
}

fn part2(lines: &[&str]) -> u32 {
    lazy_static! {
        static ref REGEX: Regex =
            Regex::new("^([1-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    }

    process(&REGEX, lines)
}

fn process(regex: &Regex, lines: &[&str]) -> u32 {
    lines
        .iter()
        .map(|l| {
            // hack: ordinarily we would use a positive lookahead regex to find
            // all overlapping matches in the string, but rust regex can't do that,
            // so we just search every suffix of the string to find all matches
            let digits: Vec<_> = l
                .char_indices()
                .filter_map(|(i, _)| regex.find(&l[i..]))
                .map(|m| m.as_str())
                .collect();
            parse(digits[0]) * 10 + parse(digits[digits.len() - 1])
        })
        .sum()
}

fn parse(num: &str) -> u32 {
    lazy_static! {
        static ref MAP: HashMap<&'static str, u32> = [
            ("1", 1),
            ("2", 2),
            ("3", 3),
            ("4", 4),
            ("5", 5),
            ("6", 6),
            ("7", 7),
            ("8", 8),
            ("9", 9),
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]
        .iter()
        .cloned()
        .collect();
    }

    *MAP.get(num)
        .unwrap_or_else(|| panic!("invalid_number: {num}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        assert_eq!(part1(&data), 142);
    }

    #[test]
    fn test_part2() {
        let data = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        assert_eq!(part2(&data), 281);
    }
}
