use std::io::prelude::*;
use std::{fs, io};

const AMOUNT: i32 = 150;

pub fn solve() {
    let file = fs::File::open("data/day17.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let values: Vec<i32> = lines.iter().map(|l| l.parse().unwrap()).collect();

    // part 1
    let result = power(&values)
        .iter()
        .filter(|&g| g.iter().sum::<i32>() == AMOUNT)
        .count();
    println!("part 1: {}", result);

    // part 2
    let groups: Vec<Vec<i32>> = power(&values)
        .into_iter()
        .filter(|g| g.iter().sum::<i32>() == AMOUNT)
        .collect();
    let count = groups.iter().map(|g| g.len()).min().unwrap();
    let result = groups.iter().filter(|&g| g.len() == count).count();
    println!("part 2: {}", result);
}

fn power(set: &[i32]) -> Vec<Vec<i32>> {
    let mut pow = vec![vec![]];
    for &x in set {
        pow.append(
            &mut pow
                .iter()
                .map(|r| {
                    let mut r = r.clone();
                    r.push(x);
                    r
                })
                .filter(|r| r.iter().sum::<i32>() <= AMOUNT)
                .collect(),
        )
    }
    pow
}
