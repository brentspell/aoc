use std::cmp::min;

pub fn solve() {
    let data = std::fs::read_to_string("data/day13.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    summarize(data, 0)
}

fn part2(data: &str) -> u64 {
    summarize(data, 1)
}

fn summarize(data: &str, diffs: u64) -> u64 {
    data.split("\n\n")
        .map(|data| {
            let p: Vec<Vec<_>> = data.lines().map(|l| l.chars().collect()).collect();
            // first, check for horizontal mirrors with the desired number of diffs
            (0..p.len() - 1)
                .filter(|&i| mirror_diff_horz(&p, i) == diffs)
                .map(|i| (i + 1) * 100)
                .next()
                .unwrap_or_else(|| {
                    // if none was found, find the vertical mirror with the desired number of diffs
                    (0..p[0].len() - 1)
                        .filter(|&j| mirror_diff_vert(&p, j) == diffs)
                        .map(|j| j + 1)
                        .next()
                        .unwrap()
                }) as u64
        })
        .sum()
}

fn mirror_diff_horz(p: &[Vec<char>], i: usize) -> u64 {
    // starting from row i, match rows k before and after i
    (0..min(i + 1, p.len() - i - 1))
        .map(|k| {
            // diff the current row
            (0..p[0].len())
                .map(|j| (p[i - k][j] != p[i + k + 1][j]) as u64)
                .sum::<u64>()
        })
        .sum()
}

fn mirror_diff_vert(p: &[Vec<char>], j: usize) -> u64 {
    // starting from column j, match columns k before and after j
    (0..min(j + 1, p[0].len() - j - 1))
        .map(|k| {
            // diff the current column
            (0..p.len())
                .map(|i| (p[i][j - k] != p[i][j + k + 1]) as u64)
                .sum::<u64>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        #.##..##.\n\
        ..#.##.#.\n\
        ##......#\n\
        ##......#\n\
        ..#.##.#.\n\
        ..##..##.\n\
        #.#.##.#.\n\
        \n\
        #...##..#\n\
        #....#..#\n\
        ..##..###\n\
        #####.##.\n\
        #####.##.\n\
        ..##..###\n\
        #....#..#\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 400);
    }
}
