use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::{fs, io};

lazy_static! {
    static ref REGEX: Regex = Regex::new(r#"(?P<count>[\d]+) (?P<name>.+?) bags?"#).unwrap();
}

pub fn solve() {
    let file = fs::File::open("data/day07.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let mapping: HashMap<&str, Vec<(&str, u32)>> = lines.iter().map(|l| parse(&l)).collect();
    let mut reverse: HashMap<&str, Vec<&str>> = HashMap::new();
    for (source, target) in &mapping {
        for (target, _count) in target {
            reverse.entry(target).or_insert(vec![]).push(source);
        }
    }

    // part 1
    let mut result: HashSet<&str> = HashSet::new();
    let mut stack = vec!["shiny gold"];
    while !stack.is_empty() {
        let source = stack.pop().unwrap();
        for &target in reverse.get(source).unwrap_or(&vec![]) {
            result.insert(target);
            stack.push(target);
        }
    }
    println!("part 1: {}", result.len());

    // part 2
    let mut result = 0;
    let mut stack = vec![("shiny gold", 1)];
    while !stack.is_empty() {
        let (source, factor) = stack.pop().unwrap();
        for &(target, count) in mapping.get(source).unwrap_or(&vec![]) {
            result += factor * count;
            stack.push((target, factor * count));
        }
    }
    println!("part 2: {}", result);
}

fn parse(line: &str) -> (&str, Vec<(&str, u32)>) {
    let split: Vec<&str> = line.split(" bags contain ").collect();
    let source = split[0];
    let mut target: Vec<(&str, u32)> = Vec::new();
    if split[1] != "no other bags." {
        for part in split[1].split(", ") {
            let caps = REGEX.captures(part).unwrap();
            let name = caps.name("name").unwrap().as_str();
            let count = caps.name("count").unwrap().as_str().parse::<u32>().unwrap();
            target.push((name, count));
        }
    }
    (source, target)
}
