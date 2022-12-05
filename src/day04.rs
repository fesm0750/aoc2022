use std::str::FromStr;

use crate::helpers::read;

pub fn run() {
    let input: Vec<Assigments> = read::file_lines_to_vec("day04").unwrap();

    println!("Day 04");
    println!(
        "Part 01, total of fully overlaped assigments {}",
        input.iter().copied().filter(Assigments::is_fully_contained).count()
    );
    println!(
        "Part 02, total of overlaped assigments {}",
        input.iter().copied().filter(Assigments::is_overlaped).count()
    );
}

#[derive(Copy, Clone)]
struct Assigments {
    a0: u8,
    a1: u8,
    b0: u8,
    b1: u8,
}

impl Assigments {
    fn is_fully_contained(&self) -> bool {
        // let range0 = self.a0..self.a1 + 1;
        // let range1 = self.b0..self.b1 + 1;
        // (range0.contains(&self.b0) && range0.contains(&self.b1)) || (range1.contains(&self.a0) &&
        // range1.contains(&self.a1))
        (self.a0 <= self.b0 && self.b1 <= self.a1) || (self.b0 <= self.a0 && self.a1 <= self.b1)
    }

    fn is_overlaped(&self) -> bool {
        !(self.b1 < self.a0 || self.a1 < self.b0)
    }
}

impl FromStr for Assigments {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(&['-', ',']).flat_map(u8::from_str).take(4);
        Ok(Assigments {
            a0: iter.next().unwrap(),
            a1: iter.next().unwrap(),
            b0: iter.next().unwrap(),
            b1: iter.next().unwrap(),
        })
    }
}
