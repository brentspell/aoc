use std::collections::{HashMap, HashSet};

pub fn solve() {
    let data = std::fs::read_to_string("data/day23.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    let grid: Vec<Vec<_>> = data.lines().map(|l| l.chars().collect()).collect();
    let start = (0, 1);
    let goal = (grid.len() as i64 - 1, grid[0].len() as i64 - 2);

    // in part 1, we can assume that the graph is a DAG (cycles are stopped by the arrows),
    // so the longest path can be calculated efficiently using DFS
    let mut max = 0;
    let mut stack = vec![(start, HashSet::new())];
    while let Some(((i, j), mut p)) = stack.pop() {
        p.insert((i, j));
        if (i, j) == goal {
            max = std::cmp::max(max, p.len() - 1);
        } else {
            // expand the path to all valid neighbors, ignoring
            // any arrows pointing opposite to our current direction
            for (a, b) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let (k, l) = (i + a, j + b);
                if (0..grid.len() as i64).contains(&k)
                    && (0..grid[0].len() as i64).contains(&l)
                    && grid[k as usize][l as usize] != '#'
                    && matches!(
                        (grid[i as usize][j as usize], a, b),
                        ('.', _, _) | ('^', -1, _) | ('>', _, 1) | ('v', 1, _) | ('<', _, -1)
                    )
                    && !p.contains(&(k, l))
                {
                    stack.push(((k, l), p.clone()));
                }
            }
        }
    }

    max as u64
}

fn part2(data: &str) -> u64 {
    let grid: Vec<Vec<_>> = data.lines().map(|l| l.chars().collect()).collect();
    let start = (0, 1);
    let goal = (grid.len() as i64 - 1, grid[0].len() as i64 - 2);

    // for part 2, the graph has cycles, so longest path is NP-Hard
    // first, we find all the tiles that cause the cycles, which are
    // the tiles surrounded by arrows, along with the start/goal tiles
    let mut centers = HashSet::from([start, goal]);
    for i in 0..grid.len() as i64 {
        for j in 0..grid[0].len() as i64 {
            // find the neighboring arrow tiles
            let arrows = [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)]
                .iter()
                .copied()
                .filter(|&(k, l)| {
                    (0..grid.len() as i64).contains(&k)
                        && (0..grid[0].len() as i64).contains(&l)
                        && matches!(grid[k as usize][l as usize], '^' | '>' | 'v' | '<')
                })
                .count();

            // if there are at least 2 of them, we have found a center
            if grid[i as usize][j as usize] == '.' && arrows > 2 {
                centers.insert((i, j));
            }
        }
    }

    // since we know we can safely DFS between these centers, do so
    // in order to collect all paths between the centers, while carefully
    // not progressing any path beyond a center (which would enter a cycle)
    let mut paths = HashMap::new();
    for &center in centers.iter() {
        // use DFS to find all paths from the current center to any other reachable center
        let mut stack = vec![(center, HashSet::new())];
        while let Some(((i, j), mut set)) = stack.pop() {
            set.insert((i, j));

            // expand the path to all valid neighbors
            for (k, l) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                if (0..grid.len() as i64).contains(&k)
                    && (0..grid[0].len() as i64).contains(&l)
                    && grid[k as usize][l as usize] != '#'
                    && !set.contains(&(k, l))
                {
                    // if we have found a center, record the path and stop,
                    // otherwise, continue expanding the path
                    if centers.contains(&(k, l)) {
                        paths
                            .entry(center)
                            .or_insert_with(Vec::new)
                            .push(((k, l), set.len()));
                    } else {
                        stack.push(((k, l), set.clone()));
                    }
                }
            }
        }
    }

    // now that we have all paths between all centers, we still have an NP-Hard
    // longest path search/TSP, but over the much smaller set of centers
    let mut max = 0;
    let mut stack = vec![(start, HashSet::new(), 0)];
    while let Some(((i, j), mut done, n)) = stack.pop() {
        done.insert((i, j));
        if (i, j) == goal {
            max = std::cmp::max(max, n);
        }
        for &((k, l), steps) in paths[&(i, j)].iter() {
            if !done.contains(&(k, l)) {
                stack.push(((k, l), done.clone(), n + steps));
            }
        }
    }

    max as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        #.#####################\n\
        #.......#########...###\n\
        #######.#########.#.###\n\
        ###.....#.>.>.###.#.###\n\
        ###v#####.#v#.###.#.###\n\
        ###.>...#.#.#.....#...#\n\
        ###v###.#.#.#########.#\n\
        ###...#.#.#.......#...#\n\
        #####.#.#.#######.#.###\n\
        #.....#.#.#.......#...#\n\
        #.#####.#.#.#########v#\n\
        #.#...#...#...###...>.#\n\
        #.#.#v#######v###.###v#\n\
        #...#.>.#...>.>.#.###.#\n\
        #####v#.#.###v#.#.###.#\n\
        #.....#...#...#.#.#...#\n\
        #.#########.###.#.#.###\n\
        #...###...#...#...#.###\n\
        ###.###.#.###v#####v###\n\
        #...#...#.#.>.>.#.>.###\n\
        #.###.###.#.###.#.#v###\n\
        #.....###...###...#...#\n\
        #####################.#\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 94);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 154);
    }
}
