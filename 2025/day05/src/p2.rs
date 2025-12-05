use std::cell::RefCell;
use std::rc::Rc;

use crate::{AugmentedTreeNode, add_child};

#[derive(Debug)]
struct SumState {
    count: u64,
    start: u64,
    end: u64,
}


// Returns cummulative count of fresh IDs and last end range
fn recurse(node: &Rc<RefCell<AugmentedTreeNode>>, state: &mut SumState) {
    // Recurse down to the left as much as possible
    if let Some(n) = node.borrow().left.as_ref() {
        recurse(n, state);
    }

    // Update pointers
    if node.borrow().start > state.end {
        // New segment, update count
        state.count += state.end - state.start + 1;
        state.start = node.borrow().start;
        state.end = node.borrow().end;
    } else if node.borrow().end > state.end {
        // Extend segment
        state.end = node.borrow().end;
    } // else, do nothing

    // Recurse down right
    if let Some(n) = node.borrow().right.as_ref() {
        recurse(n, state);
    }
}

pub fn part2(input: &str) -> u64 {
    let (ranges, _ids) = input
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

    let mut state = SumState {
        count: 0,
        start: 0,
        end: 0,
    };
    recurse(&tree_head, &mut state);

    // The initial sentinel (start: 0, end: 0) value produces an additional +1 count,
    // so subtract another 1 off
    state.count + (state.end - state.start + 1) - 1
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
        assert_eq!(part2(input), 14);
    }
}
