use std::cmp::{max, min};
use std::collections::HashSet;

pub fn solve() {
    let data = std::fs::read_to_string("data/day11.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data, 1000000));
}

fn part1(data: &str) -> u64 {
    distance(data, 2)
}

fn part2(data: &str, expansion: u64) -> u64 {
    distance(data, expansion)
}

fn distance(data: &str, expansion: u64) -> u64 {
    let grid: Vec<Vec<_>> = data.lines().map(|l| l.chars().collect()).collect();

    // collect all the galaxies
    let gals: Vec<_> = (0..grid.len())
        .flat_map(|i| (0..grid[i].len()).map(move |j| (i, j)))
        .filter(|&(i, j)| grid[i][j] == '#')
        .collect();

    // find all rows/columns that expanded (have no galaxies)
    let exp_rows: HashSet<_> = (0..grid.len())
        .filter(|&i| (0..grid[0].len()).all(|j| grid[i][j] == '.'))
        .collect();
    let exp_cols: HashSet<_> = (0..grid[0].len())
        .filter(|&j| (0..grid.len()).all(|i| grid[i][j] == '.'))
        .collect();

    // compute pairwise manhattan distances, taking expansion into effect
    let mut dist: Vec<Vec<_>> = (0..gals.len())
        .map(|a| {
            (0..gals.len())
                .map(|b| {
                    let ((i0, j0), (i1, j1)) = (gals[a], gals[b]);
                    let rows: u64 = (min(i0, i1) + 1..max(i0, i1) + 1)
                        .map(|i| if exp_rows.contains(&i) { expansion } else { 1 })
                        .sum();
                    let cols: u64 = (min(j0, j1) + 1..max(j0, j1) + 1)
                        .map(|i| if exp_cols.contains(&i) { expansion } else { 1 })
                        .sum();
                    rows + cols
                })
                .collect()
        })
        .collect();

    // use floyd-warshall to find shortest paths between all pairs
    for k in 0..dist.len() {
        for i in 0..dist.len() {
            for j in 0..dist.len() {
                let d = dist[i][k] + dist[k][j];
                if d < dist[i][j] {
                    dist[i][j] = d;
                }
            }
        }
    }

    // the total distance is the sum of the upper or lower triangular
    dist.iter().map(|d| d.iter().sum::<u64>()).sum::<u64>() / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        ...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA, 10), 1030);
        assert_eq!(part2(DATA, 100), 8410);
    }
}
