// Not the prettiest solution, but works!
//
// For part 2, the function `find_best_scenic_score` uses 4 loops, to process the grid once for each direction. Loops
// usually start at the second row or column because the first one related to that given direction has values for view
// distance as zero.

use crate::helpers::{grid::Grid, read};

#[derive(Copy, Clone)]
struct Tree {
    height: u8,
    is_visible_from_outside: bool,
    north_view: u8,
    south_view: u8,
    east_view: u8,
    west_view: u8,
}

use Direction::*;
enum Direction {
    North,
    South,
    West,
    East,
}

pub fn run() {
    let input = read::file_to_string("day08").unwrap();
    let mut grid = parse_input(&input);

    println!("Day 08");
    println!("Part 01, Visible trees {}", count_visible_from_outside(&mut grid));
    println!("Part 02, Best scenic score {}", find_best_scenic_score(&mut grid));
}

fn parse_input(input: &str) -> Grid<Tree> {
    let len_x = input.find('\n').unwrap();
    let flat: Vec<Tree> = input
        .lines()
        .flat_map(str::chars)
        .map(|c| Tree::new(c.to_digit(10).unwrap() as u8))
        .collect();

    let mut grid = Grid::from_vec(len_x, flat.len() / len_x, flat);

    // set visibility of trees on the edge of the grid
    // North Edge
    grid.row_mut(0)
        .iter_mut()
        .for_each(|t| t.is_visible_from_outside = true);
    // South Edge
    grid.row_mut(grid.len_y - 1)
        .iter_mut()
        .for_each(|t| t.is_visible_from_outside = true);
    // West Edge
    grid.iter_col_mut(0).for_each(|t| t.is_visible_from_outside = true);
    // East Edge
    grid.iter_col_mut(grid.len_x - 1)
        .for_each(|t| t.is_visible_from_outside = true);

    grid
}

/// A tree may or may not be visible
/// All border trees are visible
/// An inner tree is visible if there are no bigger trees in front of each from at least one direction
/// Iterate over the grid by line and column
/// Iterate from one direction first and save the size of the biggest tree seen, then iterate in the reverse direction
/// stop iteration if the tallest tree is found
fn count_visible_from_outside(grid: &mut Grid<Tree>) -> usize {
    let iterate = [
        (North, grid.len_y),
        (South, grid.len_y),
        (West, grid.len_x),
        (East, grid.len_x),
    ];

    const TALLEST: u8 = 9;
    for (direction, len) in iterate {
        for i in 0..len {
            let iterator: Box<dyn Iterator<Item = &mut Tree>> = match direction {
                North => Box::new(grid.row_mut(i).iter_mut()),       // from north
                South => Box::new(grid.row_mut(i).iter_mut().rev()), // from south
                West => Box::new(grid.iter_col_mut(i)),              // from west
                East => Box::new(grid.iter_col_mut(i).rev()),        // from east
            };

            let mut max = 0;

            for tree in iterator {
                if tree.height > max {
                    max = tree.height;
                    tree.is_visible_from_outside = true;

                    if tree.height == TALLEST {
                        break;
                    }
                }
            }
        }
    }
    grid.iter().filter(|tree| tree.is_visible_from_outside).count()
}

