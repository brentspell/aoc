use regex::Regex;
use std::collections::HashMap;
use std::io::prelude::*;
use std::{fs, io};

#[derive(Clone)]
enum Expr {
    Terminal(String),
    Nonterminal(Vec<Vec<u32>>),
}

lazy_static! {
    static ref RULE_REGEX: Regex = Regex::new(r#"^(?P<id>[\d]+): (?P<expr>.*)$"#).unwrap();
}

pub fn solve() {
    let file = fs::File::open("data/day19.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    let split = lines.iter().position(|s| s == "").unwrap();
    let rules: HashMap<u32, Expr> = lines[..split].iter().map(|l| parse(&l)).collect();
    let items: Vec<&str> = lines[split + 1..].iter().map(|l| l.as_str()).collect();

    println!("part 1: {}", part1(&rules, &items));
    println!("part 2: {}", part2(&rules, &items));
}

fn part1(rules: &HashMap<u32, Expr>, items: &[&str]) -> usize {
    let expr = format!("^{}$", expand(0, rules));
    let regex = Regex::new(&expr).unwrap();
    items.iter().filter(|i| regex.is_match(i)).count()
}

// 0: 8 11
// 8: 42 | 42 8
// 11: 42 31 | 42 11 31
// => 0: 42+ (42(?R)31)
fn part2(rules: &HashMap<u32, Expr>, items: &[&str]) -> usize {
    let expr42 = Regex::new(&expand(42, &rules)).unwrap();
    let expr31 = Regex::new(&expand(31, &rules)).unwrap();
    let regex = Regex::new(&format!("^(?P<r42>({})+)(?P<r31>({})+)$", expr42, expr31)).unwrap();
    let mut count = 0;
    for item in items {
        if let Some(caps) = regex.captures(item) {
            let c42 = expr42.find_iter(caps.name("r42").unwrap().as_str()).count();
            let c31 = expr31.find_iter(caps.name("r31").unwrap().as_str()).count();
            if c42 > 1 && c42 > c31 {
                count += 1;
            }
        }
    }

    count
}

fn parse(line: &str) -> (u32, Expr) {
    let caps = RULE_REGEX.captures(line).unwrap();
    let id = caps.name("id").unwrap().as_str().parse().unwrap();
    let expr = caps.name("expr").unwrap().as_str().to_string();
    if expr.starts_with('"') {
        (id, Expr::Terminal(expr.trim_matches('"').to_string()))
    } else {
        (
            id,
            Expr::Nonterminal(
                expr.split('|')
                    .map(|s| s.trim().split(' ').map(|s| s.parse().unwrap()).collect())
                    .collect(),
            ),
        )
    }
}

fn expand(id: u32, rules: &HashMap<u32, Expr>) -> String {
    match &rules[&id] {
        Expr::Terminal(t) => t.to_string(),
        Expr::Nonterminal(alt) => {
            let expr = alt
                .iter()
                .map(|seq| {
                    seq.iter()
                        .map(|&id| expand(id, rules))
                        .collect::<Vec<String>>()
                        .join("")
                })
                .collect::<Vec<String>>()
                .join("|");
            format!("({})", expr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let lines = [
            "0: 4 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            "4: \"a\"",
            "5: \"b\"",
            "",
            "ababbb",
            "bababa",
            "abbbab",
            "aaabbb",
            "aaaabbb",
        ];
        let split = lines.iter().position(|&s| s == "").unwrap();
        let rules: HashMap<u32, Expr> = lines[..split].iter().map(|l| parse(&l)).collect();
        let items: Vec<&str> = lines[split + 1..].to_vec();

        assert_eq!(part1(&rules, &items), 2);

        let lines = [
            "42: 9 14 | 10 1",
            "9: 14 27 | 1 26",
            "10: 23 14 | 28 1",
            "1: \"a\"",
            "11: 42 31",
            "5: 1 14 | 15 1",
            "19: 14 1 | 14 14",
            "12: 24 14 | 19 1",
            "16: 15 1 | 14 14",
            "31: 14 17 | 1 13",
            "6: 14 14 | 1 14",
            "2: 1 24 | 14 4",
            "0: 8 11",
            "13: 14 3 | 1 12",
            "15: 1 | 14",
            "17: 14 2 | 1 7",
            "23: 25 1 | 22 14",
            "28: 16 1",
            "4: 1 1",
            "20: 14 14 | 1 15",
            "3: 5 14 | 16 1",
            "27: 1 6 | 14 18",
            "14: \"b\"",
            "21: 14 1 | 1 14",
            "25: 1 1 | 1 14",
            "22: 14 14",
            "8: 42",
            "26: 14 22 | 1 20",
            "18: 15 15",
            "7: 14 5 | 1 21",
            "24: 14 1",
            "",
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ];
        let split = lines.iter().position(|&s| s == "").unwrap();
        let rules: HashMap<u32, Expr> = lines[..split].iter().map(|l| parse(&l)).collect();
        let items: Vec<&str> = lines[split + 1..].to_vec();

        assert_eq!(part2(&rules, &items), 12);
    }
}
