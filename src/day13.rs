use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::{fs, io};

lazy_static! {
    static ref REGEX: Regex =
        Regex::new(r#"^(?P<from>[\w]+?) would (?P<op>(gain|lose)) (?P<number>[\d]+?) happiness units by sitting next to (?P<to>[\w]+?).$"#).unwrap();
}

pub fn solve() {
    let file = fs::File::open("data/day13.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let inputs: Vec<(&str, &str, i64)> = lines.iter().map(|l| parse(&l)).collect();
    let people: HashSet<&str> = inputs.iter().map(|&(f, _t, _d)| f).collect();
    let costs: HashMap<(&str, &str), i64> = inputs.iter().map(|&(f, t, d)| ((f, t), d)).collect();

    // part 1
    let result = search("Alice", "Alice", 0, &without(&people, "Alice"), &costs);
    println!("part 1: {}", result);

    // part 2
    let result = search("Brent", "Brent", 0, &people, &costs);
    println!("part 2: {}", result);
}

fn parse(line: &str) -> (&str, &str, i64) {
    let caps = REGEX.captures(line).unwrap();
    let sign = if caps.name("op").unwrap().as_str() == "gain" {
        1
    } else {
        -1
    };
    (
        caps.name("from").unwrap().as_str(),
        caps.name("to").unwrap().as_str(),
        sign * caps
            .name("number")
            .unwrap()
            .as_str()
            .parse::<i64>()
            .unwrap(),
    )
}

fn search<'a>(
    root: &'a str,
    elem: &'a str,
    cost: i64,
    people: &HashSet<&'a str>,
    costs: &HashMap<(&'a str, &'a str), i64>,
) -> i64 {
    if people.is_empty() {
        cost + costs.get(&(elem, root)).unwrap_or(&0) + costs.get(&(root, elem)).unwrap_or(&0)
    } else {
        people
            .iter()
            .map(|&next| {
                search(
                    root,
                    next,
                    cost + costs.get(&(elem, next)).unwrap_or(&0)
                        + costs.get(&(next, elem)).unwrap_or(&0),
                    &without(people, next),
                    costs,
                )
            })
            .max()
            .unwrap()
    }
}

fn without<'a>(set: &HashSet<&'a str>, elem: &'a str) -> HashSet<&'a str> {
    let mut set = set.clone();
    set.remove(elem);
    set
}
