mod p1;
mod p2;

fn main() {
    let input = include_str!("input.txt");
    println!("Day 24 part 1: {}", p1::part1(input));
    p2::part2(input);
}
