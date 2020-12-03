use std::io::prelude::*;
use std::{cmp, fs, io};

pub fn solve() {
    let file = fs::File::open("data/day24.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let gifts: Vec<i64> = lines.iter().map(|l| l.parse().unwrap()).collect();

    // part 1
    let max = gifts.iter().sum::<i64>() / 3;
    let mut length = 0;
    let result = loop {
        // find all combinations of length n that sum to the expected weight, sort by entanglement
        let mut passes = fixed_combos(&gifts, length, max);
        passes.sort_by_key(|c| c.iter().product::<i64>());
        // if there is a combination of the remaining gifts with the expected weight, we are done
        if let Some(pass) = passes.iter().find(|p| any_combos(&diff(&gifts, &p), max)) {
            break pass.iter().product::<i64>();
        }
        length += 1;
    };
    println!("part 1: {}", result);

    // part 2
    let max = gifts.iter().sum::<i64>() / 4;
    let mut length = 0;
    let mut result = 0;
    while result == 0 {
        // find all combinations of length n that sum to the expected weight, sort by entanglement
        let mut passes = fixed_combos(&gifts, length, max);
        passes.sort_by_key(|c| c.iter().product::<i64>());
        for pass in &passes {
            // next, find all combinations of any length from the remaining gifts
            // if there is third combination with the same weight, we are done
            let gifts = diff(&gifts, &pass);
            let next = all_combos(&gifts, max);
            if next.iter().any(|p| any_combos(&diff(&gifts, &p), max)) {
                result = pass.iter().product();
                break;
            }
        }
        length += 1;
    }
    println!("part 2: {}", result);
}

fn all_combos(set: &[i64], max: i64) -> Vec<Vec<i64>> {
    match max.cmp(&0) {
        cmp::Ordering::Equal => vec![vec![]],
        cmp::Ordering::Less => vec![],
        _ => {
            let mut vec: Vec<Vec<i64>> = Vec::new();
            for i in 0..set.len() {
                for v in all_combos(&set[i + 1..], max - set[i]) {
                    let mut v2: Vec<i64> = Vec::with_capacity(v.len() + 1);
                    v2.push(set[i]);
                    v2.extend_from_slice(&v);
                    vec.push(v2);
                }
            }
            vec
        }
    }
}

fn fixed_combos(set: &[i64], length: usize, max: i64) -> Vec<Vec<i64>> {
    if max == 0 && length == 0 {
        vec![vec![]]
    } else if max < 0 || length == 0 {
        vec![]
    } else {
        let mut vec: Vec<Vec<i64>> = Vec::new();
        for i in 0..set.len() {
            for v in fixed_combos(&set[i + 1..], length - 1, max - set[i]) {
                let mut v2: Vec<i64> = Vec::with_capacity(v.len() + 1);
                v2.push(set[i]);
                v2.extend_from_slice(&v);
                vec.push(v2);
            }
        }
        vec
    }
}

fn any_combos(set: &[i64], max: i64) -> bool {
    match max.cmp(&0) {
        cmp::Ordering::Equal => true,
        cmp::Ordering::Less => false,
        _ => {
            for i in 0..set.len() {
                if any_combos(&set[i + 1..], max - set[i]) {
                    return true;
                }
            }
            false
        }
    }
}

fn diff(set1: &[i64], set2: &[i64]) -> Vec<i64> {
    set1.iter().copied().filter(|i| !set2.contains(i)).collect()
}
