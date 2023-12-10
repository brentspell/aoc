use std::collections::HashSet;

pub fn solve() {
    let data = std::fs::read_to_string("data/day10.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    let grid = parse(data);
    let start = find_start(&grid);

    // for part 1, the shortest path is just half the number of cells in the pipe
    trace_pipe(&grid, start).len() as u64 / 2
}

fn part2(data: &str) -> u64 {
    let grid = parse(data);
    let start = find_start(&grid);
    let pipe = trace_pipe(&grid, start);

    // for part 2, we need to figure out which points in the grid are "inside" the pipe
    // . the key here is that we can determine whether we are inside the pipe by the number
    //   of times we cross the pipe when traversing outward in any direction
    // . so we walk to the right of every point and count the number of pipe crossings - if
    //   it is odd, we are inside the pipe
    let mut inside = 0;
    for i in 0..grid.len() as i64 {
        for j in 0..grid[0].len() as i64 {
            // ignore any points on the pipe itself
            if pipe.contains(&(i, j)) {
                continue;
            }

            // begin the rightward walk, maintaining the value of the previous bend
            let mut parity = 0;
            let mut prev = '.';
            for k in j..grid[0].len() as i64 {
                // if this isn't a pipe crossing, ignore and keep walking
                if !pipe.contains(&(i, k)) {
                    continue;
                }

                // if we have hit the start point ("S"), we need to figure out
                // its actual symbol, based on the diretions of the transitions from it
                let tile = if grid[i as usize][k as usize] != 'S' {
                    grid[i as usize][k as usize]
                } else {
                    match start_trans(&grid, start).as_slice() {
                        [(-1, 0), (1, 0)] => '|',
                        [(0, -1), (0, 1)] => '-',
                        [(-1, 0), (0, -1)] => 'J',
                        [(-1, 0), (0, 1)] => 'L',
                        [(1, 0), (0, -1)] => '7',
                        [(1, 0), (0, 1)] => 'F',
                        _ => panic!("invalid start position"),
                    }
                };

                // now we can detect the crossing
                // . since we are going left-to-right, "|" is always a crossing
                // . similarly, "F" and "L" tiles are vertical walls indicating a crossing
                // . the other bends are more tricky - we need to detect whether the pair
                //   of previous/current bends form a wall ("F..J" and "L..7") instead of
                //   a boundary that we could simply walk around ("F..7" and "L..J")
                // . if we find a wall, increment the parity
                // . if we find F/L bend, save it off so we can detect a wall later
                match (prev, tile) {
                    (_, '|') | ('F', 'J') | ('L', '7') => parity += 1,
                    (_, 'F' | 'L') => prev = tile,
                    _ => {}
                }
            }

            // if we had an odd number of crossings, count the tile as inside the pipe
            if parity % 2 == 1 {
                inside += 1;
            }
        }
    }

    inside
}

fn parse(data: &str) -> Vec<Vec<char>> {
    data.lines().map(|l| l.chars().collect()).collect()
}

fn find_start(grid: &[Vec<char>]) -> (i64, i64) {
    (0..grid.len())
        .flat_map(|i| (0..grid[i].len()).map(move |j| (i as i64, j as i64)))
        .find(|&(i, j)| grid[i as usize][j as usize] == 'S')
        .unwrap()
}

fn start_trans(grid: &[Vec<char>], start: (i64, i64)) -> Vec<(i64, i64)> {
    // determine all of the transitions from the starting point
    // . find all indexes above, below, left, and right from the start
    // . first, verify we have valid indexes into the grid
    // . then we need to make sure that the adjacent tiles connect to the starting
    //   point, since the grid may be full of garbage tiles not connected to anything
    // . we find all of the adjacent tiles so we can later use them to inpute the
    //   type of the starting tile (since it is just labeled as "S")
    let (i, j) = start;
    [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .copied()
        .filter(|&(u, v)| {
            i + u >= 0
                && i + u < grid.len() as i64
                && j + v >= 0
                && j + v < grid[0].len() as i64
                && matches!(
                    (u, v, grid[(i + u) as usize][(j + v) as usize]),
                    (-1 | 1, 0, '|')
                        | (0, -1 | 1, '-')
                        | (-1, 0, '7' | 'F')
                        | (1, 0, 'L' | 'J')
                        | (0, -1, 'L' | 'F')
                        | (0, 1, '7' | 'J')
                )
        })
        .collect()
}

fn trace_pipe(grid: &[Vec<char>], start: (i64, i64)) -> HashSet<(i64, i64)> {
    let mut pipe: HashSet<(i64, i64)> = HashSet::new();

    // simply walk round the pipe and collect all of the indices
    // until we get back to the starting position
    let mut pos = start;
    while pipe.insert(pos) {
        let (i, j) = pos;
        pos = match grid[i as usize][j as usize] {
            'S' => start_trans(grid, start)
                .iter()
                .map(|(u, v)| (i + u, j + v))
                .collect(),
            '|' => vec![(i - 1, j), (i + 1, j)],
            '-' => vec![(i, j - 1), (i, j + 1)],
            'J' => vec![(i - 1, j), (i, j - 1)],
            'L' => vec![(i - 1, j), (i, j + 1)],
            '7' => vec![(i + 1, j), (i, j - 1)],
            'F' => vec![(i + 1, j), (i, j + 1)],
            _ => panic!("invalid pipe position"),
        }
        .iter()
        .copied()
        .find(|&p| !pipe.contains(&p))
        .unwrap_or(pos);
    }

    pipe
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = "\
            .....\n\
            .S-7.\n\
            .|.|.\n\
            .L-J.\n\
            .....\n\
        ";
        assert_eq!(part1(data), 4);

        let data = "\
            ..F7.\n\
            .FJ|.\n\
            SJ.L7\n\
            |F--J\n\
            LJ...\n\
        ";
        assert_eq!(part1(data), 8);
    }

    #[test]
    fn test_part2() {
        let data: &str = "\
            .....\n\
            .S-7.\n\
            .|.|.\n\
            .L-J.\n\
            .....\n\
        ";
        assert_eq!(part2(data), 1);

        let data: &str = "\
            ...........\n\
            .S-------7.\n\
            .|F-----7|.\n\
            .||.....||.\n\
            .||.....||.\n\
            .|L-7.F-J|.\n\
            .|..|.|..|.\n\
            .L--J.L--J.\n\
            ...........\n\
        ";
        assert_eq!(part2(data), 4);

        let data: &str = "\
            ..........\n\
            .S------7.\n\
            .|F----7|.\n\
            .||....||.\n\
            .||....||.\n\
            .|L-7F-J|.\n\
            .|..||..|.\n\
            .L--JL--J.\n\
            ..........\n\
        ";
        assert_eq!(part2(data), 4);

        let data: &str = "\
            .F----7F7F7F7F-7....\n\
            .|F--7||||||||FJ....\n\
            .||.FJ||||||||L7....\n\
            FJL7L7LJLJ||LJ.L-7..\n\
            L--J.L7...LJS7F-7L7.\n\
            ....F-J..F7FJ|L7L7L7\n\
            ....L7.F7||L7|.L7L7|\n\
            .....|FJLJ|FJ|F7|.LJ\n\
            ....FJL-7.||.||||...\n\
            ....L---J.LJ.LJLJ...\n\
        ";
        assert_eq!(part2(data), 8);
    }
}
