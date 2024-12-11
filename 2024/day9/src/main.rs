#![feature(let_chains, random)]

pub mod p1;
pub mod p2;

pub const SAMPLE: &str = "2333133121414131402";

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", p1::optim::part1(input));
    println!("Part 2: {}", p2::naive::part2(input));
}
