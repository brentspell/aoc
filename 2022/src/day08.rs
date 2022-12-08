use std::collections::HashSet;

pub fn solve() {
    let data = parse(&std::fs::read_to_string("data/day08.txt").expect("data-file"));

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn parse(data: &str) -> Vec<Vec<u32>> {
    data.lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).expect("digit")).collect())
        .collect()
}

#[allow(clippy::needless_range_loop)]
fn part1(heights: &[Vec<u32>]) -> usize {
    let m = heights.len();
    let n = heights[0].len();
    let mut visible: HashSet<(usize, usize)> = HashSet::new();

    // test all rows
    for i in 0..m {
        // test left-to-right
        visible.insert((i, 0));
        let mut max = heights[i][0];
        for j in 1..n - 1 {
            if heights[i][j] > max {
                max = heights[i][j];
                visible.insert((i, j));
            }
        }

        // test right-to-left
        visible.insert((i, n - 1));
        let mut max = heights[i][n - 1];
        for j in (1..n - 1).rev() {
            if heights[i][j] > max {
                max = heights[i][j];
                visible.insert((i, j));
            }
        }
    }

    // test all columns
    for j in 0..n {
        // test top-to-bottom
        visible.insert((0, j));
        let mut max = heights[0][j];
        for i in 1..m - 1 {
            if heights[i][j] > max {
                max = heights[i][j];
                visible.insert((i, j));
            }
        }

        // test bottom-to-top
        visible.insert((m - 1, j));
        let mut max = heights[m - 1][j];
        for i in (1..m - 1).rev() {
            if heights[i][j] > max {
                max = heights[i][j];
                visible.insert((i, j));
            }
        }
    }

    visible.len()
}

fn part2(heights: &[Vec<u32>]) -> usize {
    let m = heights.len();
    let n = heights[0].len();
    let mut max = 0;

    for i in 0..m {
        for j in 0..n {
            let mut score = 1;

            // tally to the top
            let mut count = 0;
            for p in (0..i).rev() {
                count += 1;
                if heights[p][j] >= heights[i][j] {
                    break;
                }
            }
            score *= count;

            // tally to the bottom
            let mut count = 0;
            for p in i + 1..m {
                count += 1;
                if heights[p][j] >= heights[i][j] {
                    break;
                }
            }
            score *= count;

            // tally to the left
            let mut count = 0;
            for q in (0..j).rev() {
                count += 1;
                if heights[i][q] >= heights[i][j] {
                    break;
                }
            }
            score *= count;

            // tally to the right
            let mut count = 0;
            for q in j + 1..n {
                count += 1;
                if heights[i][q] >= heights[i][j] {
                    break;
                }
            }
            score *= count;

            // keep the highest score
            if score > max {
                max = score;
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        30373\n\
        25512\n\
        65332\n\
        33549\n\
        35390\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(DATA)), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(DATA)), 8);
    }
}
