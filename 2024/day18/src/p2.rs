use std::fmt::Display;

use aocutils::cartes::dim2::grid::{Grid2, Pos};
use aocutils::cartes::pathing::Pathable;
use aocutils::cartes::pathing::astar::astar_oneshot;

type BlockOrder = Vec<Pos>;

fn parse(input: &str) -> BlockOrder {
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct BlockSimulator {
    order: BlockOrder,
    step: usize,
}

impl BlockSimulator {
    /// blocks = b[i]
    /// applied blocks b[i] for i < N where N = self.step
    ///
    /// E.g. step = 1, thus blocks applied = b[0] = b[0..1]
    /// step = 2, thus blocks applied = b[0], b[1] = b[0..2]
    fn set_step(&mut self, new_step: usize, grid: &mut Grid) {
        if new_step == self.step || new_step > self.order.len() {
            return;
        }
        if new_step > self.step {
            for s in self.step..new_step {
                grid[self.order[s]] = Cell::Block;
            }
        } else {
            for s in new_step..self.step {
                grid[self.order[s]] = Cell::Empty;
            }
        }
        self.step = new_step;
    }
}

type Grid = Grid2<Cell>;

pub fn part2(input: &str) -> String {
    let order = parse(input);
    let mut simul = BlockSimulator { order, step: 0 };

    // 71 because [0, 70] (0-inclusive)
    let mut grid: Grid = Grid2::new_fill_with(|| Cell::Empty, 71, 71);

    // Binary search
    let mut low = 0;
    let mut high = simul.order.len();
    let mut mid = (low + high) >> 1;
    while mid != low {
        simul.set_step(mid, &mut grid);
        let has_path = astar_oneshot(
            &grid,
            From::from((0, 0)),
            From::from((grid.cols as isize - 1, grid.rows as isize - 1)),
        )
        .is_some();

        if has_path {
            low = mid;
        } else {
            high = mid;
        }

        mid = (low + high) >> 1;
        // when finished, mid = low, mid CANNOT be high
    }

    // visualize
    for p in astar_oneshot(
        &grid,
        From::from((0, 0)),
        From::from((grid.cols as isize - 1, grid.rows as isize - 1)),
    )
    .unwrap()
    .iter
    {
        grid[p] = Cell::Path;
    }
    grid.print_display();

    // Reset and check if next step truly has no path
    for (_, c) in grid.iter_mut() {
        if *c == Cell::Path {
            *c = Cell::Empty;
        }
    }
    simul.set_step(mid + 1, &mut grid);
    assert!(
        astar_oneshot(
            &grid,
            From::from((0, 0)),
            From::from((grid.cols as isize - 1, grid.rows as isize - 1)),
        )
        .is_none()
    );

    let block = simul.order[mid];
    format!("{},{}", block.0, block.1)
}
