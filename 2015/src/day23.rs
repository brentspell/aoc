use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs, io};

pub fn solve() {
    let file = fs::File::open("data/day23.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    // part 1
    let result = simulate(&lines, &vec![("a", 0), ("b", 0)].into_iter().collect())["b"];
    println!("part 1: {}", result);

    // part 2
    let result = simulate(&lines, &vec![("a", 1), ("b", 0)].into_iter().collect())["b"];
    println!("part 1: {}", result);
}

fn simulate<'a>(
    program: &'a [String],
    registers: &'a HashMap<&'a str, i32>,
) -> HashMap<&'a str, i32> {
    let mut registers = registers.clone();
    let mut i = 0_i32;
    while i >= 0 && i < program.len() as i32 {
        match program[i as usize]
            .split(' ')
            .collect::<Vec<&str>>()
            .as_slice()
        {
            ["hlf", r] => {
                *registers.get_mut(r).unwrap() /= 2;
                i += 1;
            }
            ["tpl", r] => {
                *registers.get_mut(r).unwrap() *= 3;
                i += 1;
            }
            ["inc", r] => {
                *registers.get_mut(r).unwrap() += 1;
                i += 1;
            }
            ["jmp", offset] => {
                i += offset.parse::<i32>().unwrap();
            }
            ["jie", r, offset] => {
                i += if registers[&r[..1]] % 2 == 0 {
                    offset.parse::<i32>().unwrap()
                } else {
                    1
                }
            }
            ["jio", r, offset] => {
                i += if registers[&r[..1]] == 1 {
                    offset.parse::<i32>().unwrap()
                } else {
                    1
                }
            }
            _ => panic!("{}", program[i as usize]),
        }
    }
    registers
}
