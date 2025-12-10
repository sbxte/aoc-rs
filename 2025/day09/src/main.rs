use std::time::Instant;

mod p1;
mod p2;

fn main() {
    let input = include_str!("input.txt");
    let now = Instant::now();
    let sol2 = p2::part2(input);
    let elapsed = now.elapsed();
    
    println!("Part 1: {}", p1::part1(input));
    println!("Part 2: {} in {} μs", sol2, elapsed.as_micros());

}
