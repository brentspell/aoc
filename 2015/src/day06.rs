use std::cmp::max;
use std::io::prelude::*;

struct Operation {
    code: u32,
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}

pub fn solve() {
    let re =
        regex::Regex::new(r"(turn off|turn on|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    let file = std::fs::File::open("data/day06.txt").unwrap();
    let input: Vec<Operation> = std::io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let c = re.captures(&l).unwrap();
            let code = match &c[1] {
                "turn off" => 0,
                "turn on" => 1,
                "toggle" => 2,
                _ => panic!(),
            };
            Operation {
                code,
                x1: c[2].parse().unwrap(),
                y1: c[3].parse().unwrap(),
                x2: c[4].parse().unwrap(),
                y2: c[5].parse().unwrap(),
            }
        })
        .collect();

    // part 1
    let mut lights = [[false; 1000]; 1000];
    for op in input.iter() {
        for light in lights.iter_mut().take(op.x2 + 1).skip(op.x1) {
            for light in light.iter_mut().take(op.y2 + 1).skip(op.y1) {
                match op.code {
                    0 => *light = false,
                    1 => *light = true,
                    2 => *light = !*light,
                    _ => panic!(),
                }
            }
        }
    }

    let result: i32 = lights
        .iter()
        .map(|r| r.iter().map(|c| *c as i32).sum::<i32>())
        .sum();

    println!("part 1: {}", result);

    // part 2
    let mut lights = [[0; 1000]; 1000];
    for op in input.iter() {
        for light in lights.iter_mut().take(op.x2 + 1).skip(op.x1) {
            for light in light.iter_mut().take(op.y2 + 1).skip(op.y1) {
                match op.code {
                    0 => *light = max(*light - 1, 0),
                    1 => *light += 1,
                    2 => *light += 2,
                    _ => panic!(),
                }
            }
        }
    }

    let result: i32 = lights
        .iter()
        .map(|r| r.iter().map(|c| *c as i32).sum::<i32>())
        .sum();

    println!("part 2: {}", result);
}
