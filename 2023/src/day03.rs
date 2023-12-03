use std::collections::HashSet;

pub fn solve() {
    let data = std::fs::read_to_string("data/day03.txt").unwrap();
    let lines: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();

    // part 1
    println!("part 1: {}", part1(&lines));

    // part 2
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[Vec<char>]) -> u32 {
    let mut sum = 0;
    let mut done: HashSet<(usize, usize)> = HashSet::new();

    // process each row and column
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            // only process symbols, no numbers or dots
            if lines[i][j] == '.' || lines[i][j].is_ascii_digit() {
                continue;
            }

            // enumerate all neighbors
            for u in -1..=1 {
                for v in -1..=1 {
                    // ignore any positions outside the grid
                    let (p, q) = (i as i32 + u, j as i32 + v);
                    if p < 0 || p >= lines.len() as i32 || q < 0 || q >= lines[0].len() as i32 {
                        continue;
                    }

                    // ignore any non-numeric neighbors
                    let (p, mut q) = (p as usize, q as usize);
                    if !lines[p][q].is_ascii_digit() {
                        continue;
                    }

                    // find the beginning of the number
                    while q > 0 && lines[p][q - 1].is_ascii_digit() {
                        q -= 1;
                    }

                    // the same number can be adjacent to more than one symbol,
                    // so maintain a set of their positions and accumulate on each unique
                    if done.insert((p, q)) {
                        sum += parse_part(&lines[p][q..]);
                    }
                }
            }
        }
    }

    sum
}

fn part2(lines: &[Vec<char>]) -> u32 {
    let mut sum = 0;
    let mut parts: HashSet<(usize, usize)> = HashSet::new();

    // process each row and column
    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            // only process gear symbols
            if lines[i][j] != '*' {
                continue;
            }

            // enumerate all neighbors
            for u in -1..=1 {
                for v in -1..=1 {
                    // ignore any positions outside the grid
                    let (p, q) = (i as i32 + u, j as i32 + v);
                    if p < 0 || p >= lines.len() as i32 || q < 0 || q >= lines[0].len() as i32 {
                        continue;
                    }

                    // ignore any non-numeric neighbors
                    let (p, mut q) = (p as usize, q as usize);
                    if !lines[p][q].is_ascii_digit() {
                        continue;
                    }

                    // find the beginning of the number
                    while q > 0 && lines[p][q - 1].is_ascii_digit() {
                        q -= 1;
                    }

                    // add the part to the gear
                    parts.insert((p, q));
                }
            }

            // if the gear is connected to two parts, compute the ratio (product)
            if parts.len() == 2 {
                sum += parts
                    .iter()
                    .map(|&(p, q)| parse_part(&lines[p][q..]))
                    .product::<u32>();
            }
            parts.clear();
        }
    }

    sum
}

fn parse_part(s: &[char]) -> u32 {
    s.iter()
        .map_while(|c| c.to_digit(10))
        .fold(0, |a, x| a * 10 + x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
        let lines: Vec<Vec<char>> = data.iter().map(|l| l.chars().collect()).collect();
        assert_eq!(part1(&lines), 4361);
    }

    #[test]
    fn test_part2() {
        let data = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ];
        let lines: Vec<Vec<char>> = data.iter().map(|l| l.chars().collect()).collect();
        assert_eq!(part2(&lines), 467835);
    }
}
