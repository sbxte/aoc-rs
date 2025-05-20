use aocutils::cartes::dim2::dir::Direction;
use aocutils::cartes::dim2::grid::Grid2;
use aocutils::cartes::grid::Grid;
use aocutils::cartes::pathing::Pathable;
use aocutils::cartes::pathing::dijkstra::{DijkstraInPlace, PathData, dijkstra_in_place};
use aocutils::cartes::pos::Pos as _;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Cell<P, C> {
    Wall,
    Air,
    Start,
    End,
    Path(PathData<P, C>),
}
impl<P, C> Cell<P, C> {
    fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            b'#' => Some(Self::Wall),
            b'S' => Some(Self::Start),
            b'E' => Some(Self::End),
            b'.' => Some(Self::Air),
            _ => None,
        }
    }
}
impl<P, C> Pathable for Cell<P, C> {
    fn can_pass(&self) -> bool {
        matches!(self, Self::Air)
    }
}
impl<P, C> DijkstraInPlace<P, C> for Cell<P, C> {
    fn pathed(&mut self, from: P, cost: C) {
        *self = Self::Path(PathData { from, cost });
    }
    fn get_path(&self) -> Option<&PathData<P, C>> {
        if let Self::Path(d) = self {
            Some(d)
        } else {
            None
        }
    }
}

pub fn part1(input: &str) -> usize {
    let mut grid = Grid2::from_str_2(input, Cell::from_byte);
    let start = grid
        .iter_mut()
        .find(|c| matches!(c.1, Cell::Start))
        .unwrap()
        .0;
    let end = grid
        .iter_mut()
        .find(|c| matches!(c.1, Cell::End))
        .unwrap()
        .0;
    grid[start] = Cell::Air;
    grid[end] = Cell::Air;

    let path = dijkstra_in_place(&mut grid, start, end).expect("Unable to find path");
    let mut count = 0;
    for pos in path.iter {
        for dir in Direction::iter_all() {
            let mut step = dir.step();
            step = step * 2;

            let p = pos + step;
            if !grid.contains_pos(p) || grid[p].get_path().is_none() {
                continue;
            }
            // cost saved
            let diff = grid[p].get_path().unwrap().cost
                - grid[pos].get_path().unwrap().cost
                - p.taxicab_dst(pos);
            if diff >= 100 {
                count += 1;
            }
        }
    }

    count
}
