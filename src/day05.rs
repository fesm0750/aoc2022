use crate::helpers::read;

// Constants for helping parsing the input file
const N_COLS: usize = 9;
const N_ROWS: usize = 8;
const CHARS_PER_ROW: usize = 36;

pub fn run() {
    let (mut crates, moves) = parse_input();

    println!("Day 05");
    println!("Part 01, top crates {}", solve(&mut crates.clone(), &moves, false));
    println!("Part 02, top crates {}", solve(&mut crates, &moves, true));
}

fn solve(crates: &mut [Vec<u8>], moves: &[Move], can_move_multiple: bool) -> String {
    for m in moves {
        let len = crates[m.from].len();
        let popped = crates[m.from].split_off(len - m.qtd);

        if can_move_multiple {
            crates[m.to].extend(popped.iter().rev());
        } else {
            crates[m.to].extend(popped);
        };
    }

    let output: Vec<_> = crates.iter().map(|v| *v.last().unwrap_or(&b' ')).collect();
    String::from_utf8(output).unwrap()
}

//------------------------------
// Helpers
//------------------------------

/// Assumes the input is well-behaved and ascii
fn parse_input() -> ([Vec<u8>; N_COLS], Vec<Move>) {
    let input = read::file_to_string("day05").unwrap();

    let mut crates: [Vec<u8>; N_COLS] = Default::default();

    // gets the crates starting arrangement
    input[0..N_ROWS * CHARS_PER_ROW].lines().rev().for_each(|s| {
        let s = s.as_bytes();
        let positions = [1usize, 5, 9, 13, 17, 21, 25, 29, 33];
        for (n, &p) in positions.iter().enumerate() {
            if s[p] != b' ' {
                crates[n].push(s[p]);
            }
        }
    });

    // gets the rearrangement procedures
    let moves: Vec<Move> = input[((N_ROWS + 1) * CHARS_PER_ROW + 1)..]
        .lines()
        .map(|s| {
            let mut iter = s[5..].split_whitespace();
            Move {
                qtd: iter.next().unwrap().parse().unwrap(),
                from: iter.nth(1).unwrap().parse::<usize>().unwrap() - 1usize,
                to: iter.nth(1).unwrap().parse::<usize>().unwrap() - 1usize,
            }
        })
        .collect();

    (crates, moves)
}

//------------------------------
// Structs
//------------------------------

struct Move {
    qtd: usize,
    from: usize,
    to: usize,
}
