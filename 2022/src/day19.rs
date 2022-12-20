use regex::Regex;
use std::cmp::max;
use std::collections::HashSet;

#[derive(Debug)]
struct Blueprint {
    ore_ore: u8,
    cla_ore: u8,
    obs_ore: u8,
    obs_cla: u8,
    geo_ore: u8,
    geo_obs: u8,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct State {
    res_ore: u8,
    res_cla: u8,
    res_obs: u8,
    res_geo: u8,
    bot_ore: u8,
    bot_cla: u8,
    bot_obs: u8,
    time: u8,
}

pub fn solve() {
    let data = std::fs::read_to_string("data/day19.txt").expect("data-file");

    // part 1
    println!("part 1: {}", part1(&parse(&data)));

    // part 2
    println!("part 2: {}", part2(&parse(&data)));
}

fn parse(data: &str) -> Vec<Blueprint> {
    data.lines()
        .map(|l| {
            let v: Vec<u8> = LINE_REGEX
                .captures(l)
                .expect("line-match")
                .iter()
                .skip(2)
                .map(|c| c.expect("cost").as_str().parse().expect("cost"))
                .collect();
            if v.len() != 6 {
                panic!("invalid-costs");
            }
            Blueprint {
                ore_ore: v[0],
                cla_ore: v[1],
                obs_ore: v[2],
                obs_cla: v[3],
                geo_ore: v[4],
                geo_obs: v[5],
            }
        })
        .collect()
}

fn part1(bps: &[Blueprint]) -> usize {
    bps.iter()
        .map(|bp| search(bp, 24))
        .enumerate()
        .map(|(i, g)| (i + 1) * g as usize)
        .sum()
}

fn part2(bps: &[Blueprint]) -> usize {
    bps.iter()
        .take(3)
        .map(|bp| search(bp, 32) as usize)
        .product()
}

fn search(bp: &Blueprint, time: u8) -> u8 {
    let state = State {
        res_ore: 0,
        res_cla: 0,
        res_obs: 0,
        res_geo: 0,
        bot_ore: 1,
        bot_cla: 0,
        bot_obs: 0,
        time,
    };
    let mut done = HashSet::new();
    let mut best = 0;
    let mut stack = vec![state];
    while !stack.is_empty() {
        let state = stack.pop().expect("stack-entry");

        // track the maximum number of geodes opened
        best = max(best, state.res_geo);
        if state.time == 0 {
            continue;
        }

        // build an ore robot
        // heuristic: we don't need more than the maximum from the blueprint
        let max_ore = max(max(max(bp.ore_ore, bp.cla_ore), bp.obs_ore), bp.geo_ore);
        if state.bot_ore < max_ore {
            // wait until we can build
            let mut s = state;
            while s.time > 0 && s.res_ore < bp.ore_ore {
                s.step();
            }

            // construct the robot
            if s.time > 0 {
                s.res_ore -= bp.ore_ore;
                s.step();
                s.bot_ore += 1;
                if done.insert(s) {
                    stack.push(s);
                }
            }
        }

        // build a clay robot
        // heuristic: we don't need more than are needed to make an obsidian robot
        if state.bot_cla < bp.obs_cla {
            // wait until we can build
            let mut s = state;
            while s.time > 0 && s.res_ore < bp.cla_ore {
                s.step();
            }

            // construct the robot
            if s.time > 0 {
                s.res_ore -= bp.cla_ore;
                s.step();
                s.bot_cla += 1;
                if done.insert(s) {
                    stack.push(s);
                }
            }
        }

        // build an obsidian robot
        if state.bot_cla > 0 {
            // wait until we can build
            let mut s = state;
            while s.time > 0 && (s.res_cla < bp.obs_cla || s.res_ore < bp.obs_ore) {
                s.step();
            }

            // construct the robot
            if s.time > 0 {
                s.res_ore -= bp.obs_ore;
                s.res_cla -= bp.obs_cla;
                s.step();
                s.bot_obs += 1;
                if done.insert(s) {
                    stack.push(s);
                }
            }
        }

        // build a geode robot
        if state.bot_obs > 0 {
            // wait until we can build
            let mut s = state;
            while s.time > 0 && (s.res_obs < bp.geo_obs || s.res_ore < bp.geo_ore) {
                s.step();
            }

            // construct the robot
            if s.time > 0 {
                s.res_ore -= bp.geo_ore;
                s.res_obs -= bp.geo_obs;
                s.step();
                // instead of tracking the robot count, go ahead and compute its output
                s.res_geo += s.time;
                if done.insert(s) {
                    stack.push(s);
                }
            }
        }
    }

    best
}

impl State {
    fn step(&mut self) {
        self.res_ore += self.bot_ore;
        self.res_cla += self.bot_cla;
        self.res_obs += self.bot_obs;
        self.time -= 1;
    }
}

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(
        r#"^Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.$"#
    )
    .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        Blueprint 1: \
            Each ore robot costs 4 ore. \
            Each clay robot costs 2 ore. \
            Each obsidian robot costs 3 ore and 14 clay. \
            Each geode robot costs 2 ore and 7 obsidian.\n\
        Blueprint 2: \
            Each ore robot costs 2 ore. \
            Each clay robot costs 3 ore. \
            Each obsidian robot costs 3 ore and 8 clay. \
            Each geode robot costs 3 ore and 12 obsidian.\n\
    ";

    #[test]
    fn test_part1() {
        let bps = parse(DATA);
        assert_eq!(search(&bps[0], 24), 9);
        assert_eq!(search(&bps[1], 24), 12);
        assert_eq!(part1(&bps), 33);
    }

    #[test]
    fn test_part2() {
        let bps = parse(DATA);
        assert_eq!(search(&bps[0], 32), 56);
        assert_eq!(search(&bps[1], 32), 62);
    }
}
