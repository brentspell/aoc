pub fn solve() {
    let data = std::fs::read_to_string("data/day04.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    // part 1
    println!("part 1: {}", part1(&lines));

    // part 2
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[&str]) -> u64 {
    let grid: Vec<Vec<_>> = lines.iter().map(|l| l.chars().collect()).collect();
    let m = grid.len() as i64;
    let n = grid[0].len() as i64;

    (0..m)
        .flat_map(|i| (0..n).map(move |j| (i, j)))
        .filter(|&(i, j)| {
            // count the number of rolls in the subgrid centered at this roll
            // if less than 5 cells in the subgrid are rolls, this roll is accessible
            grid[i as usize][j as usize] == '@'
                && (i - 1..=i + 1)
                    .flat_map(|u| (j - 1..=j + 1).map(move |v| (u, v)))
                    .filter(|&(u, v)| {
                        (0..m).contains(&u)
                            && (0..n).contains(&v)
                            && grid[u as usize][v as usize] == '@'
                    })
                    .count()
                    < 5
        })
        .count() as u64
}

fn part2(lines: &[&str]) -> u64 {
    let mut grid: Vec<Vec<_>> = lines.iter().map(|l| l.chars().collect()).collect();
    let m = grid.len() as i64;
    let n = grid[0].len() as i64;
    let mut total = 0;

    loop {
        let mut removed = false;
        for i in 0..m {
            for j in 0..n {
                // ignore non-roll cells
                if grid[i as usize][j as usize] != '@' {
                    continue;
                }

                // count the number of rolls in the grid centered at this cell
                let rolls = (i - 1..=i + 1)
                    .flat_map(|u| (j - 1..=j + 1).map(move |v| (u, v)))
                    .filter(|&(u, v)| {
                        (0..m).contains(&u)
                            && (0..n).contains(&v)
                            && grid[u as usize][v as usize] == '@'
                    })
                    .count();

                // if less than 4 neighbors are rolls, this roll is accessible
                // remove it and continue the search
                if rolls < 5 {
                    total += 1;
                    grid[i as usize][j as usize] = '.';
                    removed = true;
                }
            }
        }

        // if no rolls could be removed, we are done
        if !removed {
            return total;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &[&str] = &[
        "..@@.@@@@.",
        "@@@.@.@.@@",
        "@@@@@.@.@@",
        "@.@@@@..@.",
        "@@.@@@@.@@",
        ".@@@@@@@.@",
        ".@.@.@.@@@",
        "@.@@@.@@@@",
        ".@@@@@@@@.",
        "@.@.@@@.@.",
    ];

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 13);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 43);
    }
}
