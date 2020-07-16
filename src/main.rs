use std::env;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => dispatch(args[1].parse().unwrap()),
        _ => (1..=25).map(|i| dispatch(i)).collect()
    }
}

fn dispatch(day: i32) {
    println!("day {}", day);
    match day {
        1 => day01::solve(),
        2 => day02::solve(),
        3 => day03::solve(),
        4 => day04::solve(),
        5 => day05::solve(),
        6 => day06::solve(),
        _ => ()
    }
    println!()
}
