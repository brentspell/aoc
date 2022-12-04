use std::ops::RangeInclusive;

type Sections = RangeInclusive<u32>;

pub fn solve() {
    let data = std::fs::read_to_string("data/day04.txt").unwrap();
    let sections = parse(&data);

    // part 1
    println!("part 1: {}", part1(&sections));

    // part 2
    println!("part 2: {}", part2(&sections));
}

fn parse(string: &str) -> Vec<(Sections, Sections)> {
    string
        .lines()
        .map(|l| {
            let v: Vec<u32> = l
                .split(',')
                .flat_map(|s| s.split('-'))
                .map(|s| s.parse().unwrap())
                .collect();
            (v[0]..=v[1], v[2]..=v[3])
        })
        .collect()
}

fn part1(sections: &[(Sections, Sections)]) -> u32 {
    sections
        .iter()
        .map(|(x, y)| {
            (x.contains(y.start()) && x.contains(y.end())
                || y.contains(x.start()) && y.contains(x.end())) as u32
        })
        .sum()
}

fn part2(sections: &[(Sections, Sections)]) -> u32 {
    sections
        .iter()
        .map(|(x, y)| (y.contains(x.start()) || x.contains(y.start())) as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        2-4,6-8\n\
        2-3,4-5\n\
        5-7,7-9\n\
        2-8,3-7\n\
        6-6,4-6\n\
        2-6,4-8\n\
    ";

    lazy_static! {
        static ref INTERVALS: Vec<(Sections, Sections)> = parse(DATA);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&INTERVALS), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&INTERVALS), 4);
    }
}
