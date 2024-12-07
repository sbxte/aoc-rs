
fn day17() {
    let input = include_str!("../foo.txt");
    let jets = parse(input);
    // let max_turns = BigUint::parse_bytes(b"1_000_000_000", 10).unwrap();
    // let max_turns = BigUint::parse_bytes(b"2022", 10).unwrap();
    // let height = Chamber::new(jets).simulate(2022, 1);
    let height = Chamber::new(jets).simulate(1_000_000, 2);
    println!("Height is {}", height);
}

fn parse(i: &str) -> Vec<Jet> {
    let mut v = vec![];
    for c in i.trim().chars() {
        v.push(match c {
            '>' => Jet::Right,
            '<' => Jet::Left,
            x => panic!("Input has something other than '<' or '>'! {}", x),
        });
    }
    v
}

#[derive(Clone, Copy)]
enum Jet {
    Left,
    Right,
}

impl fmt::Debug for Jet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Jet::Left => 'L',
                Jet::Right => 'R',
            }
        )
    }
}

impl fmt::Display for Jet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
enum Rock {
    #[default]
    Horizontal,
    Plus,
    Corner,
    Vertical,
    Square,
}

impl Rock {
    fn next(self) -> Self {
        use Rock::*;

        match self {
            Horizontal => Plus,
            Plus => Corner,
            Corner => Vertical,
            Vertical => Square,
            Square => Horizontal,
        }
    }

    fn can_fall(&self, chamber: &Chamber, x: u64, y: u64) -> bool {
        use Rock::*;

        if y == 1 {
            return false;
        }

        let get = |x: u64, y: u64| chamber.get(x, y);
        !match self {
            Horizontal => {
                get(x, y - 1) || get(x + 1, y - 1) || get(x + 2, y - 1) || get(x + 3, y - 1)
            }
            Plus => get(x, y) || get(x + 1, y - 1) || get(x + 2, y),
            Corner => get(x, y - 1) || get(x + 1, y - 1) || get(x + 2, y - 1),
            Vertical => get(x, y - 1),
            Square => get(x, y - 1) || get(x + 1, y - 1),
        }
    }

    fn can_move_left(&self, chamber: &Chamber, x: u64, y: u64) -> bool {
        use Rock::*;

        if x == 1 {
            return false;
        }

        let get = |x: u64, y: u64| chamber.get(x, y);
        !match self {
            Horizontal => get(x - 1, y),
            Plus => get(x, y) || get(x - 1, y + 1) || get(x, y + 2),
            Corner => get(x - 1, y) || get(x + 1, y + 1) || get(x + 1, y + 2),
            Vertical => {
                get(x - 1, y) || get(x - 1, y + 1) || get(x - 1, y + 2) || get(x - 1, y + 3)
            }
            Square => get(x - 1, y) || get(x - 1, y + 1),
        }
    }

    fn can_move_right(&self, chamber: &Chamber, x: u64, y: u64) -> bool {
        use Rock::*;

        let get = |x: u64, y: u64| chamber.get(x, y);
        !match self {
            Horizontal => get(x + 4, y),
            Plus => get(x + 2, y) || get(x + 3, y + 1) || get(x + 2, y + 2),
            Corner => get(x + 3, y) || get(x + 3, y + 1) || get(x + 3, y + 2),
            Vertical => {
                get(x + 1, y) || get(x + 1, y + 1) || get(x + 1, y + 2) || get(x + 1, y + 3)
            }
            Square => get(x + 2, y) || get(x + 2, y + 1),
        }
    }

    fn get_height(&self) -> u64 {
        use Rock::*;

        match self {
            Horizontal => 0,
            Plus => 2,
            Corner => 2,
            Vertical => 3,
            Square => 1,
        }
    }

