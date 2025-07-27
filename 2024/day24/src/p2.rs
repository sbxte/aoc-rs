use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::{File, OpenOptions};
use std::io::Write as _;

#[derive(Debug)]
struct Gate<'a> {
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
}

#[derive(Debug)]
struct Wire {
    out: Vec<usize>,
}

struct GraphData<'a> {
    wires: HashMap<&'a str, Wire>,
    gates: Vec<Gate<'a>>,
}

// This section is taken from part 1
fn parse<'a>(input: &'a str) -> GraphData<'a> {
    let (wires_s, gates_s) = input.split_once("\n\n").unwrap();

    let mut wires = HashMap::new();
    let mut gates = vec![];

    for wire_l in wires_s.lines() {
        let (wire_n, _val) = wire_l.split_once(": ").unwrap();
        let wire_n = wire_n.trim();
        wires.insert(wire_n, Wire { out: vec![] });
    }

    for (gate_i, gate_l) in gates_s.lines().enumerate() {
        let mut gate_s = gate_l.split(' ');
        let left = gate_s.next().unwrap();
        let op = Op::from_str(gate_s.next().unwrap());
        let right = gate_s.next().unwrap();
        let out = gate_s.nth(1).unwrap();

        match wires.entry(left) {
            Entry::Vacant(e) => {
                e.insert(Wire { out: vec![gate_i] });
            }
            Entry::Occupied(mut o) => {
                o.get_mut().out.push(gate_i);
            }
        }
        match wires.entry(right) {
            Entry::Vacant(e) => {
                e.insert(Wire { out: vec![gate_i] });
            }
            Entry::Occupied(mut o) => {
                o.get_mut().out.push(gate_i);
            }
        }
        if let Entry::Vacant(e) = wires.entry(out) {
            e.insert(Wire { out: vec![] });
        }

        gates.push(Gate { op, out });
    }

    GraphData { wires, gates }
}

fn write_output(mut file: File, data: GraphData) {
    let mut buffer = String::new();
    buffer.push_str("strict digraph {\n");
    for (g, gate) in data.gates.iter().enumerate() {
        let op = match gate.op {
            Op::And => "AND",
            Op::Or => "OR",
            Op::Xor => "XOR",
        };
        let color = match gate.op {
            Op::And => "red",
            Op::Or => "blue",
            Op::Xor => "green",
        };
        let out = gate.out;
        buffer.push_str(&format!(
            "g{g} [label=\"{op}\" color=\"{color}\" comment=\"{out}\"];\n"
        ));

        buffer.push_str(&format!("g{g} -> {out} [color=\"{color}\"];\n"));
    }
    for (wname, wire) in data.wires.iter() {
        if wname.starts_with('x') || wname.starts_with('y') {
            buffer.push_str(&format!("{wname} [color=\"white\"];\n"));
        } else if wname.starts_with('z') {
            buffer.push_str(&format!("{wname} [color=\"gray\"];\n"));
        } else {
            buffer.push_str(&format!("{wname} [color=\"magenta\"];\n"));
        }
        for o in &wire.out {
            buffer.push_str(&format!("{wname} -> g{o} [color=\"lightgray\"];\n"));
        }
    }
    buffer.push_str("}");

    file.write_all(buffer.as_bytes()).unwrap();
}

pub fn part2(input: &str) {
    let data = parse(input);

    let mut path = std::env::current_dir().unwrap();
    path.push("src/output.dot");
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(path)
        .unwrap();

    write_output(file, data);

    // After saving to a DOT file,
    // load the graph in Gephi (or some other graph visualizer)
    // and find the swapped wires
}
