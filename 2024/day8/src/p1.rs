pub mod naive {

    use std::collections::HashMap;
    use std::fmt::Display;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Cell {
        content: Content,
        antinode: bool,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum Content {
        Empty,
        Node,
    }

    impl Display for Cell {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.content)
        }
    }

    impl Display for Content {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Empty => write!(f, "."),
                Self::Node => write!(f, "N"),
            }
        }
    }

    impl Cell {
        pub fn new(content: Content) -> Self {
            Self {
                content,
                antinode: false,
            }
        }
    }

    pub fn part1(input: &str) -> u32 {
        let lines: Vec<_> = input.lines().collect();
        let cols = lines[0].len();
        let rows = lines.len();
        let mut grid = Vec::with_capacity(cols * rows);

        let mut node_groups: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        for (r, line) in lines.iter().enumerate() {
            for (c, ch) in line.chars().enumerate() {
                let (c, r) = (c as i32, r as i32);
                if ch == '.' {
                    grid.push(Cell::new(Content::Empty));
                } else {
                    if let std::collections::hash_map::Entry::Vacant(e) = node_groups.entry(ch) {
                        e.insert(vec![(c, r)]);
                    } else {
                        node_groups.get_mut(&ch).unwrap().push((c, r));
                    }
                    grid.push(Cell::new(Content::Empty));
                }
            }
        }

        for (_, group) in node_groups {
            for (c1, r1) in &group {
                for (c2, r2) in &group {
                    if (c1, r1) == (c2, r2) {
                        continue;
                    }
                    let (cx, rx) = (c1 - c2, r1 - r2);
                    if (0..cols as i32).contains(&(cx + c1))
                        && (0..rows as i32).contains(&(rx + r1))
                    {
                        let (ct, rt) = ((cx + c1) as usize, (rx + r1) as usize);
                        grid[rt * cols + ct].antinode = true;
                    }
                }
            }
        }

        let mut sum = 0;
        for e in grid {
            if e.antinode {
                sum += 1;
            }
        }

        sum as u32
    }

    pub fn display_graph(grid: &[Cell], cols: usize, rows: usize) {
        println!();
        for r in 0..rows {
            for c in 0..cols {
                print!("{}", grid[r * cols + c]);
            }
            println!();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn sample() {
        assert_eq!(naive::part1(SAMPLE), 14);
    }
}
