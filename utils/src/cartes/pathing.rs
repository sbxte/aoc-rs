pub mod dijkstra;

/// Represents a cell which is part of a [Grid] that are able to be passed to
/// pathing algorithms in this crate (e.g. [dijkstra])
pub trait Pathable {
    /// Returns true when pathing algorithms are allowed to pass through this cell
    fn can_pass(&self) -> bool;
}

/// Type used for storing cost values
type Cost = u32;

/// For use in algorithms which do not path in-place, i.e. do not rely on the given [Grid] to store
/// pathing states.
#[derive(PartialEq, Eq)]
struct CellRef<P>
where
    P: PartialEq + Eq,
{
    pos: P,
    from: P,
    cost: Cost,
}

impl<P> CellRef<P>
where
    P: PartialEq + Eq + Copy,
{
    /// Constructs from a [pos], initialized with zero cost
    fn from_pos(pos: P) -> Self {
        Self {
            cost: 0,
            pos,
            from: pos,
        }
    }
}
impl<P> PartialOrd for CellRef<P>
where
    P: PartialEq + Eq,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<P> Ord for CellRef<P>
where
    P: PartialEq + Eq,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost)
    }
}

#[derive(PartialEq, Eq, Clone)]
/// Output type of pathing algorithms.
/// Contains the number of steps (path length) and an [Iterator] containing cell positions from
/// start to end
pub struct Path<I, P>
where
    I: Iterator<Item = P>,
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
