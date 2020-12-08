use regex::Regex;
use std::collections::HashSet;
use std::io::prelude::*;
use std::{fs, io};

lazy_static! {
    static ref REGEX: Regex = Regex::new(r#"^(?P<op>[\w]+?) (?P<arg>(\+|-)[\d]+)$"#).unwrap();
}

pub fn solve() {
    let file = fs::File::open("data/day08.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let program: Vec<(&str, i32)> = lines.iter().map(|l| parse(&l)).collect();

    // part 1
    let (_ip, result) = execute(&program);
    println!("part 1: {}", result);

    // part 2
    let mut result = 0;
    for fix in 0..program.len() {
        let mut program: Vec<(&str, i32)> = program.clone();
        let (ref mut op, _arg) = program[fix];
        match *op {
            "jmp" => {
                *op = "nop";
            }
            "nop" => {
                *op = "jmp";
            }
            _ => {
                continue;
            }
        }

        let (ip, ac) = execute(&program);
        if ip == program.len() {
            result = ac;
            break;
        }
    }
    println!("part 2: {}", result);
}

fn parse(line: &str) -> (&str, i32) {
    let caps = REGEX.captures(line).unwrap();
    (
        caps.name("op").unwrap().as_str(),
        caps.name("arg").unwrap().as_str().parse().unwrap(),
    )
}

fn execute(program: &[(&str, i32)]) -> (usize, i32) {
    let mut ip = 0;
    let mut ac = 0;
    let mut done: HashSet<usize> = HashSet::new();
    while !done.contains(&ip) && ip < program.len() {
        done.insert(ip);

        let (op, arg) = program[ip];
        match op {
            "acc" => {
                ac += arg;
                ip += 1;
            }
            "jmp" => {
                ip = (ip as i32 + arg) as usize;
            }
            "nop" => {
                ip += 1;
            }
            _ => panic!(),
        }
    }

    (ip, ac)
}
