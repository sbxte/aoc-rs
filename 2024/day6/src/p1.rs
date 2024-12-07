#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Cell {
    Empty,
    Block,
    Passed,
}

impl Cell {
    pub fn blocked(&self) -> bool {
        self == &Self::Block
    }

    pub fn passed(&self) -> bool {
        self == &Self::Passed
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum GuardDirection {
    Up,
    Right,
    Down,
    Left,
}

impl GuardDirection {
    pub fn rotate(self) -> Self {
        use self::GuardDirection::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    pub fn step(self) -> (i32, i32) {
        use self::GuardDirection::*;
        match self {
            Up => (0, -1),
            Down => (0, 1),
            Right => (1, 0),
            Left => (-1, 0),
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let lines: Vec<_> = input.lines().collect();
    let cols = lines[0].len();
    let rows = lines.len();
    let mut grid = Vec::with_capacity(cols * rows);

    let (mut gc, mut gr) = (0, 0);
    for (r, line) in lines.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch == '.' {
                grid.push(Cell::Empty);
            }
            if ch == '#' {
                grid.push(Cell::Block);
            }
            if ch == '^' {
                grid.push(Cell::Passed);
                (gc, gr) = (c, r);
            }
        }
    }

    let mut dir = GuardDirection::Up;

    loop {
        let step = dir.step();
        let next = (gc as i32 + step.0, gr as i32 + step.1);
        if !(0..cols as i32).contains(&next.0) || !(0..rows as i32).contains(&next.1) {
            grid[gr * cols + gc] = Cell::Passed;
            break;
        }
        if grid[next.1 as usize * cols + next.0 as usize].blocked() {
            dir = dir.rotate();
        } else {
            grid[gr * cols + gc] = Cell::Passed;
            (gc, gr) = (next.0 as usize, next.1 as usize);
        }
    }

    let result = grid
        .iter()
        .fold(0, |acc, x| if x.passed() { acc + 1 } else { acc });

    result
}
