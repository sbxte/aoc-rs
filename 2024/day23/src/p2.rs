use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};

fn parse(input: &str) -> impl Iterator<Item = (&str, &str)> {
    input.lines().map(|l| l.split_once('-').unwrap())
}

pub fn part2(input: &str) -> String {
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

    let mut groups: Vec<Vec<&str>> = vec![];
    for (n1, n2, n3) in trios {
        let trio_perm = [(n1, [n2, n3]), (n2, [n1, n3]), (n3, [n1, n2])];
        let mut part_of = false;
        for group in &mut groups {
            for (n, n_o) in trio_perm {
                if !group.contains(n) {
                    continue;
                }

                if group
                    .iter()
                    .all(|gn| gn == n_o[0] || graph[*gn].contains(n_o[0]))
                    && group
                        .iter()
                        .all(|gn| gn == n_o[1] || graph[*gn].contains(n_o[1]))
                {
                    for n in n_o {
                        if !group.contains(n) {
                            group.push(n);
                        }
                    }
                    part_of = true;
                }
            }
        }
        if !part_of {
            groups.push(trio_perm.iter().map(|(n, _)| **n).collect());
        }
    }

    let mut group = groups
        .into_iter()
        .map(|e| (e.len(), e))
        .max_by(|(l1, _), (l2, _)| l1.cmp(l2))
        .unwrap()
        .1;
    group.sort();
    group.join(",")
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
        assert_eq!(part2(input), "co,de,ka,ta");
    }
}
