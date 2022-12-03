#[macro_use]
extern crate lazy_static;

use std::env;

mod day01;
mod day02;
mod day03;

const SOLUTIONS: &[&dyn Fn()] = &[&day01::solve, &day02::solve, &day03::solve];

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => dispatch(args[1].parse().unwrap()),
        _ => (1..=SOLUTIONS.len()).map(dispatch).collect(),
    }
}

fn dispatch(day: usize) {
    println!("day {}", day);
    match SOLUTIONS.get(day - 1) {
        Some(f) => f(),
        None => println!("not implemented yet"),
    }
    println!()
}
