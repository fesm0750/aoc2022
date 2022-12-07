//! Uses a brute-force approach
//!
//! Alternative implementations would be a Hashset or an array of frequency to track duplicates.
use crate::helpers::read;

pub fn run() {
    let input = read::file_to_string("day06").unwrap();
    let input = input.as_bytes();

    println!("Day 06");
    println!(
        "Part 01, Total characters processed for start-of-packet marker {}",
        find_marker_end(input, 4).unwrap()
    );
    println!(
        "Part 02, Total characters processed for start-of-message marker {}",
        find_marker_end(input, 14).unwrap()
    );
}

fn find_marker_end(input: &[u8], window_size: usize) -> Option<usize> {
    input
        .windows(window_size)
        .enumerate()
        .find(|(_, w)| w.iter().enumerate().all(|(e, c)| !w[e + 1..window_size].contains(c)))
        .map(|(e, _)| e + window_size)
}