fn find_best_scenic_score(forest: &mut Grid<Tree>) -> u64 {
    // scan from north, looking to the northern border
    // for each element of a line starting from the second line from the northern border, find the view_distance looking
    // north
    for row in 1..forest.len_y {
        for x in 0..forest.len_x {
            let view_distance;
            {
                let curr_tree = forest.get(x, row);

                view_distance = forest
                    .iter_col(x)
                    .take(row)
                    .rev()
                    .enumerate()
                    .find(|(_, tree)| curr_tree.height <= tree.height)
                    .map(|(i, _)| i + 1)
                    .unwrap_or(row) as u8;
            }
            forest.get_mut(x, row).north_view = view_distance;
        }
    }

    // scan from south
    for row in (0..forest.len_y - 1).rev() {
        for x in 0..forest.len_x {
            let view_distance;
            {
                let curr_tree = forest.get(x, row);

                view_distance = forest
                    .iter_col(x)
                    .skip(row + 1)
                    .enumerate()
                    .find(|(_, tree)| curr_tree.height <= tree.height)
                    .map(|(i, _)| i + 1)
                    .unwrap_or(forest.len_y - row - 1) as u8;
            }
            forest.get_mut(x, row).south_view = view_distance;
        }
    }

    // scan from western boarder
    // for each element of a column, starting from the second column from the western border, find the view distance
    // looking west
    // If iterating over the columns, the view is over the row
    for col in 1..forest.len_x {
        for y in 0..forest.len_y {
            let view_distance;
            {
                let curr_tree = forest.get(col, y);

                view_distance = forest
                    .row(y)
                    .iter()
                    .take(col)
                    .rev()
                    .enumerate()
                    .find(|(_, tree)| curr_tree.height <= tree.height)
                    .map(|(i, _)| i + 1)
                    .unwrap_or(col) as u8;
            }
            forest.get_mut(col, y).west_view = view_distance;
        }
    }

    // scan from east
    for col in (0..forest.len_x - 1).rev() {
        for y in 0..forest.len_y {
            let view_distance;
            {
                let curr_tree = forest.get(col, y);

                view_distance = forest
                    .row(y)
                    .iter()
                    .skip(col + 1)
                    .enumerate()
                    .find(|(_, tree)| curr_tree.height <= tree.height)
                    .map(|(i, _)| i + 1)
                    .unwrap_or(forest.len_x - col - 1) as u8;
            }
            forest.get_mut(col, y).east_view = view_distance;
        }
    }

    forest.iter().map(Tree::calculate_scenic_score).max().unwrap()
}

#[allow(dead_code)]
fn count_visible_from_outside_old(grid: &mut Grid<Tree>) -> usize {
    // iterates over rows
    for y in 0..grid.len_y {
        let mut max = 0;

        // on normal direction
        let row = grid.row_mut(y).iter_mut();
        for tree in row {
            if tree.height > max {
                max = tree.height;
                tree.is_visible_from_outside = true;

                if tree.height == 9 {
                    break;
                }
            }
        }

        let tallest = max;
        max = 0;
        // on reverse
        let row = grid.row_mut(y).iter_mut().rev();
        for tree in row {
            if tree.height > max {
                max = tree.height;
                tree.is_visible_from_outside = true;

                if tree.height == tallest {
                    break;
                }
            }
        }
    }

    // iterates over columns
    for x in 0..grid.len_x {
        let mut max = 0;

        // iterates on normal direction
        let col = grid.iter_col_mut(x);
        for tree in col {
            if tree.height > max {
                max = tree.height;
                tree.is_visible_from_outside = true;

                if tree.height == 9 {
                    break;
                }
            }
        }

        let tallest = max;
        max = 0;
        // iterates on reverse
        let col_rev = grid.iter_col_mut(x).rev();
        for tree in col_rev {
            if tree.height > max {
                max = tree.height;
                tree.is_visible_from_outside = true;

                if tree.height == tallest {
                    break;
                }
            }
        }
    }

    grid.iter().filter(|tree| tree.is_visible_from_outside).count()
}
//------------------------------
// Implementations
//------------------------------

impl Tree {
    fn new(height: u8) -> Self {
        Self {
            height,
            is_visible_from_outside: false,
            north_view: 0,
            south_view: 0,
            east_view: 0,
            west_view: 0,
        }
    }

    fn calculate_scenic_score(&self) -> u64 {
        self.north_view as u64 * self.south_view as u64 * self.east_view as u64 * self.west_view as u64
    }
}

//------------------------------
// Tests
//------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT: String = "30373
25512
65332
33549
35390"
            .to_string();
        static ref GRID: Grid<Tree> = parse_input(&INPUT);
    }

    #[test]
    fn teste_count_visible() {
        let mut grid: Grid<Tree> = GRID.clone();
        assert_eq!(count_visible_from_outside(&mut grid), 21);
    }

    #[test]
    fn teste_find_best_scenic_score() {
        let mut grid: Grid<Tree> = GRID.clone();
        let score = find_best_scenic_score(&mut grid);
        assert_eq!(score, 8);
    }

    #[test]
    fn teste_find_best_scenic_score2() {
        // best one is the 3 at the fourth line and fourth column
        let input = "00000
11111
22222
31234
33333";
        let mut grid: Grid<Tree> = parse_input(input);
        let score = find_best_scenic_score(&mut grid);
        assert_eq!(score, 9);
    }
}
