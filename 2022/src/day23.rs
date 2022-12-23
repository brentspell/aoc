use std::collections::{HashMap, HashSet, VecDeque};

pub fn solve() {
    let elves = parse(&std::fs::read_to_string("data/day23.txt").expect("data-file"));

    // part 1
    println!("part 1: {}", part1(elves.clone()));

    // part 2
    println!("part 2: {}", part2(elves));
}

fn parse(data: &str) -> HashSet<(i32, i32)> {
    let chars: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    chars
        .iter()
        .enumerate()
        .flat_map(move |(i, r)| r.iter().copied().enumerate().map(move |(j, x)| (i, j, x)))
        .filter(|&(_i, _j, x)| x == '#')
        .map(|(i, j, _x)| (i as i32, j as i32))
        .collect()
}

fn part1(mut elves: HashSet<(i32, i32)>) -> u32 {
    // these are the allowed moves (first tuple) and directions to scan for each move
    // the deque is rotated by 1 during each round
    let mut moves = VecDeque::from([
        vec![(-1, 0), (-1, -1), (-1, 1)],
        vec![(1, 0), (1, -1), (1, 1)],
        vec![(0, -1), (-1, -1), (1, -1)],
        vec![(0, 1), (-1, 1), (1, 1)],
    ]);

    for _ in 0..10 {
        // compute proposed moves for all elves (target position->list of sources)
        let mut proposals = HashMap::new();
        for (i, j) in elves.iter().copied() {
            // an elf can only move if it has a neighboring elf
            let neighbors = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            if neighbors
                .iter()
                .any(|(p, q)| elves.contains(&(i + p, j + q)))
            {
                // propose the first move that is possible based on the neighborhood scan
                for m in moves.iter() {
                    if !m.iter().any(|(p, q)| elves.contains(&(i + p, j + q))) {
                        // if the move is valid, add it to the proposals
                        let (p, q) = m[0];
                        proposals
                            .entry((i + p, j + q))
                            .or_insert_with(Vec::new)
                            .push((i, j));
                        break;
                    }
                }
            }
        }

        // execute any proposed moves with only 1 source elf
        for (new, olds) in proposals {
            if olds.len() == 1 {
                elves.remove(&olds[0]);
                elves.insert(new);
            }
        }

        // rotate the move deque
        let m = moves.pop_front().expect("moves");
        moves.push_back(m);
    }

    // compute the size of the bounding box and subtract the elf count for the result
    let mini = elves.iter().map(|(i, _)| i).min().expect("elves");
    let maxi = elves.iter().map(|(i, _)| i).max().expect("elves");
    let minj = elves.iter().map(|(_, j)| j).min().expect("elves");
    let maxj = elves.iter().map(|(_, j)| j).max().expect("elves");
    (maxi - mini + 1) as u32 * (maxj - minj + 1) as u32 - elves.len() as u32
}

fn part2(mut elves: HashSet<(i32, i32)>) -> u32 {
    // these are the allowed moves (first tuple) and directions to scan for each move
    // the deque is rotated by 1 during each round
    let mut moves = VecDeque::from([
        vec![(-1, 0), (-1, -1), (-1, 1)],
        vec![(1, 0), (1, -1), (1, 1)],
        vec![(0, -1), (-1, -1), (1, -1)],
        vec![(0, 1), (-1, 1), (1, 1)],
    ]);

    let mut round = 0;
    let mut done = false;
    while !done {
        // compute proposed moves for all elves (target position->list of sources)
        let mut proposals = HashMap::new();
        for (i, j) in elves.iter().copied() {
            let neighbors = [
                (-1, -1),
                (-1, 0),
                (-1, 1),
                (0, -1),
                (0, 1),
                (1, -1),
                (1, 0),
                (1, 1),
            ];
            if neighbors
                .iter()
                .any(|(p, q)| elves.contains(&(i + p, j + q)))
            {
                // propose the first move that is possible based on the neighborhood scan
                for m in moves.iter() {
                    if !m.iter().any(|(p, q)| elves.contains(&(i + p, j + q))) {
                        let (p, q) = m[0];
                        proposals
                            .entry((i + p, j + q))
                            .or_insert_with(Vec::new)
                            .push((i, j));
                        break;
                    }
                }
            }
        }

        // execute any proposed moves with only 1 source elf
        // bail out if no elves moved
        done = true;
        for (new, olds) in proposals {
            if olds.len() == 1 {
                elves.remove(&olds[0]);
                elves.insert(new);
                done = false;
            }
        }

        // rotate the move deque
        let m = moves.pop_front().expect("moves");
        moves.push_back(m);

        round += 1;
    }

    round
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse(DATA)), 110);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse(DATA)), 20);
    }
}
