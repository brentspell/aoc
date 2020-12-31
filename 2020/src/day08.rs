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

    println!("part 1: {}", part1(&program));
    println!("part 2: {}", part2(&program));
}

fn part1(program: &[(&str, i32)]) -> i32 {
    let (_ip, result) = execute(&program);
    result
}

fn part2(program: &[(&str, i32)]) -> i32 {
    for fix in 0..program.len() {
        let mut program: Vec<(&str, i32)> = program.to_vec();
        let (ref mut op, _arg) = program[fix];
        match *op {
            "jmp" => *op = "nop",
            "nop" => *op = "jmp",
            _ => continue,
        }

        let (ip, ac) = execute(&program);
        if ip == program.len() {
            return ac;
        }
    }
    panic!();
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

fn parse(line: &str) -> (&str, i32) {
    let caps = REGEX.captures(line).unwrap();
    (
        caps.name("op").unwrap().as_str(),
        caps.name("arg").unwrap().as_str().parse().unwrap(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let lines = vec![
            "nop +0", "acc +1", "jmp +4", "acc +3", "jmp -3", "acc -99", "acc +1", "jmp -4",
            "acc +6",
        ];
        let program: Vec<(&str, i32)> = lines.into_iter().map(parse).collect();

        assert_eq!(part1(&program), 5);
        assert_eq!(part2(&program), 8);
    }
}
