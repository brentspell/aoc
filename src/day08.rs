use std::io::prelude::*;

pub fn solve() {
    let file = std::fs::File::open("data/day08.txt").unwrap();
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    // part 1
    let result: usize = lines.iter().map(|l| l.len() - decode(l)).sum();

    println!("part 1: {}", result);

    // part 2
    let result: usize = lines.iter().map(|l| encode(l) - l.len()).sum();

    println!("part 2: {}", result);
}

fn decode(s: &str) -> usize {
    let mut c = 0;
    let mut i = 1;
    let s: Vec<char> = s.chars().collect();
    while i < s.len() - 1 {
        c += 1;
        if s[i] == '\\' {
            i += 1;
            if s[i] == 'x' {
                i += 2;
            }
        }
        i += 1;
    }
    c
}

fn encode(s: &str) -> usize {
    s.chars()
        .map(|c| (c == '"' || c == '\\') as usize + 1)
        .sum::<usize>()
        + 2
}
