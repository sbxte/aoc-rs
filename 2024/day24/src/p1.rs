use std::collections::hash_map::Entry;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Gate<'a> {
    left: &'a str,
    right: &'a str,
    op: Op,
    out: &'a str,
}

#[derive(Debug)]
enum Op {
    And,
    Or,
    Xor,
}
impl Op {
    fn from_str(input: &str) -> Self {
        match input {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            x => panic!("Invalid op: {x}"),
        }
    }

    fn op(&self, left: bool, right: bool) -> bool {
        match self {
            Self::And => left && right,
            Self::Or => left || right,
            Self::Xor => left ^ right,
        }
    }
}

#[derive(Debug)]
struct Wire {
    val: Option<bool>,
    out: Vec<usize>,
}

pub fn part1(input: &str) -> usize {
    let (wires_s, gates_s) = input.split_once("\n\n").unwrap();

    let mut wires = HashMap::new();
    let mut gates = vec![];
    let mut updates = VecDeque::new();

    for wire_l in wires_s.lines() {
        let (wire_n, val) = wire_l.split_once(": ").unwrap();
        wires.insert(
            wire_n.trim(),
            Wire {
                val: Some(val.trim().parse::<u8>().unwrap() > 0),
                out: vec![],
            },
        );
    }

    for (gate_i, gate_l) in gates_s.lines().enumerate() {
        let mut gate_s = gate_l.split(' ');
        let left = gate_s.next().unwrap();
        let op = Op::from_str(gate_s.next().unwrap());
        let right = gate_s.next().unwrap();
        let out = gate_s.nth(1).unwrap();

        match wires.entry(left) {
            Entry::Vacant(e) => {
                e.insert(Wire {
                    val: None,
                    out: vec![gate_i],
                });
            }
            Entry::Occupied(mut o) => {
                o.get_mut().out.push(gate_i);
            }
        }
        match wires.entry(right) {
            Entry::Vacant(e) => {
                e.insert(Wire {
                    val: None,
                    out: vec![gate_i],
                });
            }
            Entry::Occupied(mut o) => {
                o.get_mut().out.push(gate_i);
            }
        }
        if let Entry::Vacant(e) = wires.entry(out) {
            e.insert(Wire {
                val: None,
                out: vec![],
            });
        }

        if matches!(&left[0..1], "x" | "y") && matches!(&right[0..1], "x" | "y") {
            updates.push_back(gate_i);
        }

        gates.push(Gate {
            left,
            right,
            op,
            out,
        });
    }

    while let Some(gate_i) = updates.pop_front() {
        let gate = &gates[gate_i];

        let left = wires[gate.left].val.unwrap();
        let right = wires[gate.right].val.unwrap();
        wires.get_mut(gate.out).unwrap().val = Some(gate.op.op(left, right));

        for gate_i in &wires.get(gate.out).unwrap().out {
            let l = gates[*gate_i].left;
            let r = gates[*gate_i].right;
            if wires.get(l).unwrap().val.is_some() && wires.get(r).unwrap().val.is_some() {
                updates.push_back(*gate_i);
            }
        }
    }

    let mut outputs = wires
        .iter()
        .filter(|(k, _)| k.starts_with('z'))
        .collect::<Vec<_>>();
    outputs.sort_by(|(l1, _), (l2, _)| l1.cmp(l2).reverse());
    outputs
        .iter()
        .map(|(_, v)| v.val.unwrap())
        .fold(0, |acc, x| (acc << 1) + x as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sample() {
        let input = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02";
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn big_sample() {
        let input = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";
        assert_eq!(part1(input), 2024);
    }
}
