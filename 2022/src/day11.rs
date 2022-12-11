use regex::Regex;

#[derive(Debug)]
struct Monke {
    items: Vec<u64>,
    operator: Operator,
    operand: Operand,
    test: u64,
    cons: usize,
    ante: usize,
}

#[derive(Debug)]
enum Operator {
    Add,
    Mul,
}

#[derive(Debug)]
enum Operand {
    Old,
    Lit(u64),
}

pub fn solve() {
    let data = parse(&std::fs::read_to_string("data/day11.txt").expect("data-file"));

    // part 1
    println!("part 1: {}", part1(&data));

    // part 2
    println!("part 2: {}", part2(&data));
}

fn parse(data: &str) -> Vec<Monke> {
    data.lines()
        .collect::<Vec<_>>()
        .chunks(7)
        .map(|lines| {
            let items = ITEM_REGEX
                .captures(lines[1])
                .and_then(|c| {
                    c.name("items")
                        .map(|c| c.as_str().split(", ").map(|s| s.parse().expect("item")))
                })
                .expect("items")
                .collect();
            let op = OP_REGEX.captures(lines[2]).expect("op");
            let test = TEST_REGEX
                .captures(lines[3])
                .and_then(|c| c.name("test").and_then(|c| c.as_str().parse().ok()))
                .expect("test");
            let cons = CONS_REGEX
                .captures(lines[4])
                .and_then(|c| c.name("cons").and_then(|c| c.as_str().parse().ok()))
                .expect("cons");
            let ante = ANTE_REGEX
                .captures(lines[5])
                .and_then(|c| c.name("ante").and_then(|c| c.as_str().parse().ok()))
                .expect("ante");
            Monke {
                items,
                operator: match op.name("operator").expect("operator").as_str() {
                    "+" => Operator::Add,
                    "*" => Operator::Mul,
                    _ => panic!("op"),
                },
                operand: match op.name("operand").expect("operand").as_str() {
                    "old" => Operand::Old,
                    lit => Operand::Lit(lit.parse().expect("literal")),
                },
                test,
                cons,
                ante,
            }
        })
        .collect()
}

fn part1(monkes: &[Monke]) -> u64 {
    // track mutable item placement and inspection counts per monke
    let mut items: Vec<_> = monkes.iter().map(|m| m.items.clone()).collect();
    let mut inspects = vec![0; monkes.len()];

    for _round in 0..20 {
        for (i, m) in monkes.iter().enumerate() {
            for mut w in items[i].drain(0..).collect::<Vec<_>>() {
                // apply the monke's operator
                let p = match m.operand {
                    Operand::Old => w,
                    Operand::Lit(x) => x,
                };
                w = match m.operator {
                    Operator::Add => w + p,
                    Operator::Mul => w * p,
                };

                // compute relief
                w /= 3;

                // throw the item to the next monke
                if w % m.test == 0 {
                    items[m.cons].push(w);
                } else {
                    items[m.ante].push(w);
                }

                inspects[i] += 1;
            }
        }
    }

    // take the top-2 monkes and compute the level of monkey business
    inspects.sort_by(|a, b| b.cmp(a));
    inspects[0] * inspects[1]
}

fn part2(monkes: &[Monke]) -> u64 {
    // track mutable item placement and inspection counts per monke
    let mut items: Vec<_> = monkes.iter().map(|m| m.items.clone()).collect();
    let mut inspects = vec![0; monkes.len()];

    // by the chinese remainder theorem, we can compute all moduli of `test` values
    // by first taking the modulus of the product, since they happen to all be coprime
    let rem: u64 = monkes.iter().map(|m| m.test).product();

    for _round in 0..10000 {
        for (i, m) in monkes.iter().enumerate() {
            for mut w in items[i].drain(0..).collect::<Vec<_>>() {
                // apply the monke's operator
                let p = match m.operand {
                    Operand::Old => w,
                    Operand::Lit(x) => x,
                };
                w = match m.operator {
                    Operator::Add => w + p,
                    Operator::Mul => w * p,
                };

                // no relief, but use the crt to reduce the worry level
                w %= rem;

                // throw the item to the next monke
                if w % m.test == 0 {
                    items[m.cons].push(w);
                } else {
                    items[m.ante].push(w);
                }

                inspects[i] += 1;
            }
        }
    }
    inspects.sort_by(|a, b| b.cmp(a));
    inspects[0] * inspects[1]
}

lazy_static! {
    static ref ITEM_REGEX: Regex =
        Regex::new(r#"^  Starting items: (?P<items>(\d+(, )?)+)$"#).unwrap();
    static ref OP_REGEX: Regex =
        Regex::new(r#"^  Operation: new = old (?P<operator>[+*]) (?P<operand>(\d+)|old)$"#)
            .unwrap();
    static ref TEST_REGEX: Regex = Regex::new(r#"^  Test: divisible by (?P<test>(\d+))$"#).unwrap();
    static ref CONS_REGEX: Regex =
        Regex::new(r#"^    If true: throw to monkey (?P<cons>(\d+))$"#).unwrap();
    static ref ANTE_REGEX: Regex =
        Regex::new(r#"^    If false: throw to monkey (?P<ante>(\d+))$"#).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse(DATA)), 10605);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse(DATA)), 2713310158);
    }
}
