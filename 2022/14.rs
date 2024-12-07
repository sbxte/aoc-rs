mod aoc {
    use std::fmt::Display;

    pub fn run() {
        // Advent of code
        println!("--- My solution ---");
        day14();
        println!("--- Correct solution ---");
    }

    fn day14() {
        let input = include_str!("../foo.txt");

        let (mut lx, mut hx, mut hy) = (500, 500, 0);
        let mut polylines: Polylines = Default::default();
        for line in input.lines() {
            let polyline;
            (polyline, lx, hx, hy) = Polyline::parse(line, lx, hx, hy);
            polylines.add(polyline);
        }

        let mut grid = Grid::new(hx - lx + 1, hy + 1, lx, hx, hy);
        grid.fill(polylines);

        // grid.display();

        let count = grid.simulate();
        println!("{count}");
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Cell {
        Air,
        Wall,
        Sand
    }

    impl Display for Cell {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f, "{}", match self {
                    Cell::Air => " ",
                    Cell::Wall => "#",
                    Cell::Sand => "o"
                }
            )
        }
    }

    impl Default for Cell {
        fn default() -> Self {
            Cell::Air
        }
    }

    #[derive(Debug, PartialEq, Eq, Default)]
    struct Grid {
        data: Vec<Vec<Cell>>,
        lx: i32, hx: i32, hy: i32
    }

    impl Grid {
        fn new(width: i32, height: i32, lx: i32, hx: i32, hy: i32) -> Self {
            Self {
                data: vec![vec![Default::default(); height as usize]; width as usize],
                lx, hx, hy
            }
        }

        fn fill(&mut self, polylines: Polylines) {
            let slx = self.lx;
            for polyline in polylines.lines {
                let mut points = polyline.points.iter();
                let mut prev = points.next().unwrap();
                for point in points {
                    let hx = point.x.max(prev.x);
                    let lx = point.x.min(prev.x);
                    let hy = point.y.max(prev.y);
                    let ly = point.y.min(prev.y);
                    for x in lx..=hx {
                        for y in ly..=hy {
                            *self.data
                                .get_mut((x - slx) as usize).unwrap()
                                .get_mut(y as usize).unwrap() = Cell::Wall;
                        }
                    }

                    prev = point;
                }
            }
        }

        fn display(&self) {
            for col in &self.data {
                for cell in col {
                    print!("{cell}");
                }
                println!();
            }
            println!();
        }

        fn get(&self, x: i32, y: i32) -> &Cell {
            &self.data[(x - self.lx) as usize][y as usize]
        }

        fn get_mut(&mut self, x: i32, y: i32) -> &mut Cell {
            &mut self.data[(x - self.lx) as usize][y as usize]
        }

        fn simulate(&mut self) -> i32 {
            let (mut sx, mut sy) = (500, 0);
            let mut counter = 0;
            loop {
                if sx == self.lx || sx == self.hx || sy == self.hy {
                    self.display();
                    break;
                }

                match self.get(sx, sy + 1) {
                    Cell::Air => {
                        sy += 1;
                        continue;
                    },
                    _ => { },
                }
                match self.get(sx - 1, sy + 1) {
                    Cell::Air => {
                        sx -= 1;
                        sy += 1;
                        continue;
                    },
                    _ => { },
                }
                match self.get(sx + 1, sy + 1) {
                    Cell::Air => {
                        sx += 1;
                        sy += 1;
                        continue;
                    },
                    _ => { },
                }

                counter += 1;
                *self.get_mut(sx, sy) = Cell::Sand;
                (sx, sy) = (500, 0);
                // self.display();
            }
            counter
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Point {
        fn parse(s: &str) -> Point {
            let mut tokens = s.split(',');
            let (x, y) = (tokens.next().unwrap(), tokens.next().unwrap());
            Self {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        }
    }

    #[derive(Debug, Default)]
    struct Polyline {
        points: Vec<Point>,
    }

    impl Polyline {
        fn parse(s: &str, mut lx: i32, mut hx: i32, mut hy: i32) -> (Self, i32, i32, i32) {
            (Self {
                points: s.split(" -> ").map(|s| {
                    let point = Point::parse(s);
                    lx = lx.min(point.x);
                    hx = hx.max(point.x);
                    hy = hy.max(point.y);
                    point
                }).collect()
            }, lx, hx, hy)
        }
    }

    #[derive(Debug, Default)]
    struct Polylines {
        lines: Vec<Polyline>
    }

    impl Polylines {
        fn add(&mut self, polyline: Polyline) {
            self.lines.push(polyline);
        }
    }
}

