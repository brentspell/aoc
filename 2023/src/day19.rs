use std::cmp::{max, min};
use std::collections::HashMap;

pub fn solve() {
    let data = std::fs::read_to_string("data/day19.txt").unwrap();

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn part1(data: &str) -> u64 {
    let sections: Vec<_> = data.split("\n\n").collect();
    let rules = parse_rules(sections[0]);

    // filter the list of parts by whether they are accepted by a workflow
    // accumulate their total ratings for the solution
    sections[1]
        .lines()
        .map(Part::parse)
        .filter(|p| {
            // pass the part through the series of workflows
            // in order to determine whether it is accepted
            let mut w = "in";
            loop {
                w = 'workflow: {
                    for r in rules[w].iter() {
                        // evaluate the current condition to see if it matches the rule
                        // if there is no condition, assume a match
                        let o = match &r.cond {
                            Some(c) => match c.operator {
                                Operator::LT if p.get(c.rating) < c.operand => Some(&r.result),
                                Operator::GT if p.get(c.rating) > c.operand => Some(&r.result),
                                _ => None,
                            },
                            None => Some(&r.result),
                        };

                        // continue to the next workflow or accept/reject
                        match &o {
                            None => continue,
                            Some(Result::Next(name)) => break 'workflow &name,
                            Some(Result::Accept) => return true,
                            Some(Result::Reject) => return false,
                        }
                    }
                    w
                }
            }
        })
        .map(|p| p.total())
        .sum::<u64>()
}

fn part2(data: &str) -> u64 {
    let sections: Vec<_> = data.split("\n\n").collect();
    let rules = parse_rules(sections[0]);

    // use DFS to find all acceptance states, restricting the
    // range of allowed ratings based on rule conditions along the way
    let mut total = 0;
    let mut stack = vec![("in", PartRange::new())];
    while let Some((w, state)) = stack.pop() {
        let mut old = state;
        for r in rules[w].iter() {
            // create a copy of the current range values for applying the current rule
            let mut new = old.clone();
            if let Some(cond) = &r.cond {
                let (mut old_lo, mut old_hi) = old.get(cond.rating);
                let (mut new_lo, mut new_hi) = new.get(cond.rating);

                // use the condition to restrict the range of the old/new states
                // new is used to expand the search space based on a match with the condition
                // old is used to continue processing subsequent rules and inverts the condition
                match cond.operator {
                    Operator::LT => {
                        // x < v => old >= v and new < v
                        old_lo = max(old_lo, cond.operand);
                        new_hi = min(new_hi, cond.operand - 1);
                    }
                    Operator::GT => {
                        // x > v => old <= v and new > v
                        old_hi = min(old_hi, cond.operand);
                        new_lo = max(new_lo, cond.operand + 1);
                    }
                }

                old.put(cond.rating, (old_lo, old_hi));
                new.put(cond.rating, (new_lo, new_hi));
            }

            // if we have reached an acceptance state, accumulate the total number of combinations
            // otherwise, expand the search to the next workflow with the updated state
            match &r.result {
                Result::Next(name) => stack.push((&name, new)),
                Result::Accept => total += new.total(),
                _ => {}
            }
        }
    }

    total
}

fn parse_rules(data: &str) -> HashMap<String, Vec<Rule>> {
    data.lines()
        .map(|l| {
            let s: Vec<_> = l[..l.len() - 1].split('{').collect();
            let r: Vec<_> = s[1].split(',').map(Rule::parse).collect();
            (s[0].to_string(), r)
        })
        .collect()
}

struct Rule {
    cond: Option<Cond>,
    result: Result,
}

struct Cond {
    rating: Rating,
    operator: Operator,
    operand: u64,
}

enum Result {
    Next(String),
    Accept,
    Reject,
}

#[derive(Clone, Copy)]
enum Rating {
    X,
    M,
    A,
    S,
}

#[derive(Clone, Copy)]
enum Operator {
    LT,
    GT,
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Clone)]
struct PartRange {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

impl Rule {
    fn parse(s: &str) -> Rule {
        if !s.contains(':') {
            Rule {
                cond: None,
                result: Result::parse(s),
            }
        } else {
            let s: Vec<_> = s.split(':').collect();
            Rule {
                cond: Some(Cond {
                    rating: match &s[0][0..1] {
                        "x" => Rating::X,
                        "m" => Rating::M,
                        "a" => Rating::A,
                        "s" => Rating::S,
                        _ => panic!("invalid rating"),
                    },
                    operator: match &s[0][1..2] {
                        "<" => Operator::LT,
                        ">" => Operator::GT,
                        _ => panic!("invalid operator"),
                    },
                    operand: s[0][2..].parse().unwrap(),
                }),
                result: Result::parse(s[1]),
            }
        }
    }
}

impl Result {
    fn parse(s: &str) -> Result {
        match s {
            "A" => Result::Accept,
            "R" => Result::Reject,
            _ => Result::Next(s.to_string()),
        }
    }
}

impl Part {
    fn parse(s: &str) -> Part {
        let mut p = Part {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };
        for s in s[1..s.len() - 1].split(',') {
            let s: Vec<_> = s.split('=').collect();
            let v: u64 = s[1].parse().unwrap();
            match s[0] {
                "x" => p.x = v,
                "m" => p.m = v,
                "a" => p.a = v,
                "s" => p.s = v,
                _ => panic!("invalid rating"),
            }
        }
        p
    }

    fn get(&self, r: Rating) -> u64 {
        match r {
            Rating::X => self.x,
            Rating::M => self.m,
            Rating::A => self.a,
            Rating::S => self.s,
        }
    }

    fn total(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

impl PartRange {
    fn new() -> PartRange {
        PartRange {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn get(&self, r: Rating) -> (u64, u64) {
        match r {
            Rating::X => self.x,
            Rating::M => self.m,
            Rating::A => self.a,
            Rating::S => self.s,
        }
    }

    fn put(&mut self, r: Rating, v: (u64, u64)) {
        match r {
            Rating::X => self.x = v,
            Rating::M => self.m = v,
            Rating::A => self.a = v,
            Rating::S => self.s = v,
        }
    }

    fn total(&self) -> u64 {
        [self.x, self.m, self.a, self.s]
            .iter()
            .map(|(lo, hi)| max(hi - lo + 1, 0))
            .product()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        px{a<2006:qkq,m>2090:A,rfg}\n\
        pv{a>1716:R,A}\n\
        lnx{m>1548:A,A}\n\
        rfg{s<537:gd,x>2440:R,A}\n\
        qs{s>3448:A,lnx}\n\
        qkq{x<1416:A,crn}\n\
        crn{x>2662:A,R}\n\
        in{s<1351:px,qqz}\n\
        qqz{s>2770:qs,m<1801:hdj,R}\n\
        gd{a>3333:R,R}\n\
        hdj{m>838:A,pv}\n\
        \n\
        {x=787,m=2655,a=1222,s=2876}\n\
        {x=1679,m=44,a=2067,s=496}\n\
        {x=2036,m=264,a=79,s=2244}\n\
        {x=2461,m=1339,a=466,s=291}\n\
        {x=2127,m=1623,a=2188,s=1013}\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(DATA), 19114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(DATA), 167409079868000);
    }
}
