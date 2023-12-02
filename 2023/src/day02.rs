use std::cmp::max;

pub fn solve() {
    let data = std::fs::read_to_string("data/day02.txt").unwrap();
    let games: Vec<_> = data.lines().map(Game::parse).collect();

    // part 1
    println!("part 1: {}", part1(&games));

    // part 2
    println!("part 2: {}", part2(&games));
}

fn part1(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            let valid = game.draws.iter().all(|d| match *d {
                Draw::R(n) => n <= 12,
                Draw::G(n) => n <= 13,
                Draw::B(n) => n <= 14,
            });
            if valid {
                game.id
            } else {
                0
            }
        })
        .sum()
}

fn part2(games: &[Game]) -> u32 {
    games
        .iter()
        .map(|game| {
            let (r, g, b) = game.draws.iter().fold((0, 0, 0), |(r, g, b), d| match *d {
                Draw::R(n) => (max(r, n), g, b),
                Draw::G(n) => (r, max(g, n), b),
                Draw::B(n) => (r, g, max(b, n)),
            });

            r * g * b
        })
        .sum()
}

struct Game {
    id: u32,
    draws: Vec<Draw>,
}

enum Draw {
    R(u32),
    G(u32),
    B(u32),
}

impl Game {
    fn parse(line: &str) -> Game {
        let s: Vec<_> = line[5..].split(": ").collect();
        Game {
            id: s[0].parse().unwrap(),
            draws: s[1]
                .split("; ")
                .flat_map(|s| s.split(", "))
                .map(|s| {
                    let s: Vec<_> = s.split(' ').collect();
                    let n = s[0].parse().unwrap();
                    match s[1] {
                        "red" => Draw::R(n),
                        "green" => Draw::G(n),
                        "blue" => Draw::B(n),
                        _ => panic!("invalid color"),
                    }
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let data = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let games = data.map(Game::parse);
        assert_eq!(part1(&games), 8);
    }

    #[test]
    fn test_part2() {
        let data = [
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];
        let games = data.map(Game::parse);
        assert_eq!(part2(&games), 2286);
    }
}
