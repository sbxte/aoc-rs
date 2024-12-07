use std::cmp::Ordering;
use std::collections::HashMap;

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

pub fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut lines = input.lines();

    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
    #[allow(clippy::while_let_on_iterator)]
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let (left, right) = line.split_once('|').unwrap();
        let (left, right) = (
            left.trim().parse::<u32>().unwrap(),
            right.trim().parse::<u32>().unwrap(),
        );
        if let Some(x) = map.get_mut(&left) {
            x.push(right);
        } else {
            map.insert(left, vec![right]);
        }
    }

    let mut updates = vec![];
    #[allow(clippy::while_let_on_iterator)]
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        updates.push(
            line.split(',')
                .map(|x| x.trim().parse::<u32>().unwrap())
                .collect::<Vec<_>>(),
        );
    }

    (map, updates)
}

pub fn part1_naive(input: &str) -> u32 {
    let (map, updates) = parse_input(input);
    let mut sum = 0;
    for update in &updates {
        let mut valid = true;
        'out: for i in 0..update.len() {
            let check = match map.get(&{ update[i] }) {
                None => continue,
                Some(x) => x,
            };
            for j in 0..i {
                if check.contains(&update[j]) {
                    valid = false;
                    break 'out;
                }
            }
            for j in i + 1..update.len() {
                if !check.contains(&update[j]) {
                    valid = false;
                    break 'out;
                }
            }
        }
        if valid {
            sum += update[update.len() >> 1];
        }
    }
    sum
}

pub fn bool2order(b: Option<bool>) -> Ordering {
    match b {
        None => Ordering::Equal,
        Some(true) => Ordering::Less,
        Some(false) => Ordering::Greater,
    }
}

pub fn part2_naive(input: &str) -> u32 {
    let (map, mut updates) = parse_input(input);

    let mut sum = 0;
    for update in &updates {
        let mut valid = true;
        'out: for i in 0..update.len() {
            let check = match map.get(&{ update[i] }) {
                None => continue,
                Some(x) => x,
            };
            for j in 0..i {
                if check.contains(&update[j]) {
                    valid = false;
                    break 'out;
                }
            }
            for j in i + 1..update.len() {
                if !check.contains(&update[j]) {
                    valid = false;
                    break 'out;
                }
            }
        }
        if valid {
            sum += update[update.len() >> 1];
        }
    }

    for update in &mut updates {
        update.sort_by(|a, b| bool2order(map.get(a).map(|x| x.contains(b))));
    }

    let mut sum2 = 0;
    for update in &updates {
        sum2 += update[update.len() >> 1];
    }
    sum2 - sum
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 2: {}", part2_naive(input));
    println!("Part 1: {}", part1_naive(input));
}

#[cfg(test)]
mod d5 {
    use super::*;

    #[test]
    fn p1_naive() {
        assert_eq!(part1_naive(SAMPLE), 143);
    }

    #[test]
    fn p2_naive() {
        assert_eq!(part2_naive(SAMPLE), 123);
    }
}
