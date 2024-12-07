pub mod p1;
pub mod p2;

use p1::part1;
use p2::part2;

pub const SAMPLE: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

fn main() {
    let input = include_str!("input.txt");
    println!("Part 2 sample: {}", part2(SAMPLE));
    println!("Part 2 real: {}", part2(input));
    println!("Part 1 sample: {}", part1(SAMPLE));
    println!("Part 1 real: {}", part1(input));
}
