use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs, io};

pub fn solve() {
    let file = fs::File::open("data/day11.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let seats = parse(&lines.iter().map(|l| l.as_str()).collect::<Vec<&str>>());

    println!("part 1: {}", part1(&seats));
    println!("part 2: {}", part2(&seats));
}

fn part1(seats: &HashMap<(i32, i32), char>) -> usize {
    let mut seats = seats.clone();
    let mut changed = true;
    while changed {
        changed = false;
        let orig = seats.clone();
        for (&(i, j), s) in seats.iter_mut() {
            let mut occupied = 0;
            for u in -1..=1 {
                for v in -1..=1 {
                    if (u, v) != (0, 0) {
                        let seat = *orig.get(&(i + u, j + v)).unwrap_or(&' ');
                        if seat == '#' {
                            occupied += 1;
                        }
                    }
                }
            }
            if *s == 'L' && occupied == 0 {
                *s = '#';
                changed = true;
            } else if *s == '#' && occupied >= 4 {
                *s = 'L';
                changed = true;
            }
        }
    }

    seats.values().filter(|&&s| s == '#').count()
}

fn part2(seats: &HashMap<(i32, i32), char>) -> usize {
    let mut seats = seats.clone();
    let mut changed = true;
    while changed {
        changed = false;
        let orig = seats.clone();
        for (&(i, j), s) in seats.iter_mut() {
            let mut occupied = 0;
            for u in -1..=1 {
                for v in -1..=1 {
                    if (u, v) != (0, 0) {
                        let mut a = i;
                        let mut b = j;
                        loop {
                            a += u;
                            b += v;
                            let seat = *orig.get(&(a, b)).unwrap_or(&' ');
                            if seat != '.' {
                                if seat == '#' {
                                    occupied += 1;
                                }
                                break;
                            }
                        }
                    }
                }
            }
            if *s == 'L' && occupied == 0 {
                *s = '#';
                changed = true;
            } else if *s == '#' && occupied >= 5 {
                *s = 'L';
                changed = true;
            }
        }
    }

    seats.values().filter(|&&s| s == '#').count()
}

fn parse(lines: &[&str]) -> HashMap<(i32, i32), char> {
    lines
        .iter()
        .enumerate()
        .flat_map(|(i, r)| {
            r.chars()
                .enumerate()
                .map(move |(j, c)| ((i as i32, j as i32), c))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let lines = [
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ];
        let seats = parse(&lines);

        assert_eq!(part1(&seats), 37);
        assert_eq!(part2(&seats), 26);
    }
}
