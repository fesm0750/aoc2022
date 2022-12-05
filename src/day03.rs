use std::collections::HashSet;

use crate::helpers::read;

pub fn run() {
    let input = read::file_to_string("day03").unwrap();

    println!("Day 03");
    println!("Part 01, priority {}", solve_part01(&input));
    println!("Part 02, priority {}", solve_part02(&input));
}

fn solve_part01(input: &str) -> u32 {
    input
        .lines()
        .map(|s| {
            let half_len = s.len() / 2;
            let set0: HashSet<&u8> = collect_ascii_set(&s[0..half_len]);
            let set1: HashSet<&u8> = collect_ascii_set(&s[half_len..]);
            **(set0.intersection(&set1).next().unwrap())
        })
        .map(calculate_priority)
        .sum()
}

/// As an Advent of Code solution, assumes the input data is well behaved according to rules of the puzzle, so it
/// process the iterator as batches of 3 without checking.
fn solve_part02(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut priorities = 0;

    while let Some(line0) = lines.next() {
        let set0: HashSet<_> = collect_ascii_set(line0);
        let set1: HashSet<_> = collect_ascii_set(lines.next().unwrap());
        let set2: HashSet<_> = collect_ascii_set(lines.next().unwrap());

        let intersect: HashSet<&u8> = set0.intersection(&set1).copied().collect();

        priorities += calculate_priority(**intersect.intersection(&set2).next().unwrap());
    }
    priorities
}

//------------------------------
// Helpers
//------------------------------

fn collect_ascii_set(str: &str) -> HashSet<&u8> {
    str.as_bytes().iter().collect()
}

fn calculate_priority(v: u8) -> u32 {
    (match v {
        0x41..=0x5a => v - 0x26, // A to Z results in priorities 27 to 52
        0x61..=0x7a => v - 0x60, // a to z results in priorities 1 to 26
        _ => 0,
    }) as u32
}

//------------------------------
// Tests
//------------------------------

#[cfg(test)]
mod tests {
    use super::calculate_priority;

    #[test]
    fn teste_calculate_priority() {
        assert_eq!(calculate_priority(b'p'), 16);
        assert_eq!(calculate_priority(b'P'), 42);
    }
}
