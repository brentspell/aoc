use cached::proc_macro::cached;
use std::io::prelude::*;
use std::{fs, io};

pub fn solve() {
    let file = fs::File::open("data/day10.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let mut numbers: Vec<u64> = lines.iter().map(|l| l.parse().unwrap()).collect();
    numbers.insert(0, 0);
    numbers.sort_unstable();

    println!("part 1: {}", part1(&numbers));
    println!("part 2: {}", part2(&numbers));
}

fn part1(numbers: &[u64]) -> u64 {
    let mut d1 = 0;
    let mut d3 = 1;
    let mut x = 0;
    for y in permute1(numbers) {
        let d = y - x;
        if d == 1 {
            d1 += 1;
        } else if d == 3 {
            d3 += 1;
        }
        x = y;
    }
    d1 * d3
}

fn part2(numbers: &[u64]) -> u64 {
    permute2(numbers)
}

fn permute1(numbers: &[u64]) -> Vec<u64> {
    for (i, &x) in numbers.iter().skip(1).enumerate() {
        if numbers[0] < x && x <= numbers[0] + 3 {
            let mut v = vec![x];
            v.extend(&permute1(&numbers[i + 1..]));
            return v;
        }
    }
    vec![]
}

#[cached(key = "Vec<u64>", convert = "{numbers.to_vec()}")]
fn permute2(numbers: &[u64]) -> u64 {
    let mut count = 0;
    if numbers.len() == 1 {
        count = 1;
    } else {
        for (i, &x) in numbers[1..].iter().enumerate() {
            if numbers[0] < x && x <= numbers[0] + 3 {
                count += permute2(&numbers[i + 1..]);
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut numbers = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        numbers.insert(0, 0);
        numbers.sort_unstable();

        assert_eq!(part1(&numbers), 35);
        assert_eq!(part2(&numbers), 8);

        let mut numbers = vec![
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        numbers.insert(0, 0);
        numbers.sort_unstable();
        assert_eq!(part1(&numbers), 220);
        assert_eq!(part2(&numbers), 19208);
    }
}
