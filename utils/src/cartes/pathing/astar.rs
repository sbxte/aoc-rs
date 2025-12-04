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
    /// G cost, H cost
    Closed(Cost, Cost, Pos),
}

impl<C, P> Pathable for PathState<C, P> {
    #[inline]
    fn can_pass(&self) -> bool {
        matches!(self, Self::Free)
    }
}

#[derive(PartialEq, Eq)]
struct CellRef<G>
where
    G: Grid,
{
    pos: G::Pos,
    from: G::Pos,
    g_cost: <G::Pos as Pos>::N,
    h_cost: <G::Pos as Pos>::N,
}

impl<G> CellRef<G>
where
    G: Grid,
    <G::Pos as Pos>::N: Zero,
{
    #[inline]
    /// Constructs from a [pos], initialized with zero cost
    fn from_pos(pos: G::Pos, h_cost: <G::Pos as Pos>::N) -> Self {
        Self {
            g_cost: <G::Pos as Pos>::N::zero(),
            h_cost,
            pos,
            from: pos,
        }
    }
}
impl<G> PartialOrd for CellRef<G>
where
    G: Grid,
    G::Pos: Eq + Pos,
    <G::Pos as Pos>::N: Eq + Copy + Ord + Add<Output = <G::Pos as Pos>::N>,
{
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<G> Ord for CellRef<G>
where
    G: Grid,
    G::Pos: Eq + Pos,
    <G::Pos as Pos>::N: Eq + Copy + Ord + Add<Output = <G::Pos as Pos>::N>,
{
    #[inline]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.g_cost + self.h_cost)
            .cmp(&(other.g_cost + other.h_cost))
            .then(self.h_cost.cmp(&other.h_cost))
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
///
/// Equivalent to [dijkstra][super::dijkstra] but with a heuristic function applied
pub fn astar_oneshot<G>(
    grid: &G,
    start: G::Pos,
    end: G::Pos,
) -> Option<Path<impl Iterator<Item = G::Pos> + use<G>, G::Pos>>
where
    G: Grid + Clone,
    <<G as Grid>::Pos as Pos>::N:
        Zero + One + Copy + Ord + ToUsize + Add<Output = <<G as Grid>::Pos as Pos>::N>,
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
    open.push(Reverse(CellRef::<G>::from_pos(
        start,
        start.taxicab_dst(end),
    )));

    let mut steps = 0usize;
    let mut path_found = false;
    while let Some(opened) = open.pop() {
        if !grid.get_cell(opened.0.pos).unwrap().can_pass() {
            continue;
        }
        *grid.get_cell_mut(opened.0.pos).unwrap() = PathState::Closed(
            opened.0.g_cost,
            opened.0.pos.taxicab_dst(end),
            opened.0.from,
        );
        if opened.0.pos == end {
            steps = if let PathState::Closed(c, _, _) = grid.get_cell(opened.0.pos).unwrap() {
                c.to_usize() + 1
            } else {
                unreachable!("PathState is not closed");
            };
            path_found = true;
            break;
        }
        for pos in grid.get_neighbours_adj_pos(opened.0.pos) {
            if let Some(c) = grid.get_cell(pos)
                && c.can_pass()
            {
                open.push(Reverse(CellRef {
                    pos,
                    g_cost: opened.0.g_cost + One::one(),
                    h_cost: pos.taxicab_dst(end),
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
        if let PathState::Closed(_, _, f) = grid.get_cell(cell).unwrap() {
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

#[cfg(test)]
mod tests {
    #[test]
    fn oneshot_2d() {}
}
