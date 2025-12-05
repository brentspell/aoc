pub fn solve() {
    let data = std::fs::read_to_string("data/day05.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    // part 1
    println!("part 1: {}", part1(&lines));

    // part 2
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[&str]) -> u64 {
    let (fresh, avail) = parse(lines);

    // count the number of available ingredients that are in a fresh range
    avail
        .iter()
        .filter(|&id| fresh.iter().any(|&(b, e)| (b..=e).contains(id)))
        .count() as u64
}

fn part2(lines: &[&str]) -> u64 {
    let (fresh, _avail) = parse(lines);

    // merge (potentially overlapping) ingredient ranges into disjoint ranges
    let mut old = vec![];
    for (mut b1, mut e1) in fresh.iter().copied() {
        // merge the incoming range with any overlapping existing ranges
        let mut new = vec![];
        for &(b2, e2) in old.iter() {
            if b1 <= e2 && b2 <= e1 {
                // merge the ranges into the incoming range
                (b1, e1) = (b1.min(b2), e1.max(e2));
            } else {
                // if the current range doesn't overlap, keep it
                new.push((b2, e2));
            }
        }

        new.push((b1, e1));
        old = new;
    }

    // now that the ranges are disjoint, we can simply sum their lengths
    old.iter().map(|&(b, e)| e - b + 1).sum()
}

fn parse(lines: &[&str]) -> (Vec<(u64, u64)>, Vec<u64>) {
    let mut i = lines.iter();

    let fresh = i
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            let (beg, end) = l.split_once('-').unwrap();
            (beg.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
        })
        .collect();

    let avail = i.by_ref().map(|l| l.parse::<u64>().unwrap()).collect();

    (fresh, avail)
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &[&str] = &[
        "3-5", "10-14", "16-20", "12-18", "", "1", "5", "8", "11", "17", "32",
    ];

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 14);
    }
}
