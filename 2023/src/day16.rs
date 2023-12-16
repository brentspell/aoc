use std::collections::HashSet;

pub fn solve() {
    let data = std::fs::read_to_string("data/day16.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    let grid: Vec<Vec<_>> = data.lines().map(|l| l.chars().collect()).collect();
    count_energized(&grid, ('>', 0, 0))
}

fn part2(data: &str) -> u64 {
    let grid: Vec<Vec<_>> = data.lines().map(|l| l.chars().collect()).collect();

    // brute force search the border of the grid and direction
    // that produces the largest number of energized tiles
    let mut max = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            for d in ['^', '>', 'v', '<'] {
                match (i, j, d) {
                    (0, _, 'v') => {}
                    (p, _, '^') if p == grid.len() - 1 => {}
                    (_, 0, '>') => {}
                    (_, q, '<') if q == grid[0].len() - 1 => {}
                    _ => {
                        continue;
                    }
                }

                max = std::cmp::max(max, count_energized(&grid, (d, i as i64, j as i64)));
            }
        }
    }

    max
}

fn count_energized(grid: &[Vec<char>], start: (char, i64, i64)) -> u64 {
    // maintain the set of unique energized tiles,
    // the set of seen tiles/directions,
    // and a stack to use for search
    let mut result = HashSet::new();
    let mut done = HashSet::new();
    let mut stack = vec![start];
    while let Some(p @ (d, i, j)) = stack.pop() {
        done.insert(p);
        result.insert((i, j));

        // determine the set of next directions from the current tile and direction
        // using the rules of reflection and splitting
        let next = match (d, grid[i as usize][j as usize]) {
            // simply pass through the current point
            ('^', '.' | '|') => std::slice::from_ref(&d),
            ('>', '.' | '-') => std::slice::from_ref(&d),
            ('v', '.' | '|') => std::slice::from_ref(&d),
            ('<', '.' | '-') => std::slice::from_ref(&d),
            // split the beam into two
            ('^' | 'v', '-') => &['<', '>'],
            ('<' | '>', '|') => &['^', 'v'],
            // reflect the beam
            ('^', '/') => &['>'],
            ('^', '\\') => &['<'],
            ('>', '/') => &['^'],
            ('>', '\\') => &['v'],
            ('v', '/') => &['<'],
            ('v', '\\') => &['>'],
            ('<', '/') => &['v'],
            ('<', '\\') => &['^'],
            _ => panic!("invalid direction/tile: ({i}, {j}): {d}"),
        };

        // expand the search to the new direction(s) and tile(s)
        for &d in next {
            let (i, j) = match d {
                '^' => (i - 1, j),
                '>' => (i, j + 1),
                'v' => (i + 1, j),
                '<' => (i, j - 1),
                _ => panic!("invalid direction"),
            };
            if (0..grid.len() as i64).contains(&i)
                && (0..grid[0].len() as i64).contains(&j)
                && !done.contains(&(d, i, j))
            {
                stack.push((d, i, j));
            }
        }
    }

    result.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        .|...\\....\n\
        |.-.\\.....\n\
        .....|-...\n\
        ........|.\n\
        ..........\n\
        .........\\\n\
        ..../.\\\\..\n\
        .-.-/..|..\n\
        .|....-|.\\\n\
        ..//.|....\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 51);
    }
}
