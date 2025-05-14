pub mod naive {
    use aocutils::cartes::dim2::dir::Direction;
    use aocutils::cartes::dim2::grid::Grid2;
    use aocutils::cartes::dim2::vec::Vec2;
    use aocutils::optim::prelude::*;

    fn get_robot_pos(input: &str) -> Vec2<isize> {
        let cols = input.lines().next().unwrap().len();
        let c1 = cols + 1; // Include new line as a column
        let idx = input.find('@').unwrap();
        Grid2::idx_to_vec2(idx as isize, c1 as isize)
    }

    #[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
    enum Cell {
        #[default]
        Empty,
        Wall,
        Robot,
        Box,
    }

    impl Cell {
        fn from_byte(c: u8) -> Self {
            use self::Cell::*;
            match c {
                b'.' => Empty,
                b'#' => Wall,
                b'O' => Box,
                b'@' => Robot,
                b'\n' => Wall,
                x => noreach!("from byte {}", x),
            }
        }
    }

    impl std::fmt::Display for Cell {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(
                f,
                "{}",
                match self {
                    Self::Empty => '.',
                    Self::Wall => '#',
                    Self::Robot => '@',
                    Self::Box => 'O',
                }
            )
        }
    }

    fn parse_movements(input: &str) -> Vec<Direction> {
        input.bytes().filter_map(parse_move).collect()
    }

    fn parse_move(c: u8) -> Option<Direction> {
        use Direction::*;
        match c {
            b'<' => Some(Left),
            b'^' => Some(Up),
            b'>' => Some(Right),
            b'v' => Some(Down),
            b'\n' => None,
            x => noreach!("parse move {}", x),
        }
    }

    fn parse_input(input: &str) -> (Grid2<Cell>, Vec2<isize>, Vec<Direction>) {
        input
            .split_once("\n\n")
            .map(|(g, m)| (g.trim(), m.trim()))
            .map(|(g, m)| {
                (
                    Grid2::from_str_1(g, Cell::from_byte),
                    get_robot_pos(g),
                    parse_movements(m),
                )
            })
            .unwrap()
    }

    fn try_move_robot(grid: &mut Grid2<Cell>, robot: &mut Vec2<isize>, dir: Direction) {
        let step = dir.step();
        let mut can_move = false;
        let mut check = *robot; // copy

        while !check.is_oob_inclusive(
            Vec2::from((0, 0)),
            Vec2::from((grid.cols as isize - 1, grid.rows as isize)),
        ) {
            if let Cell::Empty = grid.get_v(check.map(|x| x as usize)) {
                can_move = true;
                break;
            } else if let Cell::Wall = grid.get_v(check.map(|x| x as usize)) {
                break;
            }
            check += step;
        }

        if !can_move {
            return;
        }

        let mut mv = check; // copy
        let rstep = dir.rot180().step();
        while mv != *robot {
            let next = mv + rstep;

            if cfg!(debug_assertions) {
                grid.swap_idx(
                    Grid2::vec2_to_idx(next.map(|x| x as usize), grid.cols),
                    Grid2::vec2_to_idx(mv.map(|x| x as usize), grid.cols),
                );
            } else {
                unsafe {
                    grid.swap_idx_unchecked(
                        Grid2::vec2_to_idx(next.map(|x| x as usize), grid.cols),
                        Grid2::vec2_to_idx(mv.map(|x| x as usize), grid.cols),
                    );
                }
            }

            mv += rstep;
        }
        *robot += dir.step();
    }

    fn calc_sum_gps(grid: &Grid2<Cell>) -> i32 {
        grid.as_slice().iter().enumerate().fold(0, |acc, (i, x)| {
            if let &Cell::Box = x {
                let v = Grid2::idx_to_vec2(i, grid.cols);
                acc + 100 * v.1 + v.0
            } else {
                acc
            }
        }) as i32
    }

    pub fn part1(input: &str) -> i32 {
        let (mut grid, mut robot, movements) = parse_input(input);
        for m in movements {
            if cfg!(any(feature = "vis1")) {
                grid.print_display();
            }
            try_move_robot(&mut grid, &mut robot, m);
        }
        if cfg!(any(feature = "vis1")) {
            grid.print_display();
        }
        calc_sum_gps(&grid)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::*;

        #[test]
        fn small_sample() {
            assert_eq!(part1(SMALL_SAMPLE), 2028);
        }

        #[test]
        fn large_sample() {
            assert_eq!(part1(LARGE_SAMPLE), 10092);
        }
    }
}
