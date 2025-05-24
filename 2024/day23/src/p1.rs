use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> impl Iterator<Item = (&str, &str)> {
    input.lines().map(|l| l.split_once('-').unwrap())
}

pub fn part1(input: &str) -> usize {
    let mut graph = HashMap::new();
    for (a, b) in parse(input) {
        match graph.entry(a) {
            Entry::Vacant(e) => {
                e.insert(vec![b]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(b);
            }
        }
        match graph.entry(b) {
            Entry::Vacant(e) => {
                e.insert(vec![a]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(a);
            }
        }
    }

    let mut trios = HashSet::new();
    let mut table = HashSet::new();
    for (n1, v) in &graph {
        table.clear();
        for n2 in v {
            table.insert(n2);
        }
        for n2 in v {
            for n3 in &graph[n2] {
                if !table.contains(n3) {
                    continue;
                }
                if !trios.contains(&(n1, n2, n3))
                    && !trios.contains(&(n1, n3, n2))
                    && !trios.contains(&(n2, n1, n3))
                    && !trios.contains(&(n2, n3, n1))
                    && !trios.contains(&(n3, n1, n2))
                    && !trios.contains(&(n3, n2, n1))
                {
                    trios.insert((n1, n2, n3));
                }
            }
        }
    }
    trios
        .iter()
        .filter(|(n1, n2, n3)| n1.starts_with('t') || n2.starts_with('t') || n3.starts_with('t'))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!(part1(input), 7);
    }
}
