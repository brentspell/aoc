#[macro_use]
extern crate lazy_static;

use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

const SOLUTIONS: &[&dyn Fn()] = &[
    &day01::solve,
    &day02::solve,
    &day03::solve,
    &day04::solve,
    &day05::solve,
    &day06::solve,
    &day07::solve,
    &day08::solve,
    &day09::solve,
    &day10::solve,
    &day11::solve,
    &day12::solve,
    &day13::solve,
    &day14::solve,
    &day15::solve,
    &day16::solve,
    &day17::solve,
];

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
