use std::collections::{HashSet, VecDeque};
use std::io::prelude::*;
use std::{fs, io};

pub fn solve() {
    let file = fs::File::open("data/day22.txt").unwrap();
    let lines: Vec<_> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let players = parse(&lines);

    println!("part 1: {}", part1(&players));
    println!("part 2: {}", part2(&players).1);
}

fn part1(players: &[VecDeque<u32>]) -> u32 {
    let mut players = players.to_vec();
    while players.iter().all(|p| !p.is_empty()) {
        let tops: Vec<_> = players.iter_mut().map(|d| d.pop_front().unwrap()).collect();
        let winner = tops.iter().enumerate().max_by_key(|&(_i, c)| c).unwrap().0;
        players[winner].push_back(tops[winner]);
        players[winner].push_back(tops[tops.len() - winner - 1]);
    }

    score(&players.iter().max_by_key(|&d| d.len()).unwrap())
}

fn part2(players: &[VecDeque<u32>]) -> (usize, u32) {
    let mut players = players.to_vec();
    let mut unique: HashSet<Vec<VecDeque<u32>>> = HashSet::new();
    while players.iter().all(|p| !p.is_empty()) {
        // abort if we have seen this configuration already
        if !unique.insert(players.clone()) {
            return (0, score(&players[0]));
        }
        let tops: Vec<_> = players.iter_mut().map(|d| d.pop_front().unwrap()).collect();
        let winner = if tops
            .iter()
            .enumerate()
            .all(|(i, &c)| players[i].len() >= c as usize)
        {
            // sub-game round
            part2(
                &players
                    .iter()
                    .zip(tops.iter().copied())
                    .map(|(d, c)| d.iter().copied().take(c as usize).collect())
                    .collect::<Vec<_>>(),
            )
            .0
        } else {
            // regular round
            tops.iter().enumerate().max_by_key(|&(_i, c)| c).unwrap().0
        };
        players[winner].push_back(tops[winner]);
        players[winner].push_back(tops[tops.len() - winner - 1]);
    }

    let winner = players
        .iter()
        .enumerate()
        .max_by_key(|&(_i, d)| d.len())
        .unwrap()
        .0;
    (winner, score(&players[winner]))
}

fn parse(lines: &[String]) -> Vec<VecDeque<u32>> {
    lines
        .split(|l| l == "")
        .map(|v| v[1..].iter().map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn score(hand: &VecDeque<u32>) -> u32 {
    hand.iter()
        .enumerate()
        .map(|(i, &c)| c * (hand.len() - i) as u32)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let lines = [
            String::from("Player 1:"),
            String::from("9"),
            String::from("2"),
            String::from("6"),
            String::from("3"),
            String::from("1"),
            String::from(""),
            String::from("Player 2:"),
            String::from("5"),
            String::from("8"),
            String::from("4"),
            String::from("7"),
            String::from("10"),
        ];
        let players = parse(&lines);

        assert_eq!(part1(&players), 306);
        assert_eq!(part2(&players).1, 291);
    }
}
