use std::cmp::*;
use std::io::prelude::*;

pub fn solve() {
    let file = std::fs::File::open("data/day02.txt").unwrap();
    let input: Vec<(i32, i32, i32)> = std::io::BufReader::new(file)
        .lines()
        .map(|l| {
            let v: Vec<i32> = l
                .unwrap()
                .split('x')
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            match v.len() {
                3 => (v[0], v[1], v[2]),
                _ => panic!(),
            }
        })
        .collect();

    // part 1
    let result: i32 = input
        .iter()
        .map(|(l, w, h)| {
            let lw = l * w;
            let wh = w * h;
            let hl = h * l;
            2 * lw + 2 * wh + 2 * hl + min(min(lw, wh), hl)
        })
        .sum();
    println!("part 1: {}", result);

    // part 2
    let result: i32 = input
        .iter()
        .map(|(l, w, h)| {
            let lw = 2 * l + 2 * w;
            let wh = 2 * w + 2 * h;
            let hl = 2 * h + 2 * l;
            min(min(lw, wh), hl) + l * w * h
        })
        .sum();
    println!("part 2: {}", result);
}
