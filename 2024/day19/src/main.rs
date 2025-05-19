#![feature(iter_intersperse)]
#![allow(dead_code)]

mod p1;
mod p2;

pub const SAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

fn main() {
    let input = include_str!("input.txt");
    println!("Day 19 part 1: {}", p1::dp::part1(input));
    println!("Day 19 part 2: {}", p2::part2(input));
}
