pub mod naive {
    use std::collections::HashMap;

    #[cfg(test)]
    use std::fmt::Display;

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Cell {
        content: Content,
        antinode: bool,
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum Content {
        Empty,
        Antenna,
    }

    pub fn part2(input: &str) -> u32 {
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
                    node_groups.entry(ch).or_default().push((c, r));
                    grid.push(Cell::new(Content::Antenna));
                }
            }
        }

        for (_, group) in node_groups {
            for (c1, r1) in &group {
                for (c2, r2) in &group {
                    if (c1, r1) == (c2, r2) {
                        continue;
                    }
                    let (dc, dr) = (c1 - c2, r1 - r2);
                    let (mut c, mut r) = (dc + c1, dr + r1);
                    while (0..cols as i32).contains(&c) && (0..rows as i32).contains(&r) {
                        grid[r as usize * cols + c as usize].antinode = true;
                        (c, r) = (dc + c, dr + r);
                    }
                }
            }
        }

        #[cfg(test)]
        display_graph(&grid, cols, rows);

        let mut sum = 0;
        for e in grid {
            if e.antinode || e.content == Content::Antenna {
                sum += 1;
            }
        }

        sum as u32
    }

    #[cfg(test)]
    pub fn display_graph(grid: &[Cell], cols: usize, rows: usize) {
        println!();
        for r in 0..rows {
            for c in 0..cols {
                print!("{}", grid[r * cols + c]);
            }
            println!();
        }
    }

    #[cfg(test)]
    impl Display for Cell {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if self.antinode {
                write!(f, "#")
            } else if let Content::Antenna = self.content {
                write!(f, "N")
            } else {
                write!(f, ".")
            }
        }
    }

    #[cfg(test)]
    impl Display for Content {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Empty => write!(f, "."),
                Self::Antenna => write!(f, "N"),
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
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn sample() {
        assert_eq!(naive::part2(SAMPLE), 34);
    }
}
