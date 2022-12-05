use std::str::FromStr;

use crate::helpers::read;
use {Outcome::*, Shape::*};

pub fn run() {
    let input = read::file_to_string("day02").unwrap();
    let part01 = parse_pt01(&input);
    let part02 = parse_pt02(&input);

    let calculate_score = |rounds: &[Round]| -> u32 { rounds.iter().map(|r| r.play() as u32).sum::<u32>() };

    println!("Day 02");
    println!("Part 01, total score {}", calculate_score(&part01));
    println!("Part 02, total score {}", calculate_score(&part02));
}

/// reads the input file for day02 and returns a vector of the game rounds according to *part 01* rules.
///
/// Assumes the file exists and that its entries are valid, otherwise panics.
fn parse_pt01(input: &str) -> Vec<Round> {
    input
        .lines()
        .map(|s| {
            let to_shape = |s: &str| -> Shape {
                match s {
                    "A" | "X" => Rock,
                    "B" | "Y" => Paper,
                    "C" | "Z" => Scisors,
                    _ => panic!(),
                }
            };

            let opponent = to_shape(&s[0..1]);
            let player = to_shape(&s[2..3]);

            Round(opponent, player)
        })
        .collect()
}

/// reads the input file for day02 and returns a vector of the game rounds according to *part 02* rules.
///
/// Panics if the at any point the `input` string cannot be converted to a `Shape` or `Outcome` enum.
fn parse_pt02(input: &str) -> Vec<Round> {
    input.lines().flat_map(Strategy::from_str).map(Round::from).collect()
}

//--------------------------------------------------------------------
// Structs and Enums
//--------------------------------------------------------------------

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scisors,
}

enum Outcome {
    Draw,
    Lose,
    Win,
}

/// First shape is the opponents, second is the players
struct Round(Shape, Shape);

struct Strategy(Shape, Outcome);

//--------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------

//------------------------------
// Shape
//------------------------------
impl Shape {
    fn get_winner(&self) -> Shape {
        match &self {
            Rock => Paper,
            Paper => Scisors,
            Scisors => Rock,
        }
    }

    fn get_loser(&self) -> Shape {
        match &self {
            Rock => Scisors,
            Paper => Rock,
            Scisors => Paper,
        }
    }
}

impl FromStr for Shape {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Rock),
            "B" => Ok(Paper),
            "C" => Ok(Scisors),
            _ => Err(format!(
                "Error parsing `Shape`, found the `{}` value. Acceptable values are: A, B or C",
                s
            )),
        }
    }
}

//------------------------------
// Outcome
//------------------------------

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Lose),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(format!(
                "Error parsing `Outcome`, found the `{}` value. Acceptable values are: X, Y or Z",
                s
            )),
        }
    }
}

//------------------------------
// Round
//------------------------------

impl Round {
    /// simulates the game and returns the score
    /// The score is a value depending on the Shape played plus another value depending on the result:
    /// rock = 1, paper = 2, scisors = 3
    /// lose = 0, draw = 3, win = 6
    fn play(&self) -> u8 {
        match self {
            Round(Rock, Rock) => 1 + 3,
            Round(Rock, Paper) => 2 + 6,
            Round(Rock, Scisors) => 3,
            Round(Paper, Rock) => 1,
            Round(Paper, Paper) => 2 + 3,
            Round(Paper, Scisors) => 3 + 6,
            Round(Scisors, Rock) => 1 + 6,
            Round(Scisors, Paper) => 2,
            Round(Scisors, Scisors) => 3 + 3,
        }
    }
}

impl From<Strategy> for Round {
    fn from(value: Strategy) -> Self {
        match value {
            Strategy(shape, Lose) => Round(shape, shape.get_loser()),
            Strategy(shape, Draw) => Round(shape, shape),
            Strategy(shape, Win) => Round(shape, shape.get_winner()),
        }
    }
}

//------------------------------
// Strategy
//------------------------------

impl FromStr for Strategy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let shape = Shape::from_str(&s[0..1])?;
        let outcome = Outcome::from_str(&s[2..3])?;
        Ok(Strategy(shape, outcome))
    }
}
