pub mod p1;
pub mod p2;

pub const SAMPLE: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

fn main() {
    let input = include_str!("input.txt");
    println!("Part 2: {}", p2::naive::part2(input));
    println!("Part 1: {}", p1::naive::part1(input));
}
