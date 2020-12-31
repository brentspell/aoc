use std::collections::HashSet;
use std::io::prelude::*;
use std::{fs, io, iter};

pub fn solve() {
    let file = fs::File::open("data/day06.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .chain(iter::once("".to_string()))
        .collect();

    // part 1
    let no_answers: HashSet<char> = HashSet::new();
    let result = lines
        .iter()
        .fold((0, no_answers.clone()), |(c, a), l| {
            if l == "" {
                (c + a.len(), no_answers.clone())
            } else {
                (c, a.union(&l.chars().collect()).copied().collect())
            }
        })
        .0;
    println!("part 1: {}", result);

    // part 2
    let all_answers: HashSet<char> = ('a'..='z').into_iter().collect();
    let result = lines
        .iter()
        .fold((0, all_answers.clone()), |(c, a), l| {
            if l == "" {
                (c + a.len(), all_answers.clone())
            } else {
                (c, a.intersection(&l.chars().collect()).copied().collect())
            }
        })
        .0;
    println!("part 2: {}", result);
}
