pub const SAMPLE: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

pub fn part1(input: &str) -> u32 {
    let mut left = Vec::with_capacity(input.lines().count());
    let mut right = Vec::with_capacity(left.len());

    for line in input.lines() {
        let (l, r) = line.split_once(' ').unwrap();
        let (l, r) = (
            l.trim().parse::<u32>().unwrap(),
            r.trim().parse::<u32>().unwrap(),
        );
        left.push(l);
        right.push(r);
    }
    left.sort();
    right.sort();

    let mut sum = 0;
    for i in 0..left.len() {
        sum += left[i].abs_diff(right[i]);
    }
    sum
}

pub fn part2(input: &str) -> u32 {
    let mut left = Vec::with_capacity(input.lines().count());
    let mut right = Vec::with_capacity(left.len());

    for line in input.lines() {
        let (l, r) = line.split_once(' ').unwrap();
        let (l, r) = (
            l.trim().parse::<u32>().unwrap(),
            r.trim().parse::<u32>().unwrap(),
        );
        left.push(l);
        right.push(r);
    }
    left.sort();
    right.sort();

    let mut sum = 0;

    // Clippy does not account for the `continue` statement inside the loop when suggesting a fix;
    #[allow(clippy::needless_range_loop)]
    for i in 0..left.len() {
        let target = left[i];
        let index = right.binary_search(&target);
        if index.is_err() {
            continue;
        }
        let index = index.unwrap();

        let mut l_idx = index;
        let mut r_idx = index;
        while l_idx > 0 && right[l_idx - 1] == target {
            l_idx -= 1;
        }
        while r_idx + 1 < right.len() && right[r_idx + 1] == target {
            r_idx += 1;
        }
        let count = r_idx - l_idx + 1;
        sum += count as u32 * target;
    }
    sum
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod d1 {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(part1(SAMPLE), 11);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(SAMPLE), 31);
    }
}
