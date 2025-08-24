mod p1;
mod p2;

fn main() {
    let input = include_str!("input.txt");

    #[cfg(feature = "dot")]
    {
        use std::env;
        use std::fs::OpenOptions;
        use std::io::Write as _;

        let mut path = env::current_dir().unwrap();
        path.push("graph.dot");
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(path)
            .unwrap();
        let network = p2::parse(input);
        let mut buffer = String::new();
        buffer.push_str("strict digraph {{\n");
        for (v, (r, adj)) in network {
            for a in adj {
                let color = if r > 0 { "green" } else { "gray" };
                buffer.push_str(&format!(
                    "{v} [comment=\"{v}\" color=\"{color}\"];\n{v} -> {a} [color=\"black\"];\n"
                ));
            }
        }
        buffer.push_str("}");
        file.write_all(buffer.as_bytes()).unwrap();
        return;
    }

    // println!("Day 16 part 1: {}", p1::part1(input));
    #[cfg(not(feature = "dot"))]
    {
        println!("Day 16 part 2: {}", p2::part2(input));
    }
}
