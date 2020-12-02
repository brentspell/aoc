use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::{fs, io};

pub fn solve() {
    let file = fs::File::open("data/day19.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let rules: Vec<(&str, &str)> = lines[..lines.len() - 2]
        .iter()
        .map(|l| {
            let s: Vec<&str> = l.split(" => ").collect();
            (s[0], s[1])
        })
        .collect();
    let formula = lines.last().unwrap();

    // part 1
    let mut result: HashSet<String> = HashSet::new();
    for (source, target) in rules.iter() {
        for repl in replacements(formula, source, target) {
            result.insert(repl);
        }
    }

    let result = result.len();
    println!("part 1: {}", result);

    // part 2
    let map: HashMap<&str, &str> = rules.iter().map(|&(s, t)| (t, s)).collect();
    let mut keys: Vec<&str> = map.keys().cloned().collect();
    let mut stack: Vec<(i32, String)> = Vec::new();

    // greedy DFS longest match first from formula back to e
    keys.sort_by_key(|a| a.len());
    stack.push((0, formula.clone()));
    let result = loop {
        let (depth, formula) = stack.pop().unwrap();
        if formula == "e" {
            break depth;
        }
        for source in keys.iter() {
            for repl in replacements(&formula, source, map[source]) {
                stack.push((depth + 1, repl));
            }
        }
    };

    println!("part 2: {}", result);
}

fn replacements(formula: &str, source: &str, target: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut i = 0;
    while let Some(j) = formula[i..].find(source) {
        let mut repl = String::new();
        i += j;
        repl.push_str(&formula[..i]);
        repl.push_str(target);
        i += source.len();
        repl.push_str(&formula[i..]);
        result.push(repl);
    }
    result
}