    fn lock(&self, chamber: &mut Chamber, x: u64, y: u64) {
        use Rock::*;
        let mut set = |x: u64, y: u64| {
            chamber.set(x, y);
        };
        match self {
            Horizontal => {
                set(x, y);
                set(x + 1, y);
                set(x + 2, y);
                set(x + 3, y);
            }
            Plus => {
                set(x + 1, y);
                set(x, y + 1);
                set(x + 1, y + 1);
                set(x + 2, y + 1);
                set(x + 1, y + 2);
            }
            Corner => {
                set(x, y);
                set(x + 1, y);
                set(x + 2, y);
                set(x + 2, y + 1);
                set(x + 2, y + 2);
            }
            Vertical => {
                set(x, y);
                set(x, y + 1);
                set(x, y + 2);
                set(x, y + 3);
            }
            Square => {
                set(x, y);
                set(x + 1, y);
                set(x, y + 1);
                set(x + 1, y + 1);
            }
        }
    }
}

#[derive(Clone)]
struct Chamber {
    jets: Vec<Jet>,
    rocks: Vec<[bool; 7]>,
    offset: u64,
    highest: u64,
    turn: u64,
    current_rock: Rock,
}

impl Chamber {
    fn new(jets: Vec<Jet>) -> Self {
        Self {
            jets,
            rocks: vec![[false; 7]; 5],
            offset: 0,
            highest: 0,
            turn: 0,
            current_rock: Default::default(),
        }
    }

    fn get(&self, x: u64, y: u64) -> bool {
        if y == 0 {
            return true;
        }
        if x == 0 || x == 8 {
            return true;
        }
        self.rocks[y as usize - 1][x as usize - 1]
    }

    fn set(&mut self, x: u64, y: u64) {
        self.highest = self.highest.max(y);
        let diff = self.rocks.len() - self.highest as usize;
        if diff < 9 {
            for _ in 0..diff {
                self.rocks.push([false; 7]);
            }
        }
        self.rocks[y as usize - 1][x as usize - 1] = true;
    }

    fn get_jet(&self, jet: usize) -> Jet {
        let len = self.jets.len();
        self.jets[jet % len]
    }

    fn clean_chamber(&mut self) {
        let threshold = 100_000;
        let remainder = 200;
        let remove = threshold - remainder;
        if self.rocks.len() > threshold {
            self.rocks.drain(0..remove);
            self.highest -= remove as u64;
            self.offset += remove as u64;
        }
    }

    fn simulate(mut self, max_turns: u64, mut times: u64) -> u64 {
        let mut jet_turn = 0;
        let mut turns_left = max_turns;
        while times > 0 || turns_left > 0 {
            self.clean_chamber();
            let (mut x, mut y) = (3, self.highest + 4);
            loop {
                use Jet::*;
                match self.get_jet(jet_turn) {
                    Left => {
                        if self.current_rock.can_move_left(&self, x, y) {
                            x -= 1;
                        }
                    }
                    Right => {
                        if self.current_rock.can_move_right(&self, x, y) {
                            x += 1;
                        }
                    }
                }
                jet_turn += 1;

                if self.current_rock.can_fall(&self, x, y) {
                    y -= 1;
                    continue;
                }
                self.current_rock.clone().lock(&mut self, x, y);
                self.current_rock = self.current_rock.next();

                if turns_left == 0 {
                    turns_left = max_turns;
                    times -= 1;
                } else {
                    turns_left -= 1;
                }
                break;
            }
        }
        // println!("{}", self);
        self.highest + self.offset
    }
}

impl fmt::Debug for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let disp = |d: bool| if d { '#' } else { '.' };
        for layer in self.rocks.iter().rev() {
            write!(
                f,
                "{}{}{}{}{}{}{}\n",
                disp(layer[0]),
                disp(layer[1]),
                disp(layer[2]),
                disp(layer[3]),
                disp(layer[4]),
                disp(layer[5]),
                disp(layer[6]),
            )?;
        }
        Ok(())
    }
}

impl fmt::Display for Chamber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
