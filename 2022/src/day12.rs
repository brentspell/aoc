use std::collections::{HashSet, VecDeque};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(usize, usize);

pub fn solve() {
    let (start, end, grid) = parse(&std::fs::read_to_string("data/day12.txt").expect("data-file"));

    // part 1
    println!("part 1: {}", part1(start, end, &grid));

    // part 2
    println!("part 2: {}", part2(end, &grid));
}

fn parse(data: &str) -> (Pos, Pos, Vec<Vec<usize>>) {
    let mut start = Pos(0, 0);
    let mut end = Pos(0, 0);
    let grid = data
        .lines()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .map(|(j, ch)| match ch {
                    'S' => {
                        start = Pos(i, j);
                        0
                    }
                    'E' => {
                        end = Pos(i, j);
                        'z' as usize - 'a' as usize
                    }
                    _ => ch as usize - 'a' as usize,
                })
                .collect()
        })
        .collect();

    (start, end, grid)
}

fn part1(start: Pos, end: Pos, grid: &[Vec<usize>]) -> usize {
    // simple bfs from start to end
    let mut queue = VecDeque::from([(0, start)]);
    let mut done = HashSet::from([start]);
    while !queue.is_empty() {
        // process the next position in breadth-first order
        // if we have reached the end, we have the shortest path
        let (steps, pos @ Pos(i, j)) = queue.pop_front().expect("queue-empty");
        if pos == end {
            return steps;
        }

        // compute the next valid positions
        let mut next: Vec<Pos> = Vec::new();
        if i > 0 {
            next.push(Pos(i - 1, j));
        }
        if i < grid.len() - 1 {
            next.push(Pos(i + 1, j));
        }
        if j > 0 {
            next.push(Pos(i, j - 1));
        }
        if j < grid[0].len() - 1 {
            next.push(Pos(i, j + 1));
        }

        // advance to the next position if it's not too steep
        // and we haven't already been there
        for pos @ Pos(p, q) in next {
            if grid[p][q] <= grid[i][j] + 1 && done.insert(pos) {
                queue.push_back((steps + 1, pos));
            }
        }
    }

    panic!("end-not-found");
}

fn part2(end: Pos, grid: &[Vec<usize>]) -> usize {
    // simple bfs from the end to the first "a"
    let mut queue = VecDeque::from([(0, end)]);
    let mut done = HashSet::from([end]);
    while !queue.is_empty() {
        // process the next position in breadth-first order
        // if we have reached an "a" (zero height), we have the shortest path
        let (steps, Pos(i, j)) = queue.pop_front().expect("queue-empty");
        if grid[i][j] == 0 {
            return steps;
        }

        // compute the next valid positions
        let mut next: Vec<Pos> = Vec::new();
        if i > 0 {
            next.push(Pos(i - 1, j));
        }
        if i < grid.len() - 1 {
            next.push(Pos(i + 1, j));
        }
        if j > 0 {
            next.push(Pos(i, j - 1));
        }
        if j < grid[0].len() - 1 {
            next.push(Pos(i, j + 1));
        }

        // advance to the next position if it's not too steep (reversed)
        // and we haven't already been there
        for pos @ Pos(p, q) in next {
            if grid[i][j] <= grid[p][q] + 1 && done.insert(pos) {
                queue.push_back((steps + 1, pos));
            }
        }
    }

    panic!("end-not-found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        Sabqponm\n\
        abcryxxl\n\
        accszExk\n\
        acctuvwj\n\
        abdefghi\n\
    ";

    #[test]
    fn test_part1() {
        let (start, end, grid) = parse(DATA);
        assert_eq!(part1(start, end, &grid), 31);
    }

    #[test]
    fn test_part2() {
        let (_start, end, grid) = parse(DATA);
        assert_eq!(part2(end, &grid), 29);
    }
}
