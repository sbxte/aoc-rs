pub mod p1;
pub mod p2;

fn main() {
    let input = include_str!("input.txt");
    println!("Day 16 part 1: {}", p1::part1(input));
}
