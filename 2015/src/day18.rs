use std::collections::HashSet;
use std::io::prelude::*;
use std::{fs, io};

pub fn solve() {
    let file = fs::File::open("data/day18.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let lights: Vec<Vec<bool>> = lines
        .iter()
        .map(|l| l.chars().map(|c| c == '#').collect())
        .collect();

    // part 1
    let mut next = lights.to_vec();
    for _ in 0..100 {
        let prev: Vec<Vec<bool>> = next.to_vec();
        for (i, r) in next.iter_mut().enumerate() {
            for (j, c) in r.iter_mut().enumerate() {
                let n = neighbors(&prev, i, j);
                if *c {
                    *c = n == 2 || n == 3;
                } else {
                    *c = n == 3;
                }
            }
        }
    }
    let result = next
        .iter()
        .map(|r| r.iter().map(|&c| c as i32).sum::<i32>())
        .sum::<i32>();
    println!("part 1: {}", result);

    // part 2
    let mut next = lights.to_vec();
    let row_max = next.len() - 1;
    let col_max = next[0].len() - 1;
    let corners: HashSet<(usize, usize)> =
        vec![(0, 0), (0, col_max), (row_max, 0), (row_max, col_max)]
            .into_iter()
            .collect();
    for &(i, j) in corners.iter() {
        next[i][j] = true;
    }
    for _ in 0..100 {
        let prev: Vec<Vec<bool>> = next.to_vec();
        for (i, r) in next.iter_mut().enumerate() {
            for (j, c) in r.iter_mut().enumerate() {
                if !corners.contains(&(i, j)) {
                    let n = neighbors(&prev, i, j);
                    if *c {
                        *c = n == 2 || n == 3;
                    } else {
                        *c = n == 3;
                    }
                }
            }
        }
    }
    let result = next
        .iter()
        .map(|r| r.iter().map(|&c| c as i32).sum::<i32>())
        .sum::<i32>();
    println!("part 2: {}", result);
}

fn neighbors(array: &[Vec<bool>], i: usize, j: usize) -> i32 {
    let mut n = 0;
    if i > 0 {
        if j > 0 && array[i - 1][j - 1] {
            n += 1;
        }
        if array[i - 1][j] {
            n += 1;
        }
        if j < array[i].len() - 1 && array[i - 1][j + 1] {
            n += 1;
        }
    }
    if j > 0 && array[i][j - 1] {
        n += 1;
    }
    if j < array[i].len() - 1 && array[i][j + 1] {
        n += 1;
    }
    if i < array.len() - 1 {
        if j > 0 && array[i + 1][j - 1] {
            n += 1;
        }
        if array[i + 1][j] {
            n += 1;
        }
        if j < array[i].len() - 1 && array[i + 1][j + 1] {
            n += 1;
        }
    }
    n
}
