pub mod p1;
pub mod p2;

pub const SAMPLE: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

fn main() {
    let input = include_str!("input.txt");
    println!("Part 2: {}", p2::optim::part2(input));
    println!("Part 1: {}", p1::optim::part1(input));
}
