use std::collections::HashMap;
use std::io::{prelude::*};
use regex::Regex;

lazy_static! {
    static ref REGEX: Regex = Regex::new("\
        (((?P<assign_lit>[0-9]+)|(?P<assign_var>[a-z]+)) -> (?P<assign_name>[a-z]+))|\
        ((?P<unary>NOT) ((?P<unary_lit>[0-9]+)|(?P<unary_var>[a-z]+)) -> (?P<unary_name>[a-z]+))|\
        (((?P<binary_lit1>[0-9]+)|(?P<binary_var1>[a-z]+)) (?P<binary>AND|OR|LSHIFT|RSHIFT) ((?P<binary_lit2>[0-9]+)|(?P<binary_var2>[a-z]+)) -> (?P<binary_name>[a-z]+))|\
    ").unwrap();
}

#[derive(Clone, Copy, Debug)]
enum Value<'a> {
    Lit(u16),
    Var(&'a str),
}

#[derive(Clone, Copy, Debug)]
enum Unary {
    Not,
}

#[derive(Clone, Copy, Debug)]
enum Binary {
    And,
    Or,
    LShift,
    RShift,
}

#[derive(Clone, Copy, Debug)]
enum Expr<'a> {
    Assign(Value<'a>),
    Unary(Unary, Value<'a>),
    Binary(Binary, Value<'a>, Value<'a>),
}

#[derive(Clone, Copy, Debug)]
struct Rule<'a> {
    name: &'a str,
    expr: Expr<'a>,
}

pub fn solve() {
    let file = std::fs::File::open("data/day07.txt").unwrap();
    let lines: Vec<String> = std::io::BufReader::new(file)
        .lines()
        .map(|l| l.unwrap())
        .collect();
    let input: HashMap<&str, Expr> = lines
        .iter()
        .map(|l| parse(l.as_str()))
        .map(|r| (r.name, r.expr))
        .collect();

    // part 1
    let mut output = input.clone();
    let output = reduce(&mut output);

    let result = match output["a"] {
        Expr::Assign(Value::Lit(y)) => y,
        _ => panic!()
    };

    println!("part 1: {}", result);

    // part 2
    let mut output = input.clone();
    output.insert("b", Expr::Assign(Value::Lit(result)));
    let output = reduce(&mut output);

    let result = match output["a"] {
        Expr::Assign(Value::Lit(y)) => y,
        _ => panic!()
    };

    println!("part 2: {}", result);
}

fn parse(line: &str) -> Rule {
    let caps = REGEX.captures(line).unwrap();
    if caps.name("assign_name") != None {
        let param = parse_value(&caps, "assign_lit", "assign_var");
        Rule {
            name: caps.name("assign_name").unwrap().as_str(),
            expr: Expr::Assign(param)
        }
    } else if caps.name("unary_name") != None {
        let param = parse_value(&caps, "unary_lit", "unary_var");
        Rule {
            name: caps.name("unary_name").unwrap().as_str(),
            expr: Expr::Unary(
                match caps.name("unary").unwrap().as_str() {
                    "NOT" => Unary::Not,
                    _ => panic!()
                },
                param
            )
        }
    } else if caps.name("binary_name") != None {
        let param1 = parse_value(&caps, "binary_lit1", "binary_var1");
        let param2 = parse_value(&caps, "binary_lit2", "binary_var2");
        Rule {
            name: caps.name("binary_name").unwrap().as_str(),
            expr: Expr::Binary(
                match caps.name("binary").unwrap().as_str() {
                    "AND" => Binary::And,
                    "OR" => Binary::Or,
                    "LSHIFT" => Binary::LShift,
                    "RSHIFT" => Binary::RShift,
                    _ => panic!()
                },
                param1,
                param2
            )
        }
    } else {
        panic!();
    }
}

fn parse_value<'a>(caps: &regex::Captures<'a>, lit: &str, var: &str) -> Value<'a> {
    if caps.name(lit) != None {
        Value::Lit(caps.name(lit).unwrap().as_str().parse().unwrap())
    } else {
        Value::Var(caps.name(var).unwrap().as_str())
    }
}

fn reduce<'a>(input: &'a mut HashMap<&'a str, Expr<'a>>) -> &HashMap<&'a str, Expr<'a>> {
    loop {
        let mut done = true;
        let orig = input.clone();
        for v in input.values_mut() {
            match *v {
                Expr::Unary(op, Value::Lit(x)) => {
                    *v = apply_unary(op, x);
                },
                Expr::Binary(op, Value::Lit(x), Value::Lit(y)) => {
                    *v = apply_binary(op, x, y);
                },
                Expr::Assign(Value::Var(name)) => {
                    done = false;
                    if let Expr::Assign(Value::Lit(lit)) = orig[name] {
                        *v = Expr::Assign(Value::Lit(lit));
                    }
                },
                Expr::Unary(op, Value::Var(name)) => {
                    done = false;
                    if let Expr::Assign(Value::Lit(lit)) = orig[name] {
                        *v = Expr::Unary(op, Value::Lit(lit));
                    }
                },
                Expr::Binary(op, Value::Var(name), y) => {
                    done = false;
                    if let Expr::Assign(Value::Lit(lit)) = orig[name] {
                        *v = Expr::Binary(op, Value::Lit(lit), y);
                    }
                },
                Expr::Binary(op, x, Value::Var(name)) => {
                    done = false;
                    if let Expr::Assign(Value::Lit(lit)) = orig[name] {
                        *v = Expr::Binary(op, x, Value::Lit(lit));
                    }
                },
                _ => ()
            }
        }

        if done { break; }
    }

    input
}

fn apply_unary<'a>(op: Unary, x: u16) -> Expr<'a> {
    Expr::Assign(Value::Lit(match op {
        Unary::Not => !x,
    }))
}

fn apply_binary<'a>(op: Binary, x: u16, y: u16) -> Expr<'a> {
    Expr::Assign(Value::Lit(match op {
        Binary::And => x & y,
        Binary::Or => x | y,
        Binary::LShift => x << y,
        Binary::RShift => x >> y,
    }))
}
