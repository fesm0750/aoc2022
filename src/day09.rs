use crate::helpers::{base2d::Base2d, read};
use std::{cmp::Ordering, str::FromStr};

type Point = Base2d<i64>;

pub fn run() {
    let input: Vec<Displacement> = read::file_lines_to_vec("day09").unwrap();

    println!("Day 09");
    println!(
        "Part 01, Counting of unique tail positions for 2 knot rope {}",
        simulate_rope(&input, 2)
    );
    println!(
        "Part 02, Counting of unique tail positions for 10 knot rope {}",
        simulate_rope(&input, 10)
    );
}

fn simulate_rope(input: &[Displacement], rope_size: usize) -> usize {
    let origin = Point::from_tuple((0, 0));
    let mut rope: Vec<Point> = vec![origin; rope_size];
    let mut tail_positions: Vec<Option<Point>> = Vec::new();
    tail_positions.push(Some(origin));

    // Simulation
    for motion in input {
        for &displacement in motion.moves() {
            let mut rope_knots = rope.iter_mut();
            let head = rope_knots.next().unwrap();
            *head += displacement;

            let mut curr_head = *head;
            for tail in rope_knots {
                let distance = curr_head - *tail;
                let tail_movement = match distance {
                    // 2 Directly Up, Down, Left, Right
                    Point { x: 0, y: 2 } => (0, 1),
                    Point { x: 0, y: -2 } => (0, -1),
                    Point { x: -2, y: 0 } => (-1, 0),
                    Point { x: 2, y: 0 } => (1, 0),
                    // 2 to the sides
                    Point { x: -2, y: 1.. } => (-1, 1),
                    Point { x: -2, y: _ } => (-1, -1),
                    Point { x: 2, y: 1.. } => (1, 1),
                    Point { x: 2, y: _ } => (1, -1),
                    // 2 up or down
                    Point { x: 1.., y: -2 } => (1, -1),
                    Point { x: _, y: -2 } => (-1, -1),
                    Point { x: 1.., y: 2 } => (1, 1),
                    Point { x: _, y: 2 } => (-1, 1),
                    // Don't move, if implemented correct, there will be no case where a coordinate is 3 or more away
                    _ => (0, 0),
                };

                if tail_movement != (0, 0) {
                    let tail_move = Point::from_tuple(tail_movement);
                    *tail += tail_move;
                }

                curr_head = *tail;
            }

            tail_positions.push(Some(curr_head)); // when the loop finishes, `curr_head` holds the tail
        }
    }

    // Sort the `Vec` of tail positions built by the simulation
    tail_positions.sort_unstable_by(|a, b| {
        if let (Some(a), Some(b)) = (a, b) {
            if a.x < b.x || a.x == b.x && a.y < b.y {
                Ordering::Less
            } else if a.x == b.x && a.y == b.y {
                Ordering::Equal
            } else {
                Ordering::Greater
            }
        } else {
            // should not be a `None` inside the `Vec` right now
            panic!()
        }
    });

    // Maps non unique positions to None
    let mut iter = tail_positions.iter_mut();
    let mut previous = iter.next().unwrap();
    iter.for_each(|element| {
        if element == previous {
            *element = None;
        } else {
            previous = element;
        }
    });

    // Filters and returs the total of unique positions
    tail_positions.iter().filter_map(|&e| e).count()
}

use Direction::*;
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// A struct to represent a displacement
/// val is the total displacement
/// dir is a 2d data representing the direction of the displacement
#[derive(Debug)]
struct Displacement {
    val: u8,
    dir: [Point; 1],
}

//------------------------------
// Implementations
//------------------------------

impl Displacement {
    fn new(dir: Direction, val: u8) -> Self {
        let dir = match dir {
            Down => (0, -1),
            Up => (0, 1),
            Left => (-1, 0),
            Right => (1, 0),
        };
        let mov = [Point::from_tuple(dir); 1];
        Displacement { val, dir: mov }
    }

    fn moves(&self) -> impl Iterator<Item = &Point> {
        self.dir.iter().cycle().take(self.val.into())
    }
}

impl FromStr for Displacement {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace();
        let direction = iter.next().ok_or("Could not parse Direction.")?;
        let val = iter
            .next()
            .ok_or("No value.")?
            .parse::<u8>()
            .or(Err("Could not parse value."))?;

        let dir = match direction {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => Err("Could not parse value.")?,
        };

        Ok(Displacement::new(dir, val))
    }
}
