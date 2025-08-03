use std::cmp::Reverse;
use std::collections::BinaryHeap;

use aocutils::cartes::dim2::dir::Direction;
use aocutils::cartes::dim2::grid::Pos;
use aocutils::cartes::pos::Pos as _;

pub fn parse_npad(c: char) -> Pos {
    From::from(match c {
        'A' => (2, 3),
        '0' => (1, 3),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        x => unreachable!("Invalid numpad value! {}", x),
    })
}

#[derive(Debug)]
enum DirpadVal {
    Direction(Direction),
    Press,
}

pub fn parse_dir_dpad(dir: Direction) -> Pos {
    From::from(match dir {
        Direction::Up => (1, 0),
        Direction::Down => (1, 1),
        Direction::Right => (2, 1),
        Direction::Left => (0, 1),
    })
}
pub fn get_press_dpad() -> Pos {
    From::from((2, 0))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CellRef {
    pos: Pos,
    g_cost: usize,
    h_cost: usize,
}
impl PartialOrd for CellRef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CellRef {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.g_cost + self.h_cost).cmp(&(other.g_cost + other.h_cost))
    }
}

#[inline]
fn add(
    bheap: &mut BinaryHeap<Reverse<CellRef>>,
    pos: Pos,
    target: Pos,
    cost: usize,
    bounds: (
        ::std::ops::RangeInclusive<isize>,
        ::std::ops::RangeInclusive<isize>,
    ),
) {
    if !bounds.0.contains(&pos.0) || !bounds.1.contains(&pos.1) {
        return;
    }
    bheap.push(Reverse(CellRef {
        pos,
        g_cost: cost,
        h_cost: pos.taxicab_dst(target) as usize,
    }))
}
pub fn bfs_npad(start: Pos, end: Pos) -> usize {
    let dpad_depth = 3;
    let mut dpad_bufs = [BinaryHeap::new(), BinaryHeap::new(), BinaryHeap::new()];
    let mut dpad_positions = [get_press_dpad(), get_press_dpad(), get_press_dpad()];

    // let dpad_depth = 1;
    // let mut dpad_bufs = [BinaryHeap::new()];
    // let mut dpad_positions = [get_press_dpad()];

    let mut open = BinaryHeap::new();
    open.push(Reverse(CellRef {
        pos: start,
        g_cost: 0,
        h_cost: 0,
    }));

    while let Some(opened) = open.pop() {
        let opened = opened.0;

        if opened.pos == end {
            return opened.g_cost
                + bfs_dpad(
                    dpad_depth - 1,
                    get_press_dpad(),
                    &mut dpad_positions,
                    &mut dpad_bufs,
                );
        }

        for d in Direction::iter_all() {
            let p = opened.pos + d.step();
            if p.taxicab_dst(end) > opened.pos.taxicab_dst(end) {
                continue;
            }
            let dir = parse_dir_dpad(d);
            let cost = bfs_dpad(dpad_depth - 1, dir, &mut dpad_positions, &mut dpad_bufs);
            add(&mut open, p, end, opened.g_cost + cost, ((0..=2), (0..=3)));
        }
    }

    unreachable!("Unable to find path at npad layer")
}

// Path to and then press
fn bfs_dpad(
    depth: usize,
    target: Pos,
    positions: &mut [Pos],
    bheaps: &mut [BinaryHeap<Reverse<CellRef>>],
) -> usize {
    let start = positions[depth];
    bheaps[depth].clear();
    bheaps[depth].push(Reverse(CellRef {
        pos: start,
        g_cost: 0,
        h_cost: 0,
    }));

    if depth == 0 {
        return 1;
    }

    while let Some(opened) = bheaps[depth].pop() {
        if opened.0.pos == target {
            positions[depth] = opened.0.pos;
            // NOTE: Might ignore other costs
            return opened.0.g_cost + bfs_dpad(depth - 1, get_press_dpad(), positions, bheaps);
        }
        for d in Direction::iter_all() {
            let p = opened.0.pos + d.step();
            if p.taxicab_dst(target) > opened.0.pos.taxicab_dst(target) {
                continue;
            }
            let dir = parse_dir_dpad(d);
            let cost = bfs_dpad(depth - 1, dir, positions, bheaps);
            add(
                &mut bheaps[depth],
                p,
                target,
                opened.0.g_cost + cost,
                ((0..=2), (0..=1)),
            );
        }
    }
    unreachable!("Unable to find path at dpad layer {}", depth)
}

pub fn part1(input: &str) -> usize {
    let mut complexity = 0;
    for line in input.trim().lines() {
        let mut cost = 0;
        let mut pos = parse_npad('A');
        for c in line.trim().chars() {
            let to = parse_npad(c);
            cost += dbg!(bfs_npad(pos, to));
            pos = to;
        }
        dbg!(cost);
        complexity += line[..3].parse::<usize>().as_ref().unwrap() * cost;
    }
    complexity
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample = "029A
980A
179A
456A
379A";

        assert_eq!(part1(sample), 126384);
    }

    #[test]
    fn dbg() {
        let sample = "029A";
        assert_eq!(part1(sample), 0);
    }
}
