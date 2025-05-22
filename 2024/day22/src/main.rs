#![feature(iter_map_windows)]

mod p1;
mod p2;

fn main() {
    let input = include_str!("input.txt");
    println!("Day 22 part 1: {}", p1::part1(input));
    println!("Day 22 part 2: {}", p2::part2(input));
}
