use std::cmp::max;
use std::io::prelude::*;

pub fn solve() {
    let re =
        regex::Regex::new(r"(turn off|turn on|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    let file = std::fs::File::open("data/day06.txt").unwrap();
    let input: Vec<(i32, (usize, usize), (usize, usize))> = std::io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .map(|l| {
            let c = re.captures(&l).unwrap();
            let op = match &c[1] {
                "turn off" => 0,
                "turn on" => 1,
                "toggle" => 2,
                _ => panic!(),
            };
            (
                op,
                (c[2].parse().unwrap(), c[3].parse().unwrap()),
                (c[4].parse().unwrap(), c[5].parse().unwrap()),
            )
        })
        .collect();

    // part 1
    let mut lights = [[false; 1000]; 1000];
    for (op, (x1, y1), (x2, y2)) in input.iter() {
        for i in *x1..=*x2 {
            for j in *y1..=*y2 {
                match op {
                    0 => lights[i][j] = false,
                    1 => lights[i][j] = true,
                    2 => lights[i][j] = !lights[i][j],
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
    for (op, (x1, y1), (x2, y2)) in input.iter() {
        for i in *x1..=*x2 {
            for j in *y1..=*y2 {
                match op {
                    0 => lights[i][j] = max(lights[i][j] - 1, 0),
                    1 => lights[i][j] += 1,
                    2 => lights[i][j] += 2,
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
