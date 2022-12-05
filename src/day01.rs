use crate::helpers::read;

pub fn run() {
    let input = read::file_to_string("day01").unwrap();
    let outer_iter = input.split("\n\n");
    let mut calories: Vec<u32> = outer_iter
        .map(|s| {
            let inner_iter = s.lines();
            inner_iter.flat_map(str::parse::<u32>).sum::<u32>()
        })
        .collect();

    // Part 1
    println!("Day 01");
    println!("Max calories: {}", calories.iter().max().unwrap());

    // Part 2
    calories.sort_unstable();
    let len = calories.len();
    let top3 = calories[len - 3..].iter().sum::<u32>();

    println!("Sum of top 3: {}", top3);
}
