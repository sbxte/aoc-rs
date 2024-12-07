use std::fmt::Write as _;
use std::io::Write as _;

// const TABLE_SIZE: usize = 5;
const TABLE_SIZE: usize = 99;
const COLS: usize = TABLE_SIZE;
const ROWS: usize = TABLE_SIZE;

// const COLS: usize = 6;
// const ROWS: usize = 1;

#[inline]
fn parse_number(c: char) -> u64 {
    c.to_digit(10).unwrap() as u64
}

fn main() {
    let input = include_str!("../foo.txt");
    // let input = include_str!("../test.txt");

    // Top left to bottom right
    let mut scores: Vec<u64> = vec![1; COLS * ROWS];

    let mut hstack: Vec<(_, _)> = vec![];
    let mut vstack_arr: Vec<Vec<(_, _)>> = vec![vec![]; COLS];

    for (row, line) in input.lines().enumerate() {
        for (col, character) in line.chars().enumerate() {
            let parsed = parse_number(character);
            let mut vstack = vstack_arr.get_mut(col).unwrap();

            scores[row*ROWS+col] *=
                handle_number(parsed, &mut hstack, scores[row * ROWS + col]) *
                handle_number(parsed, &mut vstack, scores[row * ROWS + col]);
        }
        hstack.clear();
    }

    // Bottom right to top left
    hstack.clear();
    vstack_arr.clear();
    vstack_arr.append(&mut vec![vec![]; COLS]);

    for (row, line) in input.lines().rev().enumerate() {
        let row = ROWS - row;
        for (col, character) in line.chars().rev().enumerate() {
            let col = COLS - col;

            let parsed = parse_number(character);
            let mut vstack = vstack_arr.get_mut(col).unwrap();

            scores[row*ROWS+col] *=
                handle_number(parsed, &mut hstack, scores[row*ROWS+col]) *
                handle_number(parsed, &mut vstack, scores[row*ROWS+col]);
        }
        hstack.clear();
    }


    // Debug print
    let mut string = String::new();
    for row in 0..ROWS {
        for col in 0..COLS {
            let s = scores[row * ROWS + col];
            write!(&mut string, "[{s}]").unwrap();
        }
        write!(&mut string, "\n").unwrap();
    }

    ::std::fs::OpenOptions::new()
        .truncate(true)
        .write(true)
        .open("bar.txt").unwrap()
        .write_all(string.as_bytes()).unwrap();


    // Count all
    scores.sort_unstable();
    scores.reverse();
    scores.truncate(20);
    println!("{:?}", scores);
}

fn handle_number(
    number: u64,
    stack: &mut Vec<(u64, u64)>,
    previous_score: u64,
) -> u64 {
    if stack.len() == 0 {
        stack.push((number, 0));
        0
    } else if stack.last().unwrap().0 < number {
        let (_, score) = stack.pop().unwrap();

        let mut sum = 1;
        while let Some((h, s)) = stack.pop() {
            if h < number {
                sum += s;
            } else {
                break;
            }
        }
        stack.push((number, score + sum));
        previous_score + sum
    } else {
        stack.push((number, 1));
        1
    }
}