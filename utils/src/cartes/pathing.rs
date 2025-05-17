use super::pos::Pos;

pub mod astar;
pub mod dijkstra;

/// Represents a cell which is part of a [Grid][super::grid::Grid] that are able to be passed to
/// pathing algorithms in this crate (e.g. [dijkstra])
pub trait Pathable {
    /// Returns true when pathing algorithms are allowed to pass through this cell
    fn can_pass(&self) -> bool;
}

#[derive(PartialEq, Eq, Clone)]
/// Output type of pathing algorithms.
/// Contains the number of steps (path length) and an [Iterator] containing cell positions from
/// start to end
pub struct Path<I, P>
where
    I: Iterator<Item = P>,
    P: Pos,
{
    /// Number of steps required to traverse this path, a.k.a. path length
    pub steps: usize,
    /// Spits out cell positions from starting position to the end position
    pub iter: I,
}

// Conditional implementation for when elements implement Debug
impl<I, P> ::std::fmt::Debug for Path<I, P>
where
    I: Iterator<Item = P>,
    P: Pos,
    I: ::std::fmt::Debug,
    P: ::std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Path")
            .field("steps", &self.steps)
            .field("iter", &self.iter)
            .finish()
    }
}
