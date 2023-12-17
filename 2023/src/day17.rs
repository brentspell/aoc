use std::collections::{BinaryHeap, HashSet};

pub fn solve() {
    let data = std::fs::read_to_string("data/day17.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    let grid: Vec<Vec<_>> = data
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i64).collect())
        .collect();

    // use dijkstra to search the grid beginning at the top left
    // rust provides a max-heap, so use the heat gained (-lost) as the priority
    // and store the tile position (i, j), the current direction (d),
    // and the number (n) of steps in that direction as the search state
    let mut heap = BinaryHeap::new();
    let mut done = HashSet::new();
    heap.push((0, 0, 0, '>', 0));
    heap.push((0, 0, 0, 'v', 0));
    while let Some((h, i, j, d, n)) = heap.pop() {
        // if we have reached the bottom left, we have found our maximum remaining heat
        if i == grid.len() as i64 - 1 && j == grid[0].len() as i64 - 1 {
            return -h as u64;
        }

        // enumerate the valid directions (no reversing allowed)
        let next: &[char] = match d {
            '^' => &['<', '^', '>'],
            '>' => &['^', '>', 'v'],
            'v' => &['>', 'v', '<'],
            '<' => &['v', '<', '^'],
            _ => panic!("invalid direction"),
        };
        for &e in next {
            // compute the next tile position from the selected direction
            let (i, j) = match e {
                '^' => (i - 1, j),
                '>' => (i, j + 1),
                'v' => (i + 1, j),
                '<' => (i, j - 1),
                _ => panic!("invalid direction"),
            };

            // make sure we are in bounds, and avoid going more than 3 steps in the same direction
            if (0..grid.len() as i64).contains(&i)
                && (0..grid[0].len() as i64).contains(&j)
                && (n < 3 || e != d)
            {
                // update the number of consecutive steps in the same direction, and
                // add the next state to the queue, ignoring any seen states to avoid cycles
                let n = if e == d { n + 1 } else { 1 };
                if done.insert((i, j, e, n)) {
                    heap.push((h - grid[i as usize][j as usize], i, j, e, n));
                }
            }
        }
    }

    panic!("goal not found");
}

fn part2(data: &str) -> u64 {
    let grid: Vec<Vec<_>> = data
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i64).collect())
        .collect();

    // use dijkstra just as in part 1
    let mut heap = BinaryHeap::new();
    let mut done = HashSet::new();
    heap.push((0, 0, 0, '>', 0));
    heap.push((0, 0, 0, 'v', 0));
    while let Some((h, i, j, d, n)) = heap.pop() {
        // since we must move a minimum of 4 steps in one direction, the goal states change
        // the first goal state is 4 steps from the true goal on the last row, moving right
        if i == grid.len() as i64 - 1 && j == grid[0].len() as i64 - 4 && d == '>' && n <= 6 {
            // add the remaining steps on the row to get the final heat loss
            return (-h
                + grid[grid.len() - 1][grid[0].len() - 3..]
                    .iter()
                    .sum::<i64>()) as u64;
        }
        // the other goal state is 4 steps from the true goal on the last column, moving down
        if i == grid.len() as i64 - 4 && j == grid[0].len() as i64 - 1 && d == 'v' && n <= 6 {
            // add the remaining steps on the column to get the final heat loss
            return (-h
                + grid[grid.len() - 3..]
                    .iter()
                    .map(|g| g[g.len() - 1])
                    .sum::<i64>()) as u64;
        }

        // enumerate the valid directions (no reversing allowed)
        let next: &[char] = match d {
            '^' => &['<', '^', '>'],
            '>' => &['^', '>', 'v'],
            'v' => &['>', 'v', '<'],
            '<' => &['v', '<', '^'],
            _ => panic!("invalid direction"),
        };
        for &e in next {
            // compute the next tile position from the selected direction
            let (i, j) = match e {
                '^' => (i - 1, j),
                '>' => (i, j + 1),
                'v' => (i + 1, j),
                '<' => (i, j - 1),
                _ => panic!("invalid direction"),
            };

            // make sure we are in bounds, and that we are taking a minimum
            // of 4 and a maximum of 10 steps in any one direction
            if (0..grid.len() as i64).contains(&i)
                && (0..grid[0].len() as i64).contains(&j)
                && (n >= 4 || e == d)
                && (n < 10 || e != d)
            {
                // update the number of consecutive steps in the same direction, and
                // add the next state to the queue, ignoring any seen states to avoid cycles
                let n = if e == d { n + 1 } else { 1 };
                if done.insert((i, j, e, n)) {
                    heap.push((h - grid[i as usize][j as usize], i, j, e, n));
                }
            }
        }
    }

    panic!("goal not found");
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA1: &str = "\
        2413432311323\n\
        3215453535623\n\
        3255245654254\n\
        3446585845452\n\
        4546657867536\n\
        1438598798454\n\
        4457876987766\n\
        3637877979653\n\
        4654967986887\n\
        4564679986453\n\
        1224686865563\n\
        2546548887735\n\
        4322674655533\n\
    ";

    const DATA2: &str = "\
        111111111111\n\
        999999999991\n\
        999999999991\n\
        999999999991\n\
        999999999991\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA1), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA1), 94);
        assert_eq!(part2(DATA2), 71);
    }
}
