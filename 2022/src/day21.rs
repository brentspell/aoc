use std::collections::{HashMap, VecDeque};

type Num = i64;

enum Expr {
    Lit(Num),
    Binary(String, Op, String),
}

enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

pub fn solve() {
    let data = parse(&std::fs::read_to_string("data/day21.txt").expect("data-file"));

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn parse(data: &str) -> HashMap<String, Expr> {
    data.lines()
        .map(|l| {
            let (k, v) = l.split_once(": ").expect("valid-expr");
            (
                k.to_string(),
                match v.parse::<Num>() {
                    Ok(x) => Expr::Lit(x),
                    _ => {
                        let v: Vec<_> = v.split(' ').collect();
                        assert!(v.len() == 3);
                        let op = match v[1] {
                            "+" => Op::Add,
                            "-" => Op::Sub,
                            "*" => Op::Mul,
                            "/" => Op::Div,
                            _ => panic!("invalid-op"),
                        };
                        Expr::Binary(v[0].to_string(), op, v[2].to_string())
                    }
                },
            )
        })
        .collect()
}

fn part1(exprs: &HashMap<String, Expr>) -> Num {
    eval(exprs, "root")
}

fn part2(exprs: &HashMap<String, Expr>) -> Num {
    // bfs to find the path from root to humn
    let mut queue = VecDeque::from(vec![vec![String::from("root")]]);
    let path = loop {
        let path = queue.pop_front().expect("queue-entry");
        let monke = path.last().expect("path");
        if monke == "humn" {
            break path;
        }
        if let Expr::Binary(l, _, r) = &exprs[monke] {
            queue.push_back(path.iter().cloned().chain([l.clone()]).collect());
            queue.push_back(path.iter().cloned().chain([r.clone()]).collect());
        }
    };

    // evaluate all sub-expressions in the path
    if let Expr::Binary(l, _, r) = &exprs["root"] {
        // start with the value that humn lead to
        let mut value = eval(exprs, if path[1] == *l { r } else { l });
        for (m1, m2) in path.iter().skip(1).zip(path.iter().skip(2)) {
            // invert each of the sub-expressions in the path
            if let Expr::Binary(l, o, r) = &exprs[m1] {
                if *m2 == *l {
                    match o {
                        Op::Add => value -= eval(exprs, r),
                        Op::Sub => value += eval(exprs, r),
                        Op::Mul => value /= eval(exprs, r),
                        Op::Div => value *= eval(exprs, r),
                    }
                } else {
                    match o {
                        Op::Add => value -= eval(exprs, l),
                        Op::Sub => value = eval(exprs, l) - value,
                        Op::Mul => value /= eval(exprs, l),
                        Op::Div => value = eval(exprs, l) / value,
                    }
                }
            } else {
                panic!("invalid-expr");
            }
        }

        // the result will be the value of humn that produces the desired result
        value
    } else {
        panic!("invalid-root");
    }
}

fn eval(exprs: &HashMap<String, Expr>, key: &str) -> Num {
    match &exprs[key] {
        Expr::Lit(x) => *x,
        Expr::Binary(l, o, r) => {
            let l = eval(exprs, l);
            let r = eval(exprs, r);
            match o {
                Op::Add => l + r,
                Op::Sub => l - r,
                Op::Mul => l * r,
                Op::Div => l / r,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
        root: pppw + sjmn\n\
        dbpl: 5\n\
        cczh: sllz + lgvd\n\
        zczc: 2\n\
        ptdq: humn - dvpt\n\
        dvpt: 3\n\
        lfqf: 4\n\
        humn: 5\n\
        ljgn: 2\n\
        sjmn: drzm * dbpl\n\
        sllz: 4\n\
        pppw: cczh / lfqf\n\
        lgvd: ljgn * ptdq\n\
        drzm: hmdt - zczc\n\
        hmdt: 32\n\
    ";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(DATA)), 152);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(DATA)), 301);
    }
}
