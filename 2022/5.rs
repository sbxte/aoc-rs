use std::io::{Read, Write};
use std::fs::OpenOptions;

use regex::Regex;



fn main() {
    let mut file = OpenOptions::new()
                .read(true)
                // .open("./test.txt").unwrap();
                .open("./foo.txt").unwrap();

    let mut string = String::new();
    let _ = file.read_to_string(&mut string);
    string = String::from(string.trim_end());

    // Solve here


    // Just manual hardcode lol
    // let mut crates = [
    //     vec!('Z', 'N'),
    //     vec!('M', 'C', 'D'),
    //     vec!('P'),
    // ];

    let mut crates = [
        vec!('D', 'M', 'S', 'Z', 'R', 'F', 'W', 'N'),
        vec!('W', 'P', 'Q', 'G', 'S'),
        vec!('W', 'R', 'V', 'Q', 'F', 'N', 'J', 'C'),
        vec!('F', 'Z', 'P', 'C', 'G', 'D', 'L'),
        vec!('T', 'P', 'S'),
        vec!('H', 'D', 'F', 'W', 'R', 'L'),
        vec!('Z', 'N', 'D', 'C'),
        vec!('W', 'N', 'R', 'F', 'V', 'S', 'J', 'Q'),
        vec!('R', 'M', 'S', 'G', 'Z', 'W', 'V'),
    ];

    // [N]     [C]                 [Q]
    // [W]     [J] [L]             [J] [V]
    // [F]     [N] [D]     [L]     [S] [W]
    // [R] [S] [F] [G]     [R]     [V] [Z]
    // [Z] [G] [Q] [C]     [W] [C] [F] [G]
    // [S] [Q] [V] [P] [S] [F] [D] [R] [S]
    // [M] [P] [R] [Z] [P] [D] [N] [N] [M]
    // [D] [W] [W] [F] [T] [H] [Z] [W] [R]
    // 1   2   3   4   5   6   7   8   9

    let pattern = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let mut vec = vec!();

    for s in string.split("\n") {
        if s.is_empty() { break; }

        let m = pattern.captures(s).unwrap();
        // println!("{} {} {}", &m[1], &m[2], &m[3]);

        let count = m[1].parse::<usize>().unwrap();
        let from = m[2].parse::<usize>().unwrap();
        let to = m[3].parse::<usize>().unwrap();

        // Part 1
        // for _ in 0..count {
        //     let val = crates[from - 1].pop().unwrap();
        //     crates[to - 1].push(val);
        // }

        // Part 2
        for _ in 0..count {
            let val = crates[from - 1].pop().unwrap();
            vec.push(val);
        }
        for _ in 0..count {
            crates[to - 1].push(vec.pop().unwrap());
        }

        // let sl = crates[from - 1].as_slice();
        // let _ = &   sl[0..1];
    }


    for mut cr in crates {
        print!("{}", cr.pop().unwrap());
    }
    ::std::io::stdout().flush().unwrap();
}
