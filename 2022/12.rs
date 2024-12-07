use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;

fn day12() {
    let input = include_str!("../foo.txt");

    let mut grid = Grid::init(input);
    let mut step = 0;
    let mut path_cell;
    loop {
        step += 1;
        if step % 2 == 0{
            ::std::thread::sleep(::std::time::Duration::from_millis(50));
            grid.display();
        }
        let cell = grid.step_pathfinder();
        if cell.is_some() {
            path_cell = cell.unwrap();
            break;
        }
    }

    // Traverse backwards, find height 0 (a)
    let mut cstep = 0;
    loop {
        {
            path_cell.borrow_mut().path = true;
        }
        {
            let borrow = path_cell.borrow();
            if borrow.height == 0 || borrow.prev.is_none() { break; }
        }
        cstep += 1;
        let prev = path_cell.borrow_mut().prev.clone();
        path_cell = prev.unwrap();

        if cstep % 30 == 0 {
            ::std::thread::sleep(::std::time::Duration::from_millis(100));
            grid.display();
        }
    }

    grid.display();
    println!("\n\nSteps: {step} | Csteps: {cstep}");
}

struct Cell {
    prev: Option<Rc<RefCell<Cell>>>,
    height: u32,
    opened: bool,
    closed: bool,
    end: bool,
    path: bool,
    x: i32,
    y: i32,
}

impl Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}",
            if self.path {'#'}
            else if self.opened {'+'}
            else if self.closed {' '}
            else {(self.height + 97) as u8 as char}
        )
    }
}

struct Grid {
    grid: Vec<Rc<RefCell<Cell>>>,
    open: Vec<Rc<RefCell<Cell>>>,
    rows: i32,
    cols: i32,
}

impl Grid {
    fn init(input: &str) -> Self {
        let lines: Vec<_> = input.split('\n').collect(); // Remove new lines from this
        let cols = lines[0].len();
        let rows = lines.len();

        let mut grid = Grid {
            grid: Vec::with_capacity(cols * rows),
            open:Vec::with_capacity(1),
            rows: rows as i32,
            cols: cols as i32,
        };

        let (mut x, mut y) = (0, 0);
        for ch in input.chars() {
            if ch == '\n' {
                x = 0;
                y += 1;
                continue;
            }
            grid.grid.push(Rc::new(RefCell::new(
                Cell {
                    prev: None,
                    height: 0,
                    opened: false,
                    closed: false,
                    end: false,
                    path: false,
                    x, y
                }
            )));
            let node = &grid.grid.last().unwrap();
            if ch == 'S' {
                grid.open.push((*node).clone());
                node.borrow_mut().opened = true;
            } else if ch == 'E' {
                node.borrow_mut().end = true;
                node.borrow_mut().height = ('z' as u32) - 97;
            } else {
                node.borrow_mut().height = (ch as u32) - 97;
            }
            x += 1;
        }
        grid
    }

    fn display(&self) {
        let mut col = 0;
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        for node in &self.grid {
            if col >= self.cols {
                col = 0;
                print!("\n");
            }
            print!("{}", node.borrow());
            col += 1;
        }
    }

    fn get_node(&self, x: i32, y: i32) -> Rc<RefCell<Cell>> {
        // println!("{x}, {y}");
        self.grid[(y * self.cols + x) as usize].clone()
    }

    fn step_pathfinder(&mut self) -> Option<Rc<RefCell<Cell>>> { // returns end cell
        let mut new_opens = vec![];
        for node in &self.open {
            {
                node.borrow_mut().closed = true; // Mark current as closed
                node.borrow_mut().opened = false; // Mark current as closed
            }

            { // Mark neighbouring unclosed nodes as opened (add to list)
                let node_borrow = node.borrow();
                let (x, y) = (node_borrow.x, node_borrow.y);

                // Up down left right
                if (1..self.rows).contains(&y) {
                    let neighbour = self.get_node(x, y - 1);
                    let mut neighbour_borrow = neighbour.borrow_mut();
                    if (0..=(node_borrow.height + 1)).contains(&neighbour_borrow.height) {
                        if !neighbour_borrow.closed && !neighbour_borrow.opened{
                            neighbour_borrow.opened = true;
                            neighbour_borrow.prev = Some(node.clone());
                            if neighbour_borrow.end { return Some(neighbour.clone()); }
                            new_opens.push(neighbour.clone());
                        }
                    }
                }
                if (0..self.rows-1).contains(&y) {
                    let neighbour = self.get_node(x, y + 1);
                    let mut neighbour_borrow = neighbour.borrow_mut();
                    if (0..=(node_borrow.height + 1)).contains(&neighbour_borrow.height) {
                        if !neighbour_borrow.closed && !neighbour_borrow.opened{
                            neighbour_borrow.opened = true;
                            neighbour_borrow.prev = Some(node.clone());
                            if neighbour_borrow.end { return Some(neighbour.clone()); }
                            new_opens.push(neighbour.clone());
                        }
                    }
                }
                if (1..self.cols).contains(&x) {
                    let neighbour = self.get_node(x - 1, y);
                    let mut neighbour_borrow = neighbour.borrow_mut();
                    if (0..=(node_borrow.height + 1)).contains(&neighbour_borrow.height) {
                        if !neighbour_borrow.closed && !neighbour_borrow.opened{
                            neighbour_borrow.opened = true;
                            neighbour_borrow.prev = Some(node.clone());
                            if neighbour_borrow.end { return Some(neighbour.clone()); }
                            new_opens.push(neighbour.clone());
                        }
                    }
                }
                if (0..self.cols-1).contains(&x) {
                    let neighbour = self.get_node(x + 1, y);
                    let mut neighbour_borrow = neighbour.borrow_mut();
                    if (0..=(node_borrow.height + 1)).contains(&neighbour_borrow.height) {
                        if !neighbour_borrow.closed && !neighbour_borrow.opened{
                            neighbour_borrow.opened = true;
                            neighbour_borrow.prev = Some(node.clone());
                            if neighbour_borrow.end { return Some(neighbour.clone()); }
                            new_opens.push(neighbour.clone());
                        }
                    }
                }
            }
        }
        self.open.clear();
        self.open.append(&mut new_opens); // add new opened cells

        None
    }
}