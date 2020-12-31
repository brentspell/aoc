use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs, io};

lazy_static! {
    static ref MASK_REGEX: Regex = Regex::new(r#"^mask = (?P<mask>[01X]{36})$"#).unwrap();
    static ref WRITE_REGEX: Regex =
        Regex::new(r#"^mem\[(?P<addr>\d+)\] = (?P<value>\d+)$"#).unwrap();
}

enum Op {
    Mask { base: u64, mask: u64 },
    Write { addr: u64, value: u64 },
}

pub fn solve() {
    let file = fs::File::open("data/day14.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let program: Vec<Op> = lines.iter().map(|l| parse(&l)).collect();

    println!("part 1: {}", part1(&program));
    println!("part 2: {}", part2(&program));
}

fn part1(program: &[Op]) -> u64 {
    let mut base = 0;
    let mut mask = 0;
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for op in program {
        match *op {
            Op::Mask { base: b, mask: m } => {
                base = b;
                mask = m;
            }
            Op::Write { addr: a, value: v } => {
                memory.insert(a, base | (v & mask));
            }
        }
    }

    memory.values().sum()
}

fn part2(program: &[Op]) -> u64 {
    let mut base = 0;
    let mut mask = 0;
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for op in program {
        match *op {
            Op::Mask { base: b, mask: m } => {
                base = b;
                mask = m;
            }
            Op::Write { addr: a, value: v } => {
                let mut addrs = vec![a | base];
                for b in (0..36).map(|i| 1 << i).filter(|&b| mask & b != 0) {
                    for i in 0..addrs.len() {
                        let a = addrs[i];
                        addrs.push(a | b);
                        addrs.push(a & !b);
                    }
                }
                for a in addrs {
                    memory.insert(a, v);
                }
            }
        }
    }

    memory.values().sum()
}

fn parse(line: &str) -> Op {
    if let Some(caps) = MASK_REGEX.captures(line) {
        let mask = caps.name("mask").unwrap().as_str();
        Op::Mask {
            base: u64::from_str_radix(&mask.replace("X", "0"), 2).unwrap(),
            mask: u64::from_str_radix(&mask.replace("1", "0").replace("X", "1"), 2).unwrap(),
        }
    } else {
        let caps = WRITE_REGEX.captures(line).unwrap();
        Op::Write {
            addr: caps.name("addr").unwrap().as_str().parse().unwrap(),
            value: caps.name("value").unwrap().as_str().parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let lines = vec![
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
            "mem[8] = 11",
            "mem[7] = 101",
            "mem[8] = 0",
        ];
        let program: Vec<Op> = lines.into_iter().map(parse).collect();

        assert_eq!(part1(&program), 165);

        let lines = vec![
            "mask = 000000000000000000000000000000X1001X",
            "mem[42] = 100",
            "mask = 00000000000000000000000000000000X0XX",
            "mem[26] = 1",
        ];
        let program: Vec<Op> = lines.into_iter().map(parse).collect();

        assert_eq!(part2(&program), 208);
    }
}
