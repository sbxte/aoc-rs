use aocutils::cartes::dim2::dir::Direction;
use aocutils::cartes::dim2::grid::{Grid2, Pos};
use aocutils::cartes::dim2::vec::Vec2;
use aocutils::cartes::grid::Grid as _;
use aocutils::optim::prelude::*;

type Grid = Grid2<Cell>;
fn get_robot_pos(grid: &Grid) -> Pos {
    grid.iter()
        .find(|(p, c)| matches!(c, Cell::Robot))
        .unwrap()
        .0
}

type Access = u16;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Cell {
    #[default]
    Empty,
    Wall,
    Robot,
    /// this int is the last step ID which accessed this cell
    BoxLeft(Access),
    BoxRight,
}

impl Cell {
    fn from_byte(c: u8) -> Self {
        use self::Cell::*;
        match c {
            b'.' => Empty,
            b'#' => Wall,
            b'O' => BoxLeft(0),
            b'@' => Robot,
            b'\n' => Wall,
            x => noreach!("from byte {}", x),
        }
    }

    fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Empty => '.',
                Self::Wall => '#',
                Self::Robot => '@',
                Self::BoxLeft(_) => '[',
                Self::BoxRight => ']',
            }
        )
    }
}

fn parse_movements(input: &str) -> Vec<Direction> {
    input.bytes().filter_map(parse_move).collect()
}

fn parse_move(c: u8) -> Option<Direction> {
    use Direction::*;
    match c {
        b'<' => Some(Left),
        b'^' => Some(Up),
        b'>' => Some(Right),
        b'v' => Some(Down),
        b'\n' => None,
        x => noreach!("parse move {}", x),
    }
}

fn parse_input(input: &str) -> (Grid, Pos, Vec<Direction>) {
    input
        .split_once("\n\n")
        .map(|(g, m)| (g.trim(), m.trim()))
        .map(|(g, m)| {
            let mut data = vec![];
            let mut rows = 1;
            for c in g.chars() {
                if c == '\n' {
                    rows += 1;
                    continue;
                }

                let cell = Cell::from_byte(c as u8);
                if matches!(cell, Cell::BoxLeft(_)) {
                    data.push(cell);
                    data.push(Cell::BoxRight);
                } else if matches!(cell, Cell::Robot) {
                    data.push(cell);
                    data.push(Cell::Empty);
                } else {
                    data.push(cell);
                    data.push(cell);
                }
            }
            let cols = g.lines().next().unwrap().chars().count() * 2;
            let grid = Grid2::from_raw(data, cols, rows);
            let robot = get_robot_pos(&grid);

            (grid, robot, parse_movements(m))
        })
        .unwrap()
}

/// Can move up or down
fn can_move_ud(grid: &mut Grid2<Cell>, pos: Pos, dir: Direction, access: Access) -> bool {
    match grid.get_cell_unchecked(pos) {
        Cell::BoxLeft(n) => {
            if *n == access {
                return true;
            }
            if let Cell::BoxLeft(n) = grid.get_cell_mut_unchecked(pos) {
                *n = access;
            }

            can_move_ud(grid, pos + dir.step(), dir, access)
                && can_move_ud(grid, pos + Direction::Right.step(), dir, access)
        }
        Cell::BoxRight => {
            can_move_ud(grid, pos + dir.step(), dir, access)
                && can_move_ud(grid, pos + Direction::Left.step(), dir, access)
        }
        Cell::Empty => true,
        Cell::Wall => false,
        Cell::Robot => unreachable!("You are not supposed to push back into the robot again"),
    }
}

fn move_box_ud(grid: &mut Grid2<Cell>, pos: Pos, dir: Direction, access: Access) {
    if grid.get_cell_unchecked(pos).is_empty() {
        return;
    }

    match grid.get_cell_unchecked(pos) {
        Cell::BoxLeft(n) => {
            if *n == access {
                return;
            }
            if let Cell::BoxLeft(n) = grid.get_cell_mut_unchecked(pos) {
                *n = access;
            }
            let right = pos + Direction::Right.step();
            move_box_ud(grid, pos + dir.step(), dir, access);
            move_box_ud(grid, right + dir.step(), dir, access);

            grid.swap_idx(pos.to_idx(grid.cols), (pos + dir.step()).to_idx(grid.cols));
            grid.swap_idx(
                right.to_idx(grid.cols),
                (right + dir.step()).to_idx(grid.cols),
            );
        }
        Cell::BoxRight => {
            move_box_ud(grid, pos + Direction::Left.step(), dir, access);
        }
        Cell::Robot => {
            move_box_ud(grid, pos + dir.step(), dir, access);
            grid.swap_idx(pos.to_idx(grid.cols), (pos + dir.step()).to_idx(grid.cols));
        }
        Cell::Wall => unreachable!("You are not supposed to hit a wall at this point"),
        _ => unreachable!(),
    }
}

fn try_move_robot(grid: &mut Grid2<Cell>, robot: &mut Pos, dir: Direction, access: Access) {
    let step = dir.step();

    if matches!(dir, Direction::Right | Direction::Left) {
        let mut can_move = false;
        let mut check = *robot; // copy

        while !check.is_oob_inclusive(
            Vec2::from((0, 0)),
            Vec2::from((grid.cols as isize - 1, grid.rows as isize)),
        ) {
            if let Cell::Empty = grid[check] {
                can_move = true;
                break;
            } else if let Cell::Wall = grid[check] {
                break;
            }
            check += step;
        }

        if !can_move {
            return;
        }

        let mut mv = check; // copy
        let rstep = dir.rot180().step();
        while mv != *robot {
            let next = mv + rstep;

            if cfg!(debug_assertions) {
                grid.swap_idx(Pos::to_idx(next, grid.cols), Pos::to_idx(mv, grid.cols));
            } else {
                unsafe {
                    grid.swap_idx_unchecked(
                        Pos::to_idx(next, grid.cols),
                        Pos::to_idx(mv, grid.cols),
                    );
                }
            }

            mv += rstep;
        }
    } else {
        // Move up and down requires additional checks
        let check = *robot + dir.step();
        if !can_move_ud(grid, check, dir, access) {
            return;
        }

        // access - 1 because access was used already for checking
        move_box_ud(grid, *robot, dir, access - 1);
    }

    *robot += dir.step();
}

fn calc_sum_gps(grid: &Grid) -> i32 {
    grid.as_slice().iter().enumerate().fold(0, |acc, (i, x)| {
        if let &Cell::BoxLeft(_) = x {
            let v = Pos::from_idx(i, grid.cols);
            acc + 100 * v.1 + v.0
        } else {
            acc
        }
    }) as i32
}

pub fn part2(input: &str) -> i32 {
    let (mut grid, mut robot, movements) = parse_input(input);
    if cfg!(any(feature = "vis2")) {
        grid.print_display();
    }
    for (i, m) in movements.iter().enumerate() {
        try_move_robot(&mut grid, &mut robot, *m, i as Access + 1);
        if cfg!(any(feature = "vis2")) {
            grid.print_display();
            println!("{i}: {m:?}");
        }
    }
    calc_sum_gps(&grid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn large_sample() {
        assert_eq!(part2(LARGE_SAMPLE), 9021);
    }
}
