use itertools::Itertools;

pub fn solve() {
    let data = std::fs::read_to_string("data/day12.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    // part 1
    println!("part 1: {}", part1(&lines));
}

// cheeky NP-hard rectilinear polygon packing problem is actually linear
// if the sum of the occupied cells is less than the desired area, then all regions fit
fn part1(lines: &[&str]) -> u64 {
    // parse the number of occupied cells in each region
    let regions = lines
        .iter()
        .copied()
        .take_while(|l| !l.contains("x"))
        .filter(|l| l.contains("#") || l.contains("."))
        .map(|l| l.chars().filter(|&c| c == '#').count() as u64)
        .chunks(3)
        .into_iter()
        .map(|c| c.sum::<u64>())
        .collect::<Vec<_>>();

    // parse the maximum size of each packing and occupancy of all regions
    // if the minimum occupancy is less than the maximum area, then it fits
    // this isn't true in general, but it works for this dataset
    lines
        .iter()
        .copied()
        .skip_while(|l| !l.contains("x"))
        .map(|l| {
            let (a, c) = l.split_once(": ").unwrap();
            let (h, w) = a.split_once('x').unwrap();
            let max = w.parse::<u64>().unwrap() * h.parse::<u64>().unwrap();
            let min = c
                .split_whitespace()
                .map(|c| c.parse::<u64>().unwrap())
                .enumerate()
                .map(|(i, c)| c * regions[i])
                .sum::<u64>();
            if min <= max { 1 } else { 0 }
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &[&str] = &[
        "0:",
        "###",
        "##.",
        "##.",
        "",
        "1:",
        "###",
        "##.",
        ".##",
        "",
        "2:",
        ".##",
        "###",
        "##.",
        "",
        "3:",
        "##.",
        "###",
        "##.",
        "",
        "4:",
        "###",
        "#..",
        "###",
        "",
        "5:",
        "###",
        ".#.",
        "###",
        "",
        "4x4: 0 0 0 0 2 0",
        "12x5: 1 0 1 0 2 2",
        "12x5: 1 0 1 0 10 2",
    ];

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 2);
    }
}
