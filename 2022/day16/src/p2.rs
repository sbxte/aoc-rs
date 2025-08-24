use std::collections::{BinaryHeap, HashMap};

type Network<'a> = HashMap<&'a str, (u32, Vec<&'a str>)>;

pub fn parse(input: &str) -> Network<'_> {
    let mut network = HashMap::new();
    for line in input.lines() {
        let (valve_str, tunnels_str) = line.split_once(';').unwrap();
        let mut valve_words = valve_str.split(' ');
        let valve = valve_words.nth(1).unwrap();
        let (_, rate) = valve_words.nth(2).unwrap().split_once('=').unwrap();
        let rate = rate.parse::<u32>().unwrap();

        let tunnels_str = tunnels_str.trim().split(' ').skip(4);
        let tunnels: Vec<_> = tunnels_str
            .map(|s| s.strip_suffix(',').unwrap_or(s).trim())
            .collect();

        network.insert(valve, (rate, tunnels));
    }
    network
}

mod heuristics {
    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct State<'a> {
        pos: &'a str,
        next: &'a str,
        steps: u32,
    }
    impl<'a> Ord for State<'a> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.steps.cmp(&other.steps).reverse()
        }
    }
    impl<'a> PartialOrd for State<'a> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    pub fn best_path<'a>(network: &Network<'a>, start: &'a str, end: &'a str) -> (&'a str, u32) {
        let mut open = BinaryHeap::new();

        for next in &network[start].1 {
            open.push(State {
                pos: next,
                next,
                steps: 1,
            });
        }

        let mut closed = vec![];
        while let Some(opened) = open.pop() {
            if opened.pos == end {
                return (opened.next, opened.steps);
            }
            closed.push(opened.pos);

            for neighbour in &network[opened.pos].1 {
                if closed.contains(neighbour) {
                    continue;
                }
                open.push(State {
                    pos: neighbour,
                    next: opened.next,
                    steps: opened.steps + 1,
                });
            }
        }

        panic!("Unable to find next path");
    }

    pub type NetHeu<'a> = HashMap<(&'a str, &'a str), (&'a str, u32)>;

    pub fn net_heuristics<'a>(network: &Network<'a>) -> NetHeu<'a> {
        let mut heu = NetHeu::new();

        for target in network
            .iter()
            .filter_map(|(valve, (rate, _a))| if *rate > 0 { Some(valve) } else { None })
        {
            for valve in network.keys() {
                heu.insert((valve, target), best_path(network, valve, target));
            }
        }

        heu
    }
}

mod finale {
    use crate::p2::heuristics::NetHeu;

    use super::*;

