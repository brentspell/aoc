use std::collections::HashSet;
use std::io::prelude::*;
use std::{cmp, fs, io};

pub fn solve() {
    let file = fs::File::open("data/day09.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let numbers: Vec<i64> = lines.iter().map(|l| l.parse().unwrap()).collect();

    let invalid = part1(25, &numbers);
    println!("part 1: {}", invalid);
    println!("part 2: {}", part2(invalid, &numbers));
}

fn part1(preamble: usize, numbers: &[i64]) -> i64 {
    for (i, &x) in numbers.iter().skip(preamble).enumerate() {
        let prefix = &numbers[i..][..preamble];
        let diff: HashSet<i64> = prefix.iter().map(|y| x - y).collect();
        if !prefix.iter().any(|y| diff.contains(y)) {
            return x;
        }
    }
    panic!();
}

fn part2(invalid: i64, numbers: &[i64]) -> i64 {
    for (i, &x) in numbers.iter().enumerate() {
        let mut s = x;
        let mut l = x;
        let mut h = x;
        for &x in &numbers[i + 1..] {
            s += x;
            l = cmp::min(l, x);
            h = cmp::max(h, x);
            match s.cmp(&invalid) {
                cmp::Ordering::Equal => return l + h,
                cmp::Ordering::Greater => break,
                _ => (),
            }
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let numbers = vec![
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let invalid = part1(5, &numbers);
        assert_eq!(invalid, 127);
        assert_eq!(part2(invalid, &numbers), 62);
    }
}
