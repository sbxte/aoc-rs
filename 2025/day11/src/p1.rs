use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq, Clone)]
struct BfsState<'a> {
    server: &'a str,
    prev: &'a str,
    depth: u32,
}
impl<'a> PartialOrd for BfsState<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a> Ord for BfsState<'a> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.depth.cmp(&other.depth).reverse() // lowest first
    }
}

#[derive(Debug, Default)]
struct BfsData<'a> {
    // Minimum forward bfs depth
    min_fdepth: u32,
    bpaths: u32,
    bvisisted: bool,
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
}

pub fn part1(input: &str) -> u32 {
    let mut servers = HashMap::new();
    for line in input.lines() {
        let (server, outputs_s) = line.split_once(": ").unwrap();
        let outputs_s: Vec<_> = outputs_s.split(' ').collect();
        servers.insert(
            server,
            BfsData {
                min_fdepth: u32::MAX,
                outputs: outputs_s,
                ..Default::default()
            },
        );
    }
    // Add output server as a node because it does not exist in input
    servers.insert(
        "out",
        BfsData {
            min_fdepth: u32::MAX,
            ..Default::default()
        },
    );

    // Forward Bfs
    let mut open = BinaryHeap::new();
    open.push(BfsState {
        server: "you",
        prev: "you",
        depth: 0,
    });

    while let Some(state) = open.pop() {
        if !servers[state.server].inputs.contains(&state.prev) {
            servers
                .get_mut(state.server)
                .unwrap()
                .inputs
                .push(state.prev);
        }
        if servers[state.server].min_fdepth != u32::MAX {
            continue;
        }
        servers.get_mut(state.server).unwrap().min_fdepth = servers
            .get(state.server)
            .unwrap()
            .min_fdepth
            .min(state.depth);

        for output in &servers[state.server].outputs {
            if servers[output].min_fdepth != u32::MAX {
                continue;
            }
            let mut next = state.clone();
            next.server = output;
            next.prev = state.server;
            next.depth += 1;
            open.push(next);
        }
    }
    // Prune out unreachable servers
    servers.retain(|_k, v| v.min_fdepth != u32::MAX);
    

    // Backward Bfs
    let mut open = BinaryHeap::new();
    for input in &servers["out"].inputs {
        open.push(Reverse(BfsState {
            // Max depth first so reverse cmp again
            server: input,
            prev: "out",
            depth: servers[input].min_fdepth,
        }));
    }
    servers.get_mut("out").unwrap().bpaths = 1;

    while let Some(Reverse(state)) = open.pop() {
        if state.server != "you" && !servers[state.server].bvisisted {
            for input in &servers[state.server].inputs {
                let mut next = state.clone();
                next.server = input;
                next.prev = state.server;
                next.depth = servers[input].min_fdepth;
                open.push(Reverse(next));
            }
        }

        servers.get_mut(state.server).unwrap().bpaths += servers[state.prev].bpaths;
        servers.get_mut(state.server).unwrap().bvisisted = true; 
    }
    servers["you"].bpaths
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        assert_eq!(part1(input), 5);
    }

    #[test]
    fn sample2() {
        let input = "you: aaa
aaa: bbb
bbb: ccc
ccc: out";
        assert_eq!(part1(input), 1);
    }
}
