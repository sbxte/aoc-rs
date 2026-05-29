// Robot 1: Actual keypad
// Robot 2: Directional keypad (high levels of radiation)
// Robot 3: Directional keypad (-40 degrees)
// You: Directional keypad (full of damn historians, instant pressing)

use std::collections::{BinaryHeap, HashMap};

use aocutils::cartes::dim2::dir::Direction;
use aocutils::cartes::dim2::vec::Vec2;

type Coord = Vec2<i8>;

fn action2coord_dirpad(action: DirpadAction) -> Coord {
    match action {
        DirpadAction::Press => Coord::from((2, 1)),
        DirpadAction::Move(Direction::Up) => Coord::from((1, 0)),
        DirpadAction::Move(Direction::Right) => Coord::from((2, 0)),
        DirpadAction::Move(Direction::Down) => Coord::from((1, 1)),
        DirpadAction::Move(Direction::Left) => Coord::from((0, 0)),
    }
}

fn char2coord_numpad(c: char) -> Coord {
    match c {
        '7' => Coord::from((0, 3)),
        '8' => Coord::from((1, 3)),
        '9' => Coord::from((2, 3)),
        '4' => Coord::from((0, 2)),
        '5' => Coord::from((1, 2)),
        '6' => Coord::from((2, 2)),
        '1' => Coord::from((0, 1)),
        '2' => Coord::from((1, 1)),
        '3' => Coord::from((2, 1)),
        '0' => Coord::from((1, 0)),
        'A' => Coord::from((2, 0)),
        x => unreachable!("Invalid numpad char: {}", x),
    }
}

const NUMPAD_VOID: Coord = const { Coord { 0: 0, 1: 0 } };
const DIRPAD_VOID: Coord = const { Coord { 0: 0, 1: 1 } };

const LEVELS: usize = 26;
type COST = u64;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum DirpadAction {
    Move(Direction),
    Press,
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct PathState {
    // 2: numpad bot
    // 1: dirpad bot
    // 0: dirpad bot
    // -: you (instant movement, each button cost is 1)
    pos: [Coord; LEVELS],
    pushed: bool,
    g_cost: COST,
    h_cost: COST,
}

impl PathState {
    fn cost(&self) -> COST {
        self.g_cost + self.h_cost
    }
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost().cmp(&other.cost()).reverse()
    }
}

impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Hash, PartialEq, Eq)]
struct DpState {
    pos: Coord,
    depth: usize,
    action: DirpadAction,
}
type DpMemo = HashMap<DpState, COST>;

// Move to and then press
fn path_numpad(from: Coord, to: Coord) -> COST {
    let mut open = BinaryHeap::new();

    // State returns cost. BHeap is MaxHeap.
    // Reverse to minimize cost
    let a_coord = action2coord_dirpad(DirpadAction::Press);
    let mut first_state = PathState {
        pos: [a_coord; LEVELS],
        pushed: false,
        g_cost: 0,
        h_cost: from.manhattan_dist(to) as COST,
    };
    first_state.pos[LEVELS - 1] = from;
    open.push(first_state);

    let mut dp_memo: DpMemo = HashMap::new();

    while let Some(mut state) = open.pop() {
        if state.pos[LEVELS - 1] == to && state.pushed {
            return state.g_cost;
        } else if state.pos[LEVELS - 1] == to {
            // At target, now press
            let (newpos, subcost) =
                path_dirpad(DirpadAction::Press, state.pos, LEVELS - 2, &mut dp_memo);
            state.pos = newpos;
            state.g_cost += subcost;

            state.pushed = true;
            open.push(state);
        } else {
            // Move towards target
            let p = state.pos[LEVELS - 1];
            let prev_dist = p.manhattan_dist(to);
            let dirs = Direction::iter_all().filter(|d| {
                let next = p + d.step();
                next != NUMPAD_VOID && next.manhattan_dist(to) < prev_dist
            });
            for dir in dirs {
                let mut next_state = state;

                let (newpos, subcost) = path_dirpad(
                    DirpadAction::Move(dir),
                    next_state.pos,
                    LEVELS - 2,
                    &mut dp_memo,
                );

                next_state.pos = newpos;
                next_state.g_cost += subcost;
                next_state.pos[LEVELS - 1] += dir.step();

                open.push(next_state);
            }
        }
    }

    unreachable!("No path found at numpad layer!");
}

fn path_dirpad(
    action: DirpadAction,
    initial_state: [Coord; LEVELS],
    depth: usize,
    dp_memo: &mut DpMemo,
) -> ([Coord; LEVELS], COST) {
    // Move and then press
    let to = action2coord_dirpad(action);
    let mut open = BinaryHeap::new();
    open.push(PathState {
        pos: initial_state,
        g_cost: 0,
        h_cost: initial_state[depth].manhattan_dist(to) as COST,
        pushed: false,
    });

    while let Some(mut state) = open.pop() {
        if state.pos[depth] == to && state.pushed {
            return (state.pos, state.g_cost);
        } else if state.pos[depth] == to {
            if depth == 0 {
                return (state.pos, state.g_cost + 1);
            } else if let Some(cost) = dp_memo.get(&DpState {
                pos: state.pos[depth - 1],
                depth,
                action: DirpadAction::Press,
            }) {
                state.pos[depth - 1] = action2coord_dirpad(DirpadAction::Press);
                state.g_cost += *cost;
            } else {
                // At target, now press
                let (newpos, subcost) =
                    path_dirpad(DirpadAction::Press, state.pos, depth - 1, dp_memo);

                dp_memo.insert(
                    DpState {
                        pos: state.pos[depth - 1],
                        depth,
                        action: DirpadAction::Press,
                    },
                    subcost,
                );

                state.pos = newpos;
                state.g_cost += subcost;
            }

            state.pushed = true;
            open.push(state);
        } else {
            // Move towards target
            let p = state.pos[depth];
            let prev_dist = p.manhattan_dist(to);
            let dirs = Direction::iter_all().filter(|d| {
                let next = p + d.step();
                next != DIRPAD_VOID && next.manhattan_dist(to) < prev_dist
            });
            for dir in dirs {
                let mut next_state = state;
                if depth == 0 {
                    // its just you, you move instantly
                    next_state.g_cost += 1;
                } else if let Some(cost) = dp_memo.get(&DpState {
                    pos: state.pos[depth - 1],
                    depth,
                    action: DirpadAction::Move(dir),
                }) {
                    next_state.pos[depth - 1] = action2coord_dirpad(DirpadAction::Move(dir));
                    next_state.g_cost += *cost;
                } else {
                    let (newpos, subcost) =
                        path_dirpad(DirpadAction::Move(dir), next_state.pos, depth - 1, dp_memo);

                    dp_memo.insert(
                        DpState {
                            pos: next_state.pos[depth - 1],
                            depth,
                            action: DirpadAction::Move(dir),
                        },
                        subcost,
                    );

                    next_state.pos = newpos;
                    next_state.g_cost += subcost;
                }
                next_state.pos[depth] += dir.step();

                open.push(next_state);
            }
        }
    }

    unreachable!("No path found at dirpad layer {}!", depth);
}

pub fn part2(input: &str) -> COST {
    let mut total = 0;
    for line in input.lines() {
        let mut prev = char2coord_numpad('A');
        let mut cost = 0;
        for c in line.chars() {
            let to = char2coord_numpad(c);
            let deltacost = path_numpad(prev, to);
            cost += deltacost;
            prev = to;
        }
        let num = line[..line.len() - 1].parse::<COST>().unwrap();
        let delta = num * cost;
        total += delta;
    }
    return total;
}
