use std::collections::HashSet;
use std::fmt::Display;

use colored::Colorize;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Cell {
    Empty,
    Blocked(Hits, usize),
}

impl Cell {
    pub fn blocked(&self) -> bool {
        matches!(self, Self::Blocked(_, _))
    }

    pub fn hit(&mut self, dir: Direction, id: usize) {
        if let Self::Blocked(hits, hitid) = self {
            if *hitid != id {
                hits.up = 0;
                hits.down = 0;
                hits.right = 0;
                hits.left = 0;
            }
            *hitid = id;
            match dir {
                Direction::Up => hits.up += 1,
                Direction::Down => hits.down += 1,
                Direction::Right => hits.right += 1,
                Direction::Left => hits.left += 1,
            }
        }
    }

    pub fn is_loop(&self, dir: Direction, id: usize) -> bool {
        match self {
            Self::Blocked(hits, hitid) => {
                id == *hitid
                    && match dir {
                        Direction::Up => hits.up > 1,
                        Direction::Down => hits.down > 1,
                        Direction::Right => hits.right > 1,
                        Direction::Left => hits.left > 1,
                    }
            }
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct Hits {
    up: u32,
    down: u32,
    right: u32,
    left: u32,
}

impl Hits {
    pub fn new() -> Self {
        Self {
            up: 0,
            down: 0,
            right: 0,
            left: 0,
        }
    }

    pub fn count(&self) -> u32 {
        self.up + self.down + self.right + self.left
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn rotate(self) -> Self {
        use self::Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub fn step(self) -> (i32, i32) {
        use self::Direction::*;
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Right => (1, 0),
            Left => (-1, 0),
        }
    }
}

pub fn part2(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let cols = lines[0].len();
    let rows = lines.len();
    let mut grid = Vec::with_capacity(cols * rows);

    let (mut gc, mut gr) = (0, 0);
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.trim().chars().enumerate() {
            if ch == '.' {
                grid.push(Cell::Empty);
            }
            if ch == '#' {
                grid.push(Cell::Blocked(Hits::new(), 0));
            }
            if ch == '^' {
                grid.push(Cell::Empty);
                (gc, gr) = (c, r);
            }
        }
    }
    let (ogc, ogr) = (gc, gr);

    let mut dir = Direction::Up;
    let mut set = HashSet::new();
    loop {
        let step = dir.step();
        let next = (gc as i32 + step.0, gr as i32 + step.1);
        if !(0..cols as i32).contains(&next.0) || !(0..rows as i32).contains(&next.1) {
            break;
        }
        if grid[next.1 as usize * cols + next.0 as usize].blocked() {
            dir = dir.rotate();
        } else {
            (gc, gr) = (next.0 as usize, next.1 as usize);
            set.insert((gc, gr));
        }
    }

    let mut hit_id = 1;
    let mut result = 0;
    for passed_pos in set {
        dir = Direction::Up;
        (gc, gr) = (ogc, ogr);
        grid[passed_pos.1 * cols + passed_pos.0] = Cell::Blocked(Hits::new(), 0);
        loop {
            let step = dir.step();
            let next = (gc as i32 + step.0, gr as i32 + step.1);
            if !(0..cols as i32).contains(&next.0) || !(0..rows as i32).contains(&next.1) {
                break;
            }
            if grid[next.1 as usize * cols + next.0 as usize].blocked() {
                grid[next.1 as usize * cols + next.0 as usize].hit(dir, hit_id);
                if grid[next.1 as usize * cols + next.0 as usize].is_loop(dir, hit_id) {
                    result += 1;
                    break;
                }
                dir = dir.rotate();
            } else {
                (gc, gr) = (next.0 as usize, next.1 as usize);
            }
        }
        grid[passed_pos.1 * cols + passed_pos.0] = Cell::Empty;
        hit_id += 1;
    }

    result
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

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Blocked(hits, id) => {
                let i = (id % 10).to_string();
                write!(f, "{}", match hits.count() {
                    2.. => i.green(),
                    1 => i.blue(),
                    0 => "#".white(),
                })
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample() {
        assert_eq!(part2(crate::SAMPLE), 6);
    }

    #[test]
    fn off_the_side() {
        let input = "...#..
.....#
..#...
....#.
^.....";
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn u_turn() {
        let input = "...#..
......
......
......
...^..
.#....
....#.
#.....
...#..";
        assert_eq!(part2(input), 1);

        let input = "...#..
....#.
......
......
...^..
......
......
......
...#..";
        assert_eq!(part2(input), 1);

        let input = "...#..
....#.
......
......
...^..
.#....
......
#.....
...#..";
        assert_eq!(part2(input), 2);

        let input = "......
....#.
......
......
...^..
.#....
....#.
#.....
...#..";
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn a_box() {
        let input = ".#....
.....#
......
.^....
#.....
......";
        assert_eq!(part2(input), 1);

        let input = "......
.....#
......
.^....
#.....
....#.";
        assert_eq!(part2(input), 1);

        let input = "......
......
......
.^...#
#.....
....#.";
        assert_eq!(part2(input), 1);

        let input = ".....
..#..
.^...
#....
.#...";
        assert_eq!(part2(input), 1);

        let input = ".....
..#..
.#.#.
..^..
.....";
        assert_eq!(part2(input), 1);
    }
}
