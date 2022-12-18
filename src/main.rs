//! Just run the binary passing the day as argument. For exemple `cargo run 1`, runs the solution for day 01 puzzle.
//!
//! The puzzles inputs need to be added to the `inputs` folder.

use aoc2022::*;
use std::env;

fn main() {
    let input = env::args().nth(1);
    if input.is_none() {
        println!("No input argument.");
        return;
    }

    match input.unwrap().parse().unwrap() {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        4 => day04::run(),
        5 => day05::run(),
        6 => day06::run(),
        7 => day07::run(),
        8 => day08::run(),
        9 => day09::run(),
        _ => println!("Invalid input argument."),
    }
}
