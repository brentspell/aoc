use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs, io};

const FIELDS: &[&str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

lazy_static! {
    static ref HGT_REGEX: Regex = Regex::new(r#"^(?P<value>[\d]+)(?P<unit>(cm|in))$"#).unwrap();
    static ref HCL_REGEX: Regex = Regex::new(r#"^#[0-9a-f]{6}$"#).unwrap();
    static ref ECL_REGEX: Regex = Regex::new(r#"^(amb|blu|brn|gry|grn|hzl|oth)$"#).unwrap();
    static ref PID_REGEX: Regex = Regex::new(r#"^[0-9]{9}$"#).unwrap();
}

pub fn solve() {
    let file = fs::File::open("data/day04.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let mut passports = vec![HashMap::new()];
    for line in &lines {
        if line == "" {
            passports.push(HashMap::new());
        } else {
            let i = passports.len() - 1;
            for field in line.split(' ') {
                let field: Vec<&str> = field.split(':').collect();
                passports[i].insert(field[0], field[1]);
            }
        }
    }

    // part 1
    let result = passports
        .iter()
        .filter(|p| FIELDS.iter().all(|k| p.contains_key(k)))
        .count();
    println!("part 1: {}", result);

    // part 2
    let result = passports
        .iter()
        .filter(|p| FIELDS.iter().all(|k| p.contains_key(k)))
        .filter(|p| p.iter().all(|(k, v)| validate(k, v)))
        .count();
    println!("part 2: {}", result);
}

fn validate(key: &str, value: &str) -> bool {
    match key {
        "byr" => value
            .parse::<u32>()
            .map_or(false, |v| v >= 1920 && v <= 2002),
        "iyr" => value
            .parse::<u32>()
            .map_or(false, |v| v >= 2010 && v <= 2020),
        "eyr" => value
            .parse::<u32>()
            .map_or(false, |v| v >= 2020 && v <= 2030),
        "hgt" => HGT_REGEX.captures(value).map_or(false, |caps| {
            let value = caps
                .name("value")
                .map_or("", |v| v.as_str())
                .parse()
                .unwrap_or(0);
            match caps.name("unit").map_or("", |v| v.as_str()) {
                "cm" => value >= 150 && value <= 193,
                "in" => value >= 59 && value <= 76,
                _ => false,
            }
        }),
        "hcl" => HCL_REGEX.is_match(value),
        "ecl" => ECL_REGEX.is_match(value),
        "pid" => PID_REGEX.is_match(value),
        "cid" => true,
        _ => false,
    }
}
