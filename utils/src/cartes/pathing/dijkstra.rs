use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::cartes::grid::Grid;

use super::{CellRef, Pathable};

#[derive(Debug, PartialEq, Eq)]
enum PathState<Cost, Pos> {
    Impassable,
    Free,
    Closed(Cost, Pos),
}

impl<C, P> Pathable for PathState<C, P> {
    fn can_pass(&self) -> bool {
        !matches!(self, Self::Impassable)
    }
}

/// Calculates a path in one go. Does not alter cell states in-place and only provides the final path
/// result.
///
/// Returns [None] when no found is found or if [pos] is out-of-bounds,
/// otherwise returns an [Iterator] containing the cell positions from [start] to [end]
///
/// Uses a [BinaryHeap] to sort for lowest cost open cells to check
/// first.
pub fn path_oneshot<G>(grid: &G, start: G::Pos, end: G::Pos) -> Option<impl Iterator<Item = G::Pos>>
where
    G: Grid + Clone,
    G::Cell: Pathable,
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
    open.push(Reverse(CellRef::from_pos(start)));

    let mut steps = 0;
    let mut path_found = false;
    while let Some(opened) = open.pop() {
        if let PathState::Free = grid.get_cell(opened.0.pos).unwrap() {
            *grid.get_cell_mut(opened.0.pos).unwrap() =
                PathState::Closed(opened.0.cost, opened.0.from);
        }
        if opened.0.pos == end {
            steps = opened.0.cost;
            path_found = true;
            break;
        }
        for pos in grid.get_neighbours_pos(opened.0.pos) {
            if let Some(c) = grid.get_cell(pos)
                && c.can_pass()
            {
                open.push(Reverse(CellRef {
                    pos,
                    cost: opened.0.cost + 1,
                    from: opened.0.pos,
                }))
            }
        }
    }

    if !path_found {
        return None;
    }

    // Retrace steps from the end
    let mut path = Vec::with_capacity(steps as usize);
    let mut cell = end;
    while cell != start {
        path.push(cell);
        if let PathState::Closed(_, f) = grid.get_cell(cell).unwrap() {
            cell = *f;
        }
    }
    path.push(start);
    path.reverse();

    Some(path.into_iter())
}

#[cfg(test)]
mod tests {
    #[test]
    fn oneshot_2d() {}
}
