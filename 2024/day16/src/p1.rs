use aocutils::cartes::dim2::grid::Grid2;
use aocutils::cartes::dim2::vec::Vec2;
use aocutils::noreach;
use aocutils::num::SignedType;

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
    Untouched,
    Visited(Cost, Vec2<isize>),
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
struct Cost {
    h: i32,
    g: i32,
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

fn parse_input(input: &str) -> (Grid2<Cell>, Vec2<isize>, Vec2<isize>) {
    let mut grid = Grid2::from_str_2(input, Cell::from_byte);
    let mut start = Vec2::default();
    let mut end = Vec2::default();
    let cols = grid.cols;
    grid.as_mut()
        .iter_mut()
        .enumerate()
        .for_each(|(i, c)| match c.content {
            Content::Start => {
                start = Grid2::idx_to_vec2(i, cols).map(|x| x.to_signtype());
                c.content = Content::Empty;
            }
            Content::End => {
                end = Grid2::idx_to_vec2(i, cols).map(|x| x.to_signtype());
                c.content = Content::Empty;
            }
            _ => {}
        });
    (grid, start, end)
}

fn astar(grid: &mut Grid2<Cell>, start: Vec2<isize>, end: Vec2<isize>) {
    let mut open = vec![start.clone()];

    fn check(grid: &mut Grid2<Cell>, v: Vec2<isize>) {
        let mut cell = grid.get_v(v.map(|x| x.to_signtype()));
    }
    while let Some(opened) = open.pop() {}
}

pub fn part1(input: &str) -> i32 {
    let (mut grid, start, end) = parse_input(input);
    dbg!(grid.cols, grid.rows, start, end);

    astar(&mut grid, start, end);

    0
}
