use std::collections::HashSet;

pub fn solve() {
    let data = std::fs::read_to_string("data/day21.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data, 64));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str, n: i64) -> u64 {
    let grid: Vec<Vec<_>> = data.lines().map(|l| l.chars().collect()).collect();

    for i in 0..grid.len() as i64 {
        for j in 0..grid.len() as i64 {
            if grid[i as usize][j as usize] == 'S' {
                return simulate(&grid, &[(i, j)], n);
            }
        }
    }

    panic!("start not found");
}

fn part2(data: &str) -> u64 {
    let grid: Vec<Vec<_>> = data.lines().map(|l| l.chars().collect()).collect();

    // the trick to part 2 is to realize that the neighboring grids are
    // all entered at the corners and at the centers of each edge
    // this causes the grids to expand in a diamond pattern that looks like
    //
    //   oeo
    //  oifio
    // oifffio
    // efffffe
    // oifffio
    //  oifio
    //   oeo
    //
    // f: these are grids that fully saturate and oscillate between two states
    //    f1 and f2 as neighboring tiles are toggled, with neighboring f grids
    //    also in alternating states
    // e: these are the edges that are crossed completely (w steps) and entered at the
    //    centers of a single edge
    // i: this is an inner border tile that is entered from two neighboring f edges,
    //    and progressed w steps from both edges
    // o: this is an outer border tile that is entered from a corner, and progressed
    //    w / 2 steps

    let n = 26501365;
    let w = grid.len() as i64;

    // first, calculate the diameter of the diamond and its hypotenuse
    let d = (2 * (n - w / 2) / w + 1) as u64;
    let h = d / 2 + 1;

    // next, simulate the alternating numbers of tiles in the inner (d - 2) full grids
    let f1 = simulate(&grid, &[(w / 2, w / 2)], w);
    let f2 = simulate(&grid, &[(w / 2, w / 2)], w + 1);
    let f = ((d - 2) / 2).pow(2) * f1 + ((d - 2) / 2 + 1).pow(2) * f2;

    // next the inner border tiles, there are h - 2 of each of these
    let i = (h - 2)
        * (simulate(&grid, &[(w / 2, 0), (0, w / 2)], w - 1)
            + simulate(&grid, &[(0, w / 2), (w / 2, w - 1)], w - 1)
            + simulate(&grid, &[(w / 2, w - 1), (w - 1, w / 2)], w - 1)
            + simulate(&grid, &[(w - 1, w / 2), (w / 2, 0)], w - 1));

    // next the outer border tiles, there are h - 1 of each of these
    let o = (h - 1)
        * (simulate(&grid, &[(0, 0)], w / 2 - 1)
            + simulate(&grid, &[(0, w - 1)], w / 2 - 1)
            + simulate(&grid, &[(w - 1, w - 1)], w / 2 - 1)
            + simulate(&grid, &[(w - 1, 0)], w / 2 - 1));

    // finally the 4 edge tiles
    let e = simulate(&grid, &[(0, w / 2)], w - 1)
        + simulate(&grid, &[(w / 2, w - 1)], w - 1)
        + simulate(&grid, &[(w - 1, w / 2)], w - 1)
        + simulate(&grid, &[(w / 2, 0)], w - 1);

    f + i + o + e
}

fn simulate(grid: &[Vec<char>], start: &[(i64, i64)], n: i64) -> u64 {
    let mut points: HashSet<_> = start.iter().copied().collect();

    for _ in 0..n {
        let mut next = HashSet::new();
        for &(i, j) in points.iter() {
            for (p, q) in [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
                if p >= 0
                    && p < grid.len() as i64
                    && q >= 0
                    && q < grid[0].len() as i64
                    && grid[p as usize][q as usize] != '#'
                {
                    next.insert((p, q));
                }
            }
        }
        points = next;
    }

    points.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        ...........\n\
        .....###.#.\n\
        .###.##..#.\n\
        ..#.#...#..\n\
        ....#.#....\n\
        .##..S####.\n\
        .##..#...#.\n\
        .......##..\n\
        .##.#.####.\n\
        .##..##.##.\n\
        ...........\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA, 6), 16);
    }
}
