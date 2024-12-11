pub mod p1;
pub mod p2;

pub const SAMPLE: &str = "125 17";

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", p1::naive::part1(input));
    println!("Part 2: {}", p2::naive::part2(input, 75));
}
