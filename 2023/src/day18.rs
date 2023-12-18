use std::cmp::{max, min};
use std::collections::HashSet;

pub fn solve() {
    let data = std::fs::read_to_string("data/day18.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    // for part 1, we can compute a bounding box around the trench
    // and find all points outside the trench to calculate those inside
    let mut trench = HashSet::new();
    let (mut i, mut j) = (0_i64, 0_i64);
    let (mut mini, mut maxi, mut minj, mut maxj) = (0, 0, 0, 0);
    trench.insert((i, j));

    // first, trace the trench and collect all its points
    for line in data.lines() {
        // parse the input data and calculate the next vertices
        let split: Vec<_> = line.split_whitespace().collect();
        let d = split[0];
        let n: i64 = split[1].parse().unwrap();
        for _ in 0..n {
            match d {
                "U" => i -= 1,
                "R" => j += 1,
                "D" => i += 1,
                "L" => j -= 1,
                _ => panic!("invalid direction"),
            }

            // track each point on the trench and expand the bounding box
            trench.insert((i, j));
            mini = min(mini, i);
            maxi = max(maxi, i);
            minj = min(minj, j);
            maxj = max(maxj, j);
        }
    }

    // expand the bounding box to make the exterior contiguous
    mini -= 1;
    minj -= 1;
    maxi += 1;
    maxj += 1;

    // use DFS to find all points in the space exterior to the trench
    let mut stack = vec![(mini, minj)];
    let mut outside: HashSet<_> = stack.iter().copied().collect();
    while let Some((i, j)) = stack.pop() {
        for (i, j) in [(i - 1, j), (i, j + 1), (i + 1, j), (i, j - 1)] {
            if (mini..=maxi).contains(&i)
                && (minj..=maxj).contains(&j)
                && !trench.contains(&(i, j))
                && !outside.contains(&(i, j))
            {
                outside.insert((i, j));
                stack.push((i, j));
            }
        }
    }

    // the number of interior points is the size of the bounding box
    // minus the number of points outside the trench
    ((maxi - mini + 1) * (maxj - minj + 1)) as u64 - outside.len() as u64
}

fn part2(data: &str) -> u64 {
    // here, the trench is much too large to enumerate the points within or without it
    // so we use the Shoelace Formula (https://en.wikipedia.org/wiki/Shoelace_formula)
    // to calculate the interior area and
    // Pick's Theorem (https://en.wikipedia.org/wiki/Pick%27s_theorem) to calculate
    // the total area given the interior area and the perimeter
    let mut area = 0;
    let mut perim = 0;
    let (mut i, mut j) = (0_i64, 0_i64);
    for line in data.lines() {
        // parse the input data and calculate the next vertex
        let tok = &line[line.len() - 7..line.len() - 1];
        let n = i64::from_str_radix(&tok[0..5], 16).unwrap();
        let d = &tok[5..];
        let (p, q) = match d {
            "3" => (i - n, j),
            "0" => (i, j + n),
            "1" => (i + n, j),
            "2" => (i, j - n),
            _ => panic!("invalid direction"),
        };

        // accumulate the perimeter and use the Shoelace Formula to accumulate the area
        perim += n;
        area += (i + p) * (j - q);

        (i, j) = (p, q);
    }

    // use Pick's Theorem to compute the final area
    (area.unsigned_abs() as f64 / 2.0).ceil() as u64 + (perim as f64 / 2.0).ceil() as u64 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        R 6 (#70c710)\n\
        D 5 (#0dc571)\n\
        L 2 (#5713f0)\n\
        D 2 (#d2c081)\n\
        R 2 (#59c680)\n\
        D 2 (#411b91)\n\
        L 5 (#8ceee2)\n\
        U 2 (#caa173)\n\
        L 1 (#1b58a2)\n\
        U 2 (#caa171)\n\
        R 2 (#7807d2)\n\
        U 3 (#a77fa3)\n\
        L 2 (#015232)\n\
        U 2 (#7a21e3)\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 62);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 952408144115);
    }
}
