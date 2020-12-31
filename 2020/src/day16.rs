use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::{fs, io, ops};

#[derive(Debug)]
struct Input {
    fields: HashMap<String, Vec<ops::RangeInclusive<u64>>>,
    ticket: Vec<u64>,
    nearby: Vec<Vec<u64>>,
}

lazy_static! {
    static ref FIELD_REGEX: Regex = Regex::new(
        r#"^(?P<name>.*?): (?P<start1>\d+)-(?P<stop1>\d+) or (?P<start2>\d+)-(?P<stop2>\d+)$"#
    )
    .unwrap();
}

pub fn solve() {
    let file = fs::File::open("data/day16.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let inputs = parse(&lines);

    println!("part 1: {}", part1(&inputs));
    println!("part 2: {}", part2(&inputs));
}

fn part1(inputs: &Input) -> u64 {
    inputs
        .nearby
        .iter()
        .flatten()
        .filter(|&value| {
            !inputs
                .fields
                .values()
                .flatten()
                .any(|range| range.contains(&value))
        })
        .sum()
}

fn part2(inputs: &Input) -> u64 {
    // ignore any invalid tickets
    let nearby: Vec<&Vec<u64>> = inputs
        .nearby
        .iter()
        .filter(|nearby| {
            nearby.iter().all(|value| {
                inputs
                    .fields
                    .values()
                    .flatten()
                    .any(|range| range.contains(&value))
            })
        })
        .collect();

    // construct a list of valid fields at each position
    // incrementally filter the fields down to those that are valid for all nearby tickets
    let keys: Vec<&String> = inputs.fields.keys().collect();
    let mut constraints: Vec<HashSet<&String>> =
        vec![keys.iter().copied().collect(); inputs.fields.len()];
    let mut removed = true;
    while removed {
        removed = false;
        for nearby in &nearby {
            for (i, value) in nearby.iter().enumerate() {
                for key in constraints[i].clone() {
                    if !inputs.fields[key]
                        .iter()
                        .any(|range| range.contains(&value))
                    {
                        constraints[i].remove(key);
                        removed = true;
                    }
                }
            }
        }
    }

    // use the constraints to isolate each position to a single field
    // collect the product of all fields that start with "departure"
    let mut done: HashSet<&String> = HashSet::new();
    let mut result = 1;
    while done.len() < keys.len() {
        for i in 0..constraints.len() {
            if constraints[i].len() == 1 {
                let single = *constraints[i].iter().next().unwrap();
                if !done.contains(single) {
                    done.insert(single);

                    if single.starts_with("departure ") {
                        result *= inputs.ticket[i];
                    }

                    for (j, constraint) in constraints.iter_mut().enumerate() {
                        if j != i {
                            constraint.remove(single);
                        }
                    }
                }
            }
        }
    }

    result
}

fn parse(lines: &[String]) -> Input {
    let splits: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(_i, l)| l.as_str() == "")
        .map(|(i, _l)| i)
        .collect();
    let fields = &lines[..splits[0]];
    let ticket = &lines[splits[0] + 2];
    let nearby = &lines[splits[1] + 2..];

    Input {
        fields: fields
            .iter()
            .map(|l| {
                let caps = FIELD_REGEX.captures(&l).unwrap();
                (
                    caps.name("name").unwrap().as_str().to_string(),
                    vec![
                        caps.name("start1").unwrap().as_str().parse().unwrap()
                            ..=caps.name("stop1").unwrap().as_str().parse().unwrap(),
                        caps.name("start2").unwrap().as_str().parse().unwrap()
                            ..=caps.name("stop2").unwrap().as_str().parse().unwrap(),
                    ],
                )
            })
            .collect(),
        ticket: ticket.split(',').map(|s| s.parse().unwrap()).collect(),
        nearby: nearby
            .iter()
            .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let lines = vec![
            String::from("class: 1-3 or 5-7"),
            String::from("departure row: 6-11 or 33-44"),
            String::from("departure seat: 13-40 or 45-50"),
            String::from(""),
            String::from("your ticket:"),
            String::from("7,1,14"),
            String::from(""),
            String::from("nearby tickets:"),
            String::from("7,3,47"),
            String::from("40,4,50"),
            String::from("55,2,20"),
            String::from("38,6,12"),
        ];
        let inputs = parse(&lines);

        assert_eq!(part1(&inputs), 71);
        assert_eq!(part2(&inputs), 7 * 14);
    }
}
