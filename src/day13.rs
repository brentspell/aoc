use std::io::prelude::*;
use std::{fs, io};

pub fn solve() {
    let file = fs::File::open("data/day13.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let (start, buses) = parse(&lines);

    println!("part 1: {}", part1(start, &buses));
    println!("part 2: {}", part2(start, &buses));
}

fn part1(start: u64, buses: &[u64]) -> u64 {
    let (wait, bus) = buses
        .iter()
        .filter(|&&b| b != 0)
        .map(|b| (b - start % b, b))
        .min()
        .unwrap();
    wait * bus
}

// sieve method for solving the chinese remainder theorem
// find x, such that x % ni == (ni - ai) for all ni, ai
// the ni are the bus numbers
// the ai are their positions in the list (time offsets)
// find the remainder mod (ni - ai), since that is the offset from the start time of bus 0
fn part2(_start: u64, buses: &[u64]) -> u64 {
    let buses: Vec<(u64, u64)> = buses
        .iter()
        .copied()
        .enumerate()
        .filter(|&(_, x)| x != 0)
        .map(|(i, x)| (x, i as u64))
        .collect();
    let (mut n, mut x) = (1, 0);
    for &(ni, ai) in &buses {
        while x % ni != (ni - ai % ni) % ni {
            x += n;
        }
        n *= ni;
    }
    x
}

fn parse(lines: &[String]) -> (u64, Vec<u64>) {
    let start: u64 = lines[0].parse().unwrap();
    let buses: Vec<u64> = lines[1]
        .split(',')
        .map(|s| if s == "x" { "0" } else { s })
        .map(|s| s.parse().unwrap())
        .collect();
    (start, buses)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let lines = [String::from("939"), String::from("7,13,x,x,59,x,31,19")];
        let (start, buses) = parse(&lines);

        assert_eq!(part1(start, &buses), 295);
        assert_eq!(part2(start, &buses), 1068781);
    }
}
