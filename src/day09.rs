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
    let locations: Vec<&str> = inputs
        .iter()
        .flat_map(|&(f, t, _d)| vec![f, t])
        .collect::<HashSet<&str>>()
        .into_iter()
        .collect();
    let costs: HashMap<(&str, &str), u32> = inputs
        .iter()
        .flat_map(|&(f, t, d)| vec![((f, t), d), ((t, f), d)])
        .collect();

    // part 1
    let result = optimize(&locations, &costs, |a, b| a.cmp(b));
    println!("part 1: {}", result);

    // part 2
    let result = optimize(&locations, &costs, |a, b| b.cmp(a));
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

fn optimize(
    locations: &[&str],
    costs: &HashMap<(&str, &str), u32>,
    comparer: fn(&u32, &u32) -> cmp::Ordering,
) -> u32 {
    permutohedron::Heap::new(&mut locations.to_vec())
        .map(|p| {
            p.iter()
                .zip(p.iter().skip(1))
                .map(|(a, b)| costs.get(&(a, b)).unwrap())
                .sum()
        })
        .min_by(comparer)
        .unwrap()
}
