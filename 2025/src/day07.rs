use cached::{UnboundCache, proc_macro::cached};

pub fn solve() {
    let data = std::fs::read_to_string("data/day07.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    // part 1
    println!("part 1: {}", part1(&lines));

    // part 2
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[&str]) -> u64 {
    // track the current beam states and count the number of times a beam splits
    let mut splits = 0;
    let mut beams = vec!['.'; lines[0].len()];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == 'S' {
                // start the initial beam
                beams[i] = '|';
            } else if c == '^' && beams[i] == '|' {
                // split the current beam into two new beams
                beams[i - 1] = '|';
                beams[i] = '.';
                beams[i + 1] = '|';
                splits += 1;
            }
        }
    }

    splits
}

fn part2(lines: &[&str]) -> u64 {
    let lines: Vec<Vec<char>> = lines.iter().map(|l| l.chars().collect()).collect();
    (0..lines[0].len())
        .map(|j| search(&lines, lines.len() - 1, j))
        .sum()
}

// DP: memoize the search function
#[cached(
    ty = "UnboundCache<(usize, usize), u64>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ (i, j) }"#
)]
fn search(lines: &[Vec<char>], i: usize, j: usize) -> u64 {
    // base case: the starting beam
    if i == 0 {
        return if lines[i][j] == 'S' { 1 } else { 0 };
    }

    let mut total = 0;
    // if the previous cell was not a split, count the beams that landed there
    if lines[i - 1][j] != '^' {
        total += search(lines, i - 1, j);
    }
    // if we landed here from a split on the left/right, count the beams that landed there
    if j > 0 && lines[i - 1][j - 1] == '^' {
        total += search(lines, i - 1, j - 1);
    }
    if j < lines[0].len() - 1 && lines[i - 1][j + 1] == '^' {
        total += search(lines, i - 1, j + 1);
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &[&str] = &[
        ".......S.......",
        "...............",
        ".......^.......",
        "...............",
        "......^.^......",
        "...............",
        ".....^.^.^.....",
        "...............",
        "....^.^...^....",
        "...............",
        "...^.^...^.^...",
        "...............",
        "..^...^.....^..",
        "...............",
        ".^.^.^.^.^...^.",
        "...............",
    ];

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 40);
    }
}
