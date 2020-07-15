use std::io::{prelude::*};

pub fn solve() {
    let file = std::fs::File::open("data/day05.txt").unwrap();
    let input: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    // part 1
    let re_vowel = regex::Regex::new("[aeiou]").unwrap();
    let re_naughty = regex::Regex::new("ab|cd|pq|xy").unwrap();
    let result = input
        .iter()
        .filter(|l| re_vowel.find_iter(l).count() >= 3)
        .filter(|l| l.chars()
            .zip(l.chars().skip(1))
            .any(|(x, y)| x == y)
        )
        .filter(|l| !re_naughty.is_match(l))
        .count();

    println!("part 1: {}", result);

    // part 2
    let result = input
        .iter()
        .filter(|l| {
            let pairs: Vec<(char, char)> = l.chars()
                .zip(l.chars().skip(1))
                .collect();
            pairs.iter()
                .enumerate()
                .any(|(i, p)| pairs.iter().skip(i + 2).any(|q| q == p))
        })
        .filter(|l| l.chars()
            .zip(l.chars().skip(2))
            .any(|(x, y)| x == y)
        )
        .count();

    println!("part 2: {}", result);
}
