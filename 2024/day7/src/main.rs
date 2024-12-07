pub const SAMPLE: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

pub mod p1;
pub mod p2;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 2: {}", p2::optim::part2(input));
    println!("Part 1: {}", p1::optim::part1(input));
}
