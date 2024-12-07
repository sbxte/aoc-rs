pub const SAMPLE: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

pub const CUSTOM_SUCC: &str = "1 2 3 4 5
1 5 6 7 8
1 8 7 6 5
4 8 7 6 5
4 3 2 1 5
4 3 2 1 6
1 2 3 4 9
5 1 4 3 2
5 1 2 3 4
1 2 3 4 1";

pub const CUSTOM_FAIL: &str = "5 4 1 2 3
5 6 7 1 2 3
5 1 2 3 7
5 1 3 5 1 3 5
1 3 5 3 1
1 3 5 3 5 1
7 5 3 1 7 5 3 1
7 5 3 1 5 1 3 5 7
1 5 6 7 11
1 5 6 10 11 12 16";

pub fn part1(input: &str) -> u32 {
    let mut sum = 0;
    'line: for line in input.lines() {
        let levels: Vec<_> = line.split(' ').map(|x| x.parse::<i32>().unwrap()).collect();

        let diff = levels[1] - levels[0];
        for window in levels.windows(2) {
            let d = window[1] - window[0];
            if d.signum() != diff.signum() || !(1..=3).contains(&d.abs()) {
                continue 'line;
            }
        }
        sum += 1;
    }
    sum
}

pub fn part2_check(levels: &[i32]) -> bool {
    let r = 1..=3;
    let mut inc = 0;
    let mut dec = 0;
    for w in levels.windows(2) {
        if !r.contains(&w[1].abs_diff(w[0])) {
            return false;
        }

        // Clippy made me do this
        use std::cmp::Ordering::*;
        match w[1].cmp(&w[0]) {
            Greater => inc += 1,
            Less => dec += 1,
            _ => {}
        }
    }
    if inc > 0 && dec > 0 {
        return false;
    }

    true
}

// I am ashamed to admit I have used brute force here...
pub fn part2_safe(levels: Vec<i32>) -> bool {
    if part2_check(&levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut l = levels.clone();
        l.remove(i);
        if part2_check(&l) {
            return true;
        }
    }

    false
}

pub fn part2(input: &str) -> u32 {
    let mut levels = vec![];
    let mut sum = 0;
    for line in input.lines() {
        levels.clear();
        levels.extend(line.split(' ').map(|x| x.parse::<i32>().unwrap()));
        if part2_safe(levels.clone()) {
            sum += 1;
        }
    }
    sum
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod d2 {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(part1(SAMPLE), 2);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(SAMPLE), 4);
    }

    #[test]
    fn p2_cs() {
        assert_eq!(part2(CUSTOM_SUCC), 10);
    }

    #[test]
    fn p2_cf() {
        assert_eq!(part2(CUSTOM_FAIL), 0);
    }
}
