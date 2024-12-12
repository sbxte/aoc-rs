pub mod p1;
pub mod p2;

pub const SAMPLE: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", p1::optim::part1(input));
    println!("Part 2: {}", p2::optim::part2(input));
}
