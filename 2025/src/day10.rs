use std::collections::HashSet;

pub fn solve() {
    let data = std::fs::read_to_string("data/day10.txt").unwrap();
    let lines: Vec<&str> = data.lines().collect();

    // part 1
    println!("part 1: {}", part1(&lines));

    // part 2
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[&str]) -> u64 {
    let mut total = 0;

    for line in lines {
        let toks = line.split_whitespace().collect::<Vec<_>>();

        // parse on/off goal states
        let mut goal = 0;
        for c in toks[0][1..toks[0].len() - 1].chars().rev() {
            goal <<= 1;
            if c == '#' {
                goal |= 1;
            }
        }

        // parse button configuration
        let buttons = toks[1..toks.len() - 1]
            .iter()
            .map(|&t| {
                t[1..t.len() - 1]
                    .split(",")
                    .map(|s| s.parse::<u64>().unwrap())
                    .fold(0, |a, b| a | (1 << b))
            })
            .collect::<Vec<_>>();

        // search for the number of presses to reach the goal state
        let mut state = HashSet::from([0]);
        let mut n = 0;
        while !state.contains(&goal) {
            let mut new = HashSet::new();
            for b in buttons.iter() {
                for s in state.iter().copied() {
                    new.insert(s ^ b);
                }
            }
            n += 1;
            state = new;
        }

        total += n;
    }

    total
}

fn part2(lines: &[&str]) -> u64 {
    let mut total = 0;

    for line in lines {
        let toks = line.split_whitespace().collect::<Vec<_>>();

        // parse joltage goals
        let goal_str = toks[toks.len() - 1];
        let goal = goal_str[1..goal_str.len() - 1]
            .split(',')
            .map(|s| s.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        // parse button configuration
        let buttons = toks[1..toks.len() - 1]
            .iter()
            .map(|&t| {
                t[1..t.len() - 1]
                    .split(",")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // create variables for the number of presses of each button
        let solver = z3::Optimize::new();
        let vars = (0..buttons.len())
            .map(|_| z3::ast::Int::fresh_const("x"))
            .collect::<Vec<_>>();

        // disallow negative presses
        for v in vars.iter() {
            solver.assert(&v.ge(0));
        }

        // apply the button presses to the joltage increments
        for (i, &jolts) in goal.iter().enumerate() {
            solver.assert(
                &(0..buttons.len())
                    .filter(|&k| buttons[k].contains(&i))
                    .map(|k| &vars[k])
                    .sum::<z3::ast::Int>()
                    .eq(jolts),
            );
        }

        // solve for the minimum total number of button presses
        let sum = vars.iter().sum::<z3::ast::Int>();
        solver.minimize(&sum);
        solver.check(&[]);

        // accumulate the results
        total += solver
            .get_model()
            .unwrap()
            .eval(&sum, true)
            .unwrap()
            .as_u64()
            .unwrap();
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &[&str] = &[
        "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}",
        "[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}",
        "[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}",
    ];

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 33);
    }
}