    #[derive(Debug, PartialEq, Eq, Clone)]
    struct State<'a> {
        /// You
        pos_a: &'a str,
        target_a: &'a str,
        /// Elephant
        pos_b: &'a str,
        target_b: &'a str,
        rem_time: u32,
        pressure: u32,
        open_valves: Vec<&'a str>,
    }
    impl<'a> Ord for State<'a> {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.rem_time
                .cmp(&other.rem_time)
                .then(self.pressure.cmp(&other.pressure))
        }
    }
    impl<'a> PartialOrd for State<'a> {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    pub fn best_pressure(network: Network, path: NetHeu) -> u32 {
        let openable_valves = network.values().filter(|(rate, _adj)| *rate > 0).count();

        let mut open = BinaryHeap::new();
        for target_a in network
            .iter()
            .filter_map(|(valve, (rate, _adj))| if *rate > 0 { Some(valve) } else { None })
        {
            for target_b in network
                .iter()
                .filter_map(|(valve, (rate, _adj))| if *rate > 0 { Some(valve) } else { None })
            {
                if target_a == target_b {
                    continue;
                }
                open.push(State {
                    pos_a: "AA",
                    target_a,
                    pos_b: "AA",
                    target_b,
                    rem_time: 26,
                    pressure: 0,
                    open_valves: vec![],
                });
            }
        }

        let mut highest_pressure = 0;
        let mut prev_high_press = 0;
        let mut prev_rem_time = 26;

        while let Some(opened) = open.pop() {
            if highest_pressure > prev_high_press || opened.rem_time < prev_rem_time {
                prev_high_press = highest_pressure;
                prev_rem_time = opened.rem_time;
            }

            if opened.pressure > highest_pressure {
                highest_pressure = opened.pressure;
            }

            if opened.open_valves.len() == openable_valves || opened.rem_time == 0 {
                continue;
            }

            let rem_time = opened.rem_time - 1;

            // Culling
            let theoretical_pressure = network
                .iter()
                .filter(|(valve, (rate, _adj))| *rate > 0 && !opened.open_valves.contains(valve))
                .fold(0, |acc, (valve, (rate, _adj))| {
                    acc + rate
                        * (rem_time + 2
                            - path[&(opened.pos_a, &**valve)]
                                .1
                                .min(path[&(opened.pos_b, &**valve)].1))
                });
            if opened.pressure + theoretical_pressure <= highest_pressure {
                continue;
            }

            let open_a =
                opened.target_a == opened.pos_a && !opened.open_valves.contains(&opened.pos_a);
            let open_b =
                opened.target_b == opened.pos_b && !opened.open_valves.contains(&opened.pos_b);

            // Open A and B
            if open_a && open_b && opened.pos_a != opened.pos_b {
                let mut activate_ab = opened.open_valves.clone();
                activate_ab.push(opened.pos_a);
                activate_ab.push(opened.pos_b);
                let pressure_a = network[opened.pos_a].0 * rem_time;
                let pressure_b = network[opened.pos_b].0 * rem_time;
                let pressure = opened.pressure + pressure_a + pressure_b;

                let openable_valves = network.iter().filter_map(|(valve, (rate, _adj))| {
                    if *rate > 0 && !activate_ab.contains(valve) {
                        Some(valve)
                    } else {
                        None
                    }
                });
                if openable_valves.clone().count() == 0 {
                    open.push(State {
                        pos_a: opened.pos_a,
                        pos_b: opened.pos_b,
                        target_a: opened.target_a,
                        target_b: opened.target_b,
                        rem_time,
                        pressure,
                        open_valves: activate_ab.clone(),
                    });
                    continue;
                }
                for target_a in openable_valves.clone() {
                    for target_b in openable_valves.clone() {
                        if target_a == target_b {
                            continue;
                        }
                        open.push(State {
                            pos_a: opened.pos_a,
                            pos_b: opened.pos_b,
                            target_a,
                            target_b,
                            rem_time,
                            pressure,
                            open_valves: activate_ab.clone(),
                        });
                    }
                }
            }
            // Move B and Open A
            else if open_a {
                let mut activate_a = opened.open_valves.clone();
                activate_a.push(opened.pos_a);
                let pressure_a = network[opened.pos_a].0 * rem_time;
                let pressure = opened.pressure + pressure_a;

                let openable_valves = network.iter().filter_map(|(valve, (rate, _adj))| {
                    if *rate > 0 && !activate_a.contains(valve) {
                        Some(valve)
                    } else {
                        None
                    }
                });
                if openable_valves.clone().count() == 0 {
                    open.push(State {
                        pos_a: opened.pos_a,
                        pos_b: opened.pos_b,
                        target_a: opened.target_a,
                        target_b: opened.target_b,
                        rem_time,
                        pressure,
                        open_valves: activate_a.clone(),
                    });
                    continue;
                }
                let next_b = path[&(opened.pos_b, opened.target_b)].0;
                for target_a in openable_valves {
                    open.push(State {
                        pos_a: opened.pos_a,
                        pos_b: next_b,
                        target_a,
                        target_b: opened.target_b,
                        rem_time,
                        pressure,
                        open_valves: activate_a.clone(),
                    })
                }
            }
            // Move A and Open B
            else if open_b {
                let mut activate_b = opened.open_valves.clone();
                activate_b.push(opened.pos_b);
                let pressure_b = network[opened.pos_b].0 * rem_time;
                let pressure = opened.pressure + pressure_b;

                let openable_valves = network.iter().filter_map(|(valve, (rate, _adj))| {
                    if *rate > 0 && !activate_b.contains(valve) {
                        Some(valve)
                    } else {
                        None
                    }
                });
                if openable_valves.clone().count() == 0 {
                    open.push(State {
                        pos_a: opened.pos_a,
                        pos_b: opened.pos_b,
                        target_a: opened.target_a,
                        target_b: opened.target_b,
                        rem_time,
                        pressure,
                        open_valves: activate_b.clone(),
                    });
                    continue;
                }
                let next_a = path[&(opened.pos_a, opened.target_a)].0;
                for target_b in openable_valves {
                    open.push(State {
                        pos_a: next_a,
                        pos_b: opened.pos_b,
                        target_a: opened.target_a,
                        target_b,
                        rem_time,
                        pressure,
                        open_valves: activate_b.clone(),
                    })
                }
            }
            // Move A and B
            else {
                let steps_a = path[&(opened.pos_a, opened.target_a)].1;
                let steps_b = path[&(opened.pos_b, opened.target_b)].1;
                let steps = steps_a.min(steps_b);
                let mut pos_a = opened.pos_a;
                let mut pos_b = opened.pos_b;
                for _ in 0..steps {
                    pos_a = path[&(pos_a, opened.target_a)].0;
                    pos_b = path[&(pos_b, opened.target_b)].0;
                }
                if steps >= rem_time {
                    continue;
                }
                open.push(State {
                    pos_a,
                    pos_b,
                    target_a: opened.target_a,
                    target_b: opened.target_b,
                    rem_time: rem_time + 1 - steps,
                    pressure: opened.pressure,
                    open_valves: opened.open_valves.clone(),
                })
            }
        }

        highest_pressure
    }
}
pub fn part2(input: &str) -> u32 {
    let network = parse(input);

    let heuristics = heuristics::net_heuristics(&network);

    finale::best_pressure(network, heuristics)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

        assert_eq!(part2(input), 1707);
    }

    #[test]
    fn small() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves BB, CC
Valve BB has flow rate=2; tunnels lead to valves AA, CC
Valve CC has flow rate=3; tunnels lead to valves BB, AA";

        assert_eq!(part2(input), 120);
    }
}
