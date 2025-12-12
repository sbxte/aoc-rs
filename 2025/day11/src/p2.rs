use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq, Clone)]
struct BfsState<'a> {
    server: &'a str,
    prev: &'a str,
    depth: u32,
    visited_targets: u32,
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
    visited: usize,
    target_paths: (u64, u64),
    inputs: Vec<&'a str>,
    outputs: Vec<&'a str>,
}

// Discard paths of lower tiers
// Path tiers are defined as how many paths to this section and how many targets have these path
// gone through
fn update_paths(mut server: (u64, u64), prev: (u64, u64)) -> (u64, u64) {
    if prev.1 > server.1 {
        server.1 = prev.1;
        server.0 = prev.0;
    } else if prev.1 == server.1 {
        server.0 += prev.0;
    }
    server
}

// Having visualized the problem's graph structure using Gephi,
// I now know the structure and the trick for this problem...
pub fn part2(input: &str) -> u64 {
    let mut servers: HashMap<&str, BfsData> = HashMap::new();

    // Add output server as a node because it does not exist in input
    servers.insert(
        "out",
        BfsData {
            ..Default::default()
        },
    );

    // Parse inputs
    for line in input.lines() {
        let (server, outputs_s) = line.split_once(": ").unwrap();
        let outputs: Vec<_> = outputs_s.split(' ').collect();

        // Initiqalize both outputs and inputs or add them if entry already exists
        for output in &outputs {
            if let Some(output) = servers.get_mut(output) {
                output.inputs.push(server);
            } else {
                servers.insert(
                    output,
                    BfsData {
                        inputs: vec![server],
                        ..Default::default()
                    },
                );
            }
        }
        if let Some(server) = servers.get_mut(server) {
            server.outputs = outputs;
        } else {
            servers.insert(
                server,
                BfsData {
                    outputs,
                    ..Default::default()
                },
            );
        }
    }

    // Initialize path tiers
    for output in servers["svr"].outputs.clone() {
        servers.get_mut(output).unwrap().target_paths = (1, 0);
    }

    // BFS
    let mut open = BinaryHeap::new();
    for next in &servers["svr"].outputs {
        open.push(BfsState {
            depth: 1,
            prev: "svr",
            server: next,
            visited_targets: 0,
        });
    }

    while let Some(state) = open.pop() {
        servers.get_mut(state.server).unwrap().visited += 1;

        // Only queue next nodes after all inputs received
        if servers[state.server].visited == servers[state.server].inputs.len()
            && state.server != "out"
        {
            for output in &servers[state.server].outputs {
                let mut next = state.clone();
                next.server = output;
                next.prev = state.server;
                next.depth += 1;
                open.push(next);
            }
        }

        if matches!(state.server, "fft" | "dac") {
            // Mark as having gone through another target
            let ps = servers[state.prev].target_paths;
            servers.get_mut(state.server).unwrap().target_paths =
                update_paths(servers[state.server].target_paths, (ps.0, ps.1 + 1))
        } else {
            servers.get_mut(state.server).unwrap().target_paths = update_paths(
                servers[state.server].target_paths,
                servers[state.prev].target_paths,
            );
        }
    }
    servers["out"].target_paths.0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        assert_eq!(part2(input), 2);
    }
}
