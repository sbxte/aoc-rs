use std::cell::RefCell;
use std::rc::Rc;

use crate::{AugmentedTreeNode, add_child, intersects_node};

pub fn part1(input: &str) -> u64 {
    let (ranges, ids) = input
        .trim()
        .split_once("\n\n")
        .expect("Unable to split at empty line");

    let tree_head = {
        let (left, right) = ranges
            .trim()
            .lines()
            .next()
            .expect("Unable to obtain first range")
            .split_once('-')
            .expect("Unable to split range");
        let (start, end) = (
            left.parse::<u64>().expect("Unable to parse range start"),
            right.parse::<u64>().expect("Unable to parse range end"),
        );
        let node = AugmentedTreeNode {
            start,
            end,
            alpha: end,
            _parent: None,
            left: None,
            right: None,
        };
        Rc::new(RefCell::new(node))
    };

    for range in ranges.trim().lines().skip(1) {
        let (left, right) = range.trim().split_once('-').expect("Unable to split range");
        let (start, end) = (
            left.parse::<u64>().expect("Unable to parse range start"),
            right.parse::<u64>().expect("Unable to parse range end"),
        );
        add_child(&tree_head, start, end);
    }

    let mut sum = 0;
    for id in ids.trim().lines() {
        let id = id.parse::<u64>().expect("Unable to parse ID");
        if intersects_node(&tree_head, id) {
            sum += 1;
        }
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";
        assert_eq!(part1(input), 3);
    }
}
