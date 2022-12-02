pub fn solve() {
    let games = parse(&std::fs::read_to_string("data/day02.txt").unwrap());

    // part 1
    println!("part 1: {}", part1(&games));

    // part 2
    println!("part 2: {}", part2(&games));
}

fn parse(string: &str) -> Vec<(String, String)> {
    string
        .lines()
        .map(|l| {
            let v = l.split_whitespace().collect::<Vec<_>>();
            (v[0].to_string(), v[1].to_string())
        })
        .collect()
}

fn part1(games: &[(String, String)]) -> u32 {
    games
        .iter()
        .map(|(p1, p2)| {
            let v1 = ord(p1);
            let v2 = ord(p2);
            let result = if v2 == (v1 + 1) % 3 {
                // win for (opponent + 1) % 3
                6
            } else if v2 == v1 {
                // draw for equal to opponent
                3
            } else {
                // lose otherwise
                0
            };
            result + v2 + 1
        })
        .sum()
}

fn part2(games: &[(String, String)]) -> u32 {
    games
        .iter()
        .map(|(p1, p2)| {
            let v1 = ord(p1);
            match p2.as_str() {
                // need to lose, so choose (opponent - 1) % 3 + 0
                // note mod 2, because rust doesn't do floormod
                "X" => (v1 + 2) % 3 + 1,
                // need to draw, so choose opponent's shape + 3
                "Y" => 3 + v1 + 1,
                // need to win, so choose (opponent + 1) % 3 + 6
                "Z" => 6 + (v1 + 1) % 3 + 1,
                _ => panic!(),
            }
        })
        .sum()
}

fn ord(s: &str) -> u32 {
    match s {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GAMES: &str = "\
        A Y\n\
        B X\n\
        C Z\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(GAMES)), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(GAMES)), 12);
    }
}
