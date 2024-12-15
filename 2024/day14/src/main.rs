pub mod p1;
pub mod p2;

pub const SAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

fn main() {
    let input = if cfg!(feature = "p1sample") {
        SAMPLE
    } else {
        include_str!("input.txt")
    };
    println!("Part 1: {}", p1::naive::part1(input));
    p2::naive::part2(input);
}
