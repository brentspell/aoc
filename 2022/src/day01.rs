use std::collections::BinaryHeap;

pub fn solve() {
    let elves = parse(&std::fs::read_to_string("data/day02.txt").unwrap());

    // part 1
    println!("part 1: {}", part1(&elves));

    // part 2
    println!("part 2: {}", part2(&elves));
}

fn parse(string: &str) -> Vec<Vec<u32>> {
    let mut elves = vec![vec![]];
    for line in string.lines() {
        if line.is_empty() {
            elves.push(vec![]);
        } else {
            let last = elves.len() - 1;
            elves[last].push(line.parse().unwrap());
        }
    }

    elves
}

fn part1(elves: &[Vec<u32>]) -> u32 {
    elves
        .iter()
        .map(|v| v.iter().sum())
        .max()
        .unwrap_or_default()
}

fn part2(elves: &[Vec<u32>]) -> u32 {
    const K: usize = 3;
    let mut top = BinaryHeap::new();
    top.reserve(K);
    for v in elves {
        let s = -(v.iter().sum::<u32>() as i32);
        if top.len() < K {
            top.push(s);
        } else if s < *top.peek().unwrap() {
            top.pop();
            top.push(s)
        }
    }
    -top.iter().sum::<i32>() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const ELVES: &str = "\
        1000\n\
        2000\n\
        3000\n\
        \n\
        4000\n\
        \n\
        5000\n\
        6000\n\
        \n\
        7000\n\
        8000\n\
        9000\n\
        \n\
        10000\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(ELVES)), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(ELVES)), 45000);
    }
}
