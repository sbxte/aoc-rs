use std::fmt::Display;

use aocutils::cartes::dim2::grid::{Grid2, Pos};
use aocutils::cartes::pathing::Pathable;
use aocutils::cartes::pathing::astar::astar_oneshot;

fn parse(input: &str) -> Vec<Pos> {
    let mut v = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let (x, y) = line.split_once(',').unwrap();
        let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
        v.push(Pos::from((x, y)));
    }
    v
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Cell {
    Empty,
    Block,
    Path,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Block => write!(f, "#"),
            Self::Path => write!(f, "O"),
        }
    }
}
impl Pathable for Cell {
    fn can_pass(&self) -> bool {
        matches!(self, Cell::Empty)
    }
}

pub fn part1(input: &str) -> usize {
    let blocks = parse(input);
    // 71 because [0, 70] (0-inclusive)
    let mut grid = Grid2::new_fill_with(|| Cell::Empty, 71, 71);
    for block in blocks[..1024].iter() {
        grid[*block] = Cell::Block;
    }
    let path = astar_oneshot(
        &grid,
        From::from((0, 0)),
        From::from((grid.cols as isize - 1, grid.rows as isize - 1)),
    )
    .unwrap();

    for p in path.iter {
        grid[p] = Cell::Path;
    }
    grid.print_display();

    path.steps - 1 // Exclude starting pos
}
