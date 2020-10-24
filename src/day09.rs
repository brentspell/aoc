use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::{cmp, fs, io};

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r#"^(?P<from>[\w]+) to (?P<to>[\w]+) = (?P<distance>.+)$"#).unwrap();
}

pub fn solve() {
    let file = fs::File::open("data/day09.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let inputs: Vec<(&str, &str, u32)> = lines.iter().map(|l| parse(&l)).collect();
    let locations: HashSet<&str> = inputs.iter().flat_map(|&(f, t, _d)| vec![f, t]).collect();
    let costs: HashMap<(&str, &str), u32> = inputs
        .iter()
        .flat_map(|&(f, t, d)| vec![((f, t), d), ((t, f), d)])
        .collect();

    // part 1
    let result = locations
        .iter()
        .map(|l| route(l, 0, &without(&locations, l), &costs, |a, b| a.cmp(b)))
        .min()
        .unwrap();
    println!("part 1: {}", result);

    // part 2
    let result = locations
        .iter()
        .map(|l| route(l, 0, &without(&locations, l), &costs, |a, b| b.cmp(a)))
        .max()
        .unwrap();
    println!("part 2: {}", result);
}

fn parse(line: &str) -> (&str, &str, u32) {
    let caps = REGEX.captures(line).unwrap();
    (
        caps.name("from").unwrap().as_str(),
        caps.name("to").unwrap().as_str(),
        caps.name("distance").unwrap().as_str().parse().unwrap(),
    )
}

fn route<'a>(
    start: &'a str,
    cost: u32,
    locations: &HashSet<&'a str>,
    costs: &HashMap<(&'a str, &'a str), u32>,
    comparer: fn(&u32, &u32) -> cmp::Ordering,
) -> u32 {
    if locations.is_empty() {
        cost
    } else {
        locations
            .iter()
            .map(|&l| {
                route(
                    l,
                    cost + costs[&(start, l)],
                    &without(locations, l),
                    costs,
                    comparer,
                )
            })
            .min_by(comparer)
            .unwrap()
    }
}

fn without<'a>(set: &HashSet<&'a str>, elem: &'a str) -> HashSet<&'a str> {
    let mut set = set.clone();
    set.remove(elem);
    set
}
