use std::ops::{Add, Neg, Sub};

/// Represents cartesian-coordinate structures that utilizes integer coordinates
pub trait Grid: Eq {
    /// [Pos] should be cheap to [Copy] as it will be used for referencing very often
    type Pos: Eq + Copy + Add + Sub + Neg;

    /// A Grid should contain Cells
    type Cell: Eq;

    /// Returns whether `pos` is inside the grid's bounds
    fn contains_pos(&self, pos: Self::Pos) -> bool;

    /// If [pos] is out-of-bounds then this method returns [None]
    fn get_cell(&self, pos: Self::Pos) -> Option<&Self::Cell> {
        if !self.contains_pos(pos) {
            None
        } else {
            Some(self.get_cell_unchecked(pos))
        }
    }

    fn get_cell_unchecked(&self, pos: Self::Pos) -> &Self::Cell;

    /// If [pos] is out-of-bounds then this method returns [None]
    fn get_cell_mut(&mut self, pos: Self::Pos) -> Option<&mut Self::Cell> {
        if !self.contains_pos(pos) {
            None
        } else {
            Some(self.get_cell_mut_unchecked(pos))
        }
    }

    fn get_cell_mut_unchecked(&mut self, pos: Self::Pos) -> &mut Self::Cell;

    /// Returns the direct neighbours of a cell.
    /// Should NOT include diagonal neighbours.
    fn get_neighbours(&self, pos: Self::Pos) -> impl Iterator<Item = &Self::Cell>;

    /// Returns the direct neighbours of a cell.
    /// Should NOT include diagonal neighbours.
    fn get_neighbours_pos(&self, pos: Self::Pos) -> impl Iterator<Item = Self::Pos>;

    /// Maps cells with a function. Retains the same [Pos][Grid::Pos] type.
    fn map<F, T>(self, f: F) -> impl Grid<Pos = Self::Pos, Cell = T>
    where
        F: Fn(Self::Cell) -> T,
        T: Eq;
}
