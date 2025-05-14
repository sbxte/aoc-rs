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

#[derive(Debug, PartialEq, Eq, Clone, Default)]
enum PathState {
    #[default]
    Free,
    Visited,
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

fn dijkstra(grid: &mut Grid2<Cell>, start: Vec2<usize>, end: Vec2<usize>) -> u32 {
    fn add(
        grid: &mut Grid2<Cell>,
        open: &mut BinaryHeap<CellRef>,
        pos: Vec2<usize>,
        score: u32,
        dir: Direction,
    ) {
        if Content::Empty == grid.get_v(pos).content && PathState::Free == grid.get_v(pos).state {
            grid.get_v_mut(pos).state = PathState::Visited;
            open.push(CellRef { pos, score, dir });
        }
    }

    let mut open = BinaryHeap::new();
    // Problem states Reindeer starts facing EAST
    // EAST = RIGHT
    add(grid, &mut open, start, 0, Direction::Right);

    while let Some(opened) = open.pop() {
        if opened.pos == end {
            return opened.score;
        }
        //  dbg!((opened.pos, opened.dir, opened.score));
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

    panic!("Could not find path from start to end");
}

pub fn part1(input: &str) -> u32 {
    let (mut grid, start, end) = parse_input(input);
    // dbg!(grid.cols, grid.rows, start, end);

    dijkstra(&mut grid, start, end)
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
        assert_eq!(part1(input), 7036);
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
        assert_eq!(part1(input), 11048);
    }
}
