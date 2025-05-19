use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::ops::Add;

use crate::cartes::grid::Grid;
use crate::cartes::pos::Pos;
use crate::num::cast::ToUsize;
use crate::num::{One, Zero};

use super::{Path, Pathable};

#[derive(Debug, PartialEq, Eq)]
enum PathState<Cost, Pos> {
    Impassable,
    Free,
    Closed(Cost, Pos),
}

impl<C, P> Pathable for PathState<C, P> {
    fn can_pass(&self) -> bool {
        matches!(self, Self::Free)
    }
}

/// For use in algorithms which do not path in-place, i.e. do not rely on the given [Grid] to store
/// pathing states.
#[derive(PartialEq, Eq)]
struct CellRef<G>
where
    G: Grid,
{
    pos: G::Pos,
    from: G::Pos,
    cost: <G::Pos as Pos>::N,
}

impl<G> CellRef<G>
where
    G: Grid,
    <G::Pos as Pos>::N: Zero,
{
    /// Constructs from a [pos], initialized with zero cost
    fn from_pos(pos: G::Pos) -> Self {
        Self {
            cost: <G::Pos as Pos>::N::zero(),
            pos,
            from: pos,
        }
    }
}
impl<G> PartialOrd for CellRef<G>
where
    G: Grid,
    G::Pos: Eq + Pos,
    <G::Pos as Pos>::N: Eq + Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<G> Ord for CellRef<G>
where
    G: Grid,
    G::Pos: Eq + Pos,
    <G::Pos as Pos>::N: Eq + Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

/// Calculates a path in one go. Does not alter cell states in-place and only provides the final path
/// result.
///
/// Returns [None] when no found is found or if [pos] is out-of-bounds,
/// otherwise returns an [Path] containing an [Iterator] with cell positions from [start] to [end]
///
/// Uses a [BinaryHeap] to sort for lowest cost open cells to check
/// first.
pub fn dijkstra_oneshot<G>(
    grid: &G,
    start: G::Pos,
    end: G::Pos,
) -> Option<Path<impl Iterator<Item = G::Pos> + use<G>, G::Pos>>
where
    G: Grid + Clone,
    G::Cell: Pathable,
    <G::Pos as Pos>::N: Zero + One + Copy + Ord + ToUsize + Add<Output = <G::Pos as Pos>::N>,
{
    if !grid.contains_pos(start) || !grid.contains_pos(end) {
        return None;
    }

    let mut grid = grid.clone().map(|c| {
        if !c.can_pass() {
            PathState::Impassable
        } else {
            PathState::Free
        }
    });

    // BinaryHeap is a max-heap.
    // In order to make it a min-heap the value must be wrapped in a Reverse
    let mut open = BinaryHeap::new();
    open.push(Reverse(CellRef::<G>::from_pos(start)));

    let mut steps = 0usize;
    let mut path_found = false;
    while let Some(opened) = open.pop() {
        if !grid.get_cell(opened.0.pos).unwrap().can_pass() {
            continue;
        }
        *grid.get_cell_mut(opened.0.pos).unwrap() = PathState::Closed(opened.0.cost, opened.0.from);
        if opened.0.pos == end {
            steps = opened.0.cost.to_usize() + 1;
            path_found = true;
            break;
        }
        for pos in grid.get_neighbours_pos(opened.0.pos) {
            if let Some(c) = grid.get_cell(pos)
                && c.can_pass()
            {
                open.push(Reverse(CellRef {
                    pos,
                    cost: opened.0.cost + One::one(),
                    from: opened.0.pos,
                }))
            }
        }
    }

    if !path_found {
        return None;
    }

    // Retrace steps from the end
    let mut path = Vec::with_capacity(steps);
    let mut cell = end;
    while cell != start {
        path.push(cell);
        if let PathState::Closed(_, f) = grid.get_cell(cell).unwrap() {
            cell = *f;
        }
    }
    path.push(start);
    path.reverse();

    Some(Path {
        steps,
        iter: path.into_iter(),
    })
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PathData<Pos, Cost> {
    pub from: Pos,
    pub cost: Cost,
}

pub trait DijkstraInPlace<Pos, Cost>: Pathable {
    fn get_path(&self) -> Option<&PathData<Pos, Cost>>;

    /// Implementation
    ///
    /// Once this method is called on a [Cell][Grid::Cell]
    /// the cell must return [`false`] on [can_pass][Pathable::can_pass]
    /// as to let the pathing algorithm know not to backtrack into this cell
    fn pathed(&mut self, from: Pos, cost: Cost);
}

pub fn dijkstra_in_place<G>(
    grid: &mut G,
    start: G::Pos,
    end: G::Pos,
) -> Option<Path<impl Iterator<Item = G::Pos> + use<G>, G::Pos>>
where
    G: Grid + Clone,
    G::Cell: DijkstraInPlace<G::Pos, <G::Pos as Pos>::N>,
    <G::Pos as Pos>::N: Zero + One + Copy + Ord + ToUsize + Add<Output = <G::Pos as Pos>::N>,
{
    if !grid.contains_pos(start) || !grid.contains_pos(end) {
        return None;
    }

    // BinaryHeap is a max-heap.
    // In order to make it a min-heap the value must be wrapped in a Reverse
    let mut open = BinaryHeap::new();
    open.push(Reverse(CellRef::<G>::from_pos(start)));

    let mut steps = 0usize;
    while let Some(opened) = open.pop() {
        if !grid.get_cell(opened.0.pos).unwrap().can_pass() {
            continue;
        }

        grid.get_cell_mut(opened.0.pos)
            .unwrap()
            .pathed(opened.0.from, opened.0.cost);

        if opened.0.pos == end {
            steps = opened.0.cost.to_usize() + 1;
            break;
        }
        for pos in grid.get_neighbours_pos(opened.0.pos) {
            if let Some(c) = grid.get_cell(pos)
                && c.can_pass()
            {
                open.push(Reverse(CellRef {
                    pos,
                    cost: opened.0.cost + One::one(),
                    from: opened.0.pos,
                }))
            }
        }
    }

    if steps == 0 {
        return None;
    }

    // Retrace steps from the end
    let mut path = Vec::with_capacity(steps);
    let mut cell = end;
    while cell != start {
        path.push(cell);
        if let Some(d) = grid.get_cell(cell).unwrap().get_path() {
            cell = d.from;
        }
    }
    path.push(start);
    path.reverse();

    Some(Path {
        steps,
        iter: path.into_iter(),
    })
}

#[cfg(test)]
mod tests {
    use crate::cartes::dim2::grid::Grid2;

    use super::*;

    #[test]
    fn inplace() {
        #[derive(Debug, PartialEq, Eq, Clone)]
        enum Cell<P, C> {
            Wall,
            Air,
            Path(PathData<P, C>),
        }
        impl<P, C> Pathable for Cell<P, C> {
            fn can_pass(&self) -> bool {
                matches!(self, Self::Air)
            }
        }
        impl<P, C> DijkstraInPlace<P, C> for Cell<P, C> {
            fn get_path(&self) -> Option<&PathData<P, C>> {
                if let Self::Path(d) = self {
                    Some(d)
                } else {
                    None
                }
            }

            fn pathed(&mut self, from: P, cost: C) {
                *self = Self::Path(PathData { from, cost })
            }
        }

        let mut grid = Grid2::from_str_2(
            "00
10
00",
            |x| Some(if x == b'0' { Cell::Air } else { Cell::Wall }),
        );

        dijkstra_in_place(&mut grid, From::from((0, 0)), From::from((0, 2)));
    }
}
