use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    time::Instant,
};

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    combinator::{all_consuming, map},
    multi::separated_list1,
    sequence::{preceded, tuple},
    Finish, IResult,
};

fn day16(_input: &str) {
    let time_start = Instant::now();
    let net = Network::new();
    let state = State {
        net: &net,
        position: Name(*b"AA"),
        max_turns: 30,
        turn: 0,
        pressure: 0,
        open_valves: Default::default(),
    };

    let (state, moves) = state.find_best_moves();
    println!("moves = {:?}, final pressure = {}", moves, state.pressure);
    println!("Elapsed time: {:?}", time_start.elapsed());
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Name([u8; 2]);

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let [a, b] = self.0;
        write!(f, "{}{}", a, b)
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl Name {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(take(2usize), |slice: &str| {
            Self(slice.as_bytes().try_into().unwrap())
        })(i)
    }
}

#[derive(Debug)]
struct Valve {
    name: Name,
    flow: u64,
    links: Vec<Name>,
}

impl Valve {
    fn parse(i: &str) -> IResult<&str, Self> {
        map(
            tuple((
                preceded(tag("Valve "), Name::parse),
                preceded(tag(" has flow rate="), nom::character::complete::u64),
                preceded(
                    alt((
                        tag("; tunnels lead to valves "),
                        tag("; tunnel leads to valve "),
                    )),
                    separated_list1(tag(", "), Name::parse),
                ),
            )),
            |(name, flow, links)| Self { name, flow, links },
        )(i)
    }
}

struct Network {
    valves: HashMap<Name, Valve>,
}

type Path = Vec<(Name, Name)>;

impl Network {
    #![allow(clippy::new_without_default)]
    fn new() -> Self {
        Self {
            valves: include_str!("../foo.txt")
                .lines()
                .map(|l| all_consuming(Valve::parse)(l).finish().unwrap().1)
                .map(|valve| (valve.name, valve))
                .collect(),
        }
    }

    fn connections(&self, start: Name) -> HashMap<Name, Path> {
        let mut current: HashMap<Name, Path> = Default::default();
        current.insert(start, vec![]);

        let mut connections = current.clone();

        while !current.is_empty() {
            let mut next: HashMap<Name, Path> = Default::default();
            for (name, path) in current {
                for link in self.valves[&name].links.iter().copied() {
                    if let Entry::Vacant(e) = connections.entry(link) {
                        let conn_path: Path = path
                            .iter()
                            .copied()
                            .chain(std::iter::once((name, link)))
                            .collect();
                        e.insert(conn_path.clone());
                        next.insert(link, conn_path);
                    }
                }
            }
            current = next;
        }

        connections
    }
}

#[derive(Debug, Clone)]
struct Move {
    reward: u64,
    target: Name,
    path: Path,
}

impl Move {
    fn cost(&self) -> u64 {
        let travel_turns = self.path.len() as u64;
        let open_turns = 1_u64;
        travel_turns + open_turns
    }
}

#[derive(Clone)]
struct State<'a> {
    net: &'a Network,
    position: Name,
    max_turns: u64,
    turn: u64,
    pressure: u64,
    open_valves: HashSet<Name>,
}

impl State<'_> {
    fn turns_left(&self) -> u64 {
        self.max_turns - self.turn
    }

    /// Compute all moves and expected reward (pressure contributed till time
    /// runs out if we travel to it and open it now)
    fn moves(&self) -> Vec<Move> {
        self.net
            .connections(self.position)
            .into_iter()
            .filter_map(|(name, path)| {
                if self.open_valves.contains(&name) {
                    return None;
                }

                let flow = self.net.valves[&name].flow;
                if flow == 0 {
                    return None;
                }

                let travel_turns = path.len() as u64;
                let open_turns = 1_u64;
                let turns_spent_open = self.turns_left().checked_sub(travel_turns + open_turns)?;
                let reward = flow * turns_spent_open;
                Some(Move {
                    reward,
                    target: name,
                    path,
                })
            })
            .collect()
    }

    /// Apply a given move
    fn apply(&self, mv: &Move) -> Self {
        let mut next = self.clone();
        next.position = mv.target;
        next.turn += mv.cost();
        next.pressure += mv.reward;
        next.open_valves.insert(mv.target);
        next
    }

    fn find_best_moves(&self) -> (Self, Vec<Move>) {
        let mut best_moves = vec![];
        let mut best_state = self.clone();

        for mv in self.moves() {
            let next = self.apply(&mv);
            let (next, mut next_moves) = next.find_best_moves();
            next_moves.push(mv);
            if next.pressure > best_state.pressure {
                best_moves = next_moves;
                best_state = next;
            }
        }

        (best_state, best_moves)
    }
}
