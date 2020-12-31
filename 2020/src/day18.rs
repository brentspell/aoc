use std::io::prelude::*;
use std::{fs, io};

pub fn solve() {
    let file = fs::File::open("data/day18.txt").unwrap();
    let lines: Vec<String> = io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();

    println!("part 1: {}", part1(&lines));
    println!("part 2: {}", part2(&lines));
}

fn part1(lines: &[String]) -> u64 {
    lines.iter().map(|l| eval(&parse1(&l))).sum()
}

fn part2(lines: &[String]) -> u64 {
    lines.iter().map(|l| eval(&parse2(&l))).sum()
}

// convert to RPN, no precedence
fn parse1(line: &str) -> String {
    let mut stack: Vec<char> = Vec::new();
    let mut queue = String::new();
    for ch in line.chars() {
        if ch.is_digit(10) {
            queue.push(ch);
        } else if ch == '+' || ch == '*' {
            while !stack.is_empty() && *stack.last().unwrap() != '(' {
                queue.push(stack.pop().unwrap());
            }
            stack.push(ch);
        } else if ch == '(' {
            stack.push(ch);
        } else if ch == ')' {
            loop {
                let ch = stack.pop().unwrap();
                if ch == '(' {
                    break;
                }
                queue.push(ch);
            }
        }
    }
    while !stack.is_empty() {
        queue.push(stack.pop().unwrap());
    }

    queue
}

// convert to RPN, + precedence over *
fn parse2(line: &str) -> String {
    let mut stack: Vec<char> = Vec::new();
    let mut queue = String::new();
    for ch in line.chars() {
        if ch.is_digit(10) {
            queue.push(ch);
        } else if ch == '+' || ch == '*' {
            while !stack.is_empty() {
                let top = *stack.last().unwrap();
                if top == '(' || (top == '*' && ch == '+') {
                    break;
                }
                queue.push(stack.pop().unwrap());
            }
            stack.push(ch);
        } else if ch == '(' {
            stack.push(ch);
        } else if ch == ')' {
            loop {
                let ch = stack.pop().unwrap();
                if ch == '(' {
                    break;
                }
                queue.push(ch);
            }
        }
    }
    while !stack.is_empty() {
        queue.push(stack.pop().unwrap());
    }

    queue
}

// evaluate an RPN expression
fn eval(line: &str) -> u64 {
    let mut stack: Vec<u64> = Vec::new();
    for ch in line.chars() {
        if ch == '+' {
            let r = stack.pop().unwrap();
            let l = stack.pop().unwrap();
            stack.push(r + l);
        } else if ch == '*' {
            let r = stack.pop().unwrap();
            let l = stack.pop().unwrap();
            stack.push(r * l);
        } else {
            stack.push(ch.to_string().parse().unwrap());
        }
    }

    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(eval(&parse1("1 + 2 * 3 + 4 * 5 + 6")), 71);
        assert_eq!(eval(&parse1("1 + (2 * 3) + (4 * (5 + 6))")), 51);
        assert_eq!(eval(&parse1("2 * 3 + (4 * 5)")), 26);
        assert_eq!(eval(&parse1("5 + (8 * 3 + 9 + 3 * 4 * 3)")), 437);
        assert_eq!(
            eval(&parse1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")),
            12240
        );
        assert_eq!(
            eval(&parse1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")),
            13632
        );

        assert_eq!(eval(&parse2("1 + 2 * 3 + 4 * 5 + 6")), 231);
        assert_eq!(eval(&parse2("1 + (2 * 3) + (4 * (5 + 6))")), 51);
        assert_eq!(eval(&parse2("2 * 3 + (4 * 5)")), 46);
        assert_eq!(eval(&parse2("5 + (8 * 3 + 9 + 3 * 4 * 3)")), 1445);
        assert_eq!(
            eval(&parse2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")),
            669060
        );
        assert_eq!(
            eval(&parse2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")),
            23340
        );
    }
}
