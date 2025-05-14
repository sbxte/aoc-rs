use std::collections::BinaryHeap;

use aocutils::cartes::dim2::dir::Direction;
use aocutils::cartes::dim2::grid::Grid2;
use aocutils::cartes::dim2::vec::Vec2;
use aocutils::noreach;

fn parse_input(input: &str) -> (Grid2<Cell>, Vec2<usize>, Vec2<usize>) {
    let mut grid = Grid2::from_str_2(input, Cell::from_byte);
    let mut start = Vec2::default();
    let mut end = Vec2::default();
    let cols = grid.cols;
    grid.as_mut()
        .iter_mut()
        .enumerate()
        .for_each(|(i, c)| match c.content {
            Content::Start => {
                start = Grid2::idx_to_vec2(i, cols);
                c.content = Content::Empty;
            }
            Content::End => {
                end = Grid2::idx_to_vec2(i, cols);
                c.content = Content::Empty;
            }
            _ => {}
        });
    (grid, start, end)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
enum Content {
    #[default]
    Empty,
    Wall,
    Start,
    End,
}

impl Content {
    fn from_byte(b: u8) -> Option<Self> {
        use self::Content::*;
        match b {
            b'#' => Some(Wall),
            b'.' => Some(Empty),
            b'S' => Some(Start),
            b'E' => Some(End),
            x => noreach!("From byte no reach: {}", x),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
enum PathState {
    #[default]
    Free,
    Visited(DirScore),
    Backtracked,
}

/// One for each direction
/// Up, down, left, right
#[derive(Debug, PartialEq, Eq, Clone)]
struct DirScore(u32, u32, u32, u32);

impl DirScore {
    fn new() -> Self {
        Self(u32::MAX, u32::MAX, u32::MAX, u32::MAX)
    }
    fn get(&self, dir: Direction) -> u32 {
        use Direction::*;
        match dir {
            Up => self.0,
            Down => self.1,
            Left => self.2,
            Right => self.3,
        }
    }

    fn get_mut(&mut self, dir: Direction) -> &mut u32 {
        use Direction::*;
        match dir {
            Up => &mut self.0,
            Down => &mut self.1,
            Left => &mut self.2,
            Right => &mut self.3,
        }
    }

    fn min_score_dir(&self) -> Direction {
        let min = self.0.min(self.1).min(self.2).min(self.3);
        use Direction::*;
        if min == self.0 {
            Up
        } else if min == self.1 {
            Down
        } else if min == self.2 {
            Left
        } else {
            Right
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Cell {
    content: Content,
    state: PathState,
}

impl Cell {
    fn from_byte(b: u8) -> Option<Self> {
        Content::from_byte(b).map(|content| Self {
            content,
            state: Default::default(),
        })
    }
}
impl ::std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Content::*;
        use Direction::*;
        use PathState::*;
        match self.content {
            Wall => write!(f, "#"),
            _ => match &self.state {
                Free => write!(f, "."),
                Visited(ds) => match ds.min_score_dir() {
                    Up => write!(f, "^"),
                    Down => write!(f, "v"),
                    Left => write!(f, "<"),
                    Right => write!(f, ">"),
                },
                Backtracked => write!(f, "O"),
            },
        }
    }
}

#[derive(PartialEq, Eq)]
struct CellRef {
    pos: Vec2<usize>,
    score: u32,
    dir: Direction,
}

impl PartialOrd for CellRef {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CellRef {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

#[derive(PartialEq, Eq)]
struct CellRefRev {
    cell: CellRef,
}
impl PartialOrd for CellRefRev {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CellRefRev {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cell.cmp(&other.cell).reverse()
    }
}
impl ::std::ops::Deref for CellRefRev {
    type Target = CellRef;

    fn deref(&self) -> &Self::Target {
        &self.cell
    }
}

fn add(
    grid: &mut Grid2<Cell>,
    open: &mut BinaryHeap<CellRef>,
    pos: Vec2<usize>,
    score: u32,
    dir: Direction,
) {
    if Content::Wall == grid.get_v(pos).content {
        return;
    }
    if PathState::Free == grid.get_v(pos).state {
        let mut ds = DirScore::new();
        *ds.get_mut(dir) = score;
        grid.get_v_mut(pos).state = PathState::Visited(ds);
        open.push(CellRef { pos, score, dir });
    } else if let PathState::Visited(ds) = &mut grid.get_v_mut(pos).state {
        if ds.get(dir) <= score {
            return;
        }
        *ds.get_mut(dir) = score;
        open.push(CellRef { pos, score, dir });
    }
}

fn back_add(
    grid: &mut Grid2<Cell>,
    open: &mut BinaryHeap<CellRefRev>,
    pos: Vec2<usize>,
    score: u32,
    dir: Direction,
) {
    if Content::Wall == grid.get_v(pos).content {
        return;
    }
    if let PathState::Visited(ds) = &grid.get_v(pos).state
        && ds.get(dir) == score
    {
        open.push(CellRefRev {
            cell: CellRef { pos, score, dir },
        });
    }
}

fn search(grid: &mut Grid2<Cell>, start: Vec2<usize>, end: Vec2<usize>) -> u32 {
    let mut open = BinaryHeap::new();
    // Problem states Reindeer starts facing EAST
    // EAST = RIGHT
    add(grid, &mut open, start, 0, Direction::Right);

    let mut score = 0;
    let mut dir = Direction::Right;
    while let Some(opened) = open.pop() {
        if opened.pos == end {
            score = opened.score;
            dir = opened.dir;
            break;
        }
        add(
            grid,
            &mut open,
            (Vec2::<isize>::from(opened.pos) + opened.dir.step()).into(),
            opened.score + 1,
            opened.dir,
        );
        add(
            grid,
            &mut open,
            (Vec2::<isize>::from(opened.pos) + opened.dir.rot90().step()).into(),
            opened.score + 1001,
            opened.dir.rot90(),
        );
        add(
            grid,
            &mut open,
            (Vec2::<isize>::from(opened.pos) + opened.dir.rot270().step()).into(),
            opened.score + 1001,
            opened.dir.rot270(),
        );
    }

    let mut open = BinaryHeap::new();
    back_add(grid, &mut open, end, score, dir);

    while let Some(opened) = open.pop() {
        grid.get_v_mut(opened.pos).state = PathState::Backtracked;
        if opened.pos == start {
            break;
        }

        back_add(
            grid,
            &mut open,
            (Vec2::<isize>::from(opened.pos) - opened.dir.step()).into(),
            opened.score.saturating_sub(1),
            opened.dir,
        );
        back_add(
            grid,
            &mut open,
            (Vec2::<isize>::from(opened.pos) - opened.dir.step()).into(),
            opened.score.saturating_sub(1001),
            opened.dir.rot90(),
        );
        back_add(
            grid,
            &mut open,
            (Vec2::<isize>::from(opened.pos) - opened.dir.step()).into(),
            opened.score.saturating_sub(1001),
            opened.dir.rot270(),
        );

        back_add(
            grid,
            &mut open,
            (Vec2::<isize>::from(opened.pos) - opened.dir.rot90().step()).into(),
            opened.score.saturating_sub(1),
            opened.dir,
        );
        back_add(
            grid,
            &mut open,
            (Vec2::<isize>::from(opened.pos) - opened.dir.rot90().step()).into(),
            opened.score.saturating_sub(1001),
            opened.dir.rot90(),
        );
        back_add(
            grid,
            &mut open,
            (Vec2::<isize>::from(opened.pos) - opened.dir.rot90().step()).into(),
            opened.score.saturating_sub(1001),
            opened.dir.rot270(),
        );

        back_add(
            grid,
            &mut open,
            (Vec2::<isize>::from(opened.pos) - opened.dir.rot270().step()).into(),
            opened.score.saturating_sub(1),
            opened.dir,
        );
        back_add(
            grid,
            &mut open,
            (Vec2::<isize>::from(opened.pos) - opened.dir.rot270().step()).into(),
            opened.score.saturating_sub(1001),
            opened.dir.rot90(),
        );
        back_add(
            grid,
            &mut open,
            (Vec2::<isize>::from(opened.pos) - opened.dir.rot270().step()).into(),
            opened.score.saturating_sub(1001),
            opened.dir.rot270(),
        );
    }

    grid.iter()
        .filter(|(_, cell)| cell.state == PathState::Backtracked)
        .count() as u32
}

pub fn part2(input: &str) -> u32 {
    let (mut grid, start, end) = parse_input(input);

    search(&mut grid, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_sample() {
        let input = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
        assert_eq!(part2(input), 45);
    }

    #[test]
    fn large_sample() {
        let input = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";
        assert_eq!(part2(input), 64);
    }
}
