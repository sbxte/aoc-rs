use aocutils::cartes::dim2::grid::Grid2;
use aocutils::cartes::dim2::vec::Vec2;

#[derive(Debug, Eq, PartialEq)]
enum Cell {
    Empty,
    Mul,
    Add,
    Num(u8)
}
impl Cell {
    fn is_empty(&self) -> bool {
        matches!(self, Cell::Empty)
    }
}

fn from_byte(byte: u8) -> Cell {
    match byte {
        b'*' => Cell::Mul,
        b'+' => Cell::Add,
        x if x.is_ascii_whitespace() => Cell::Empty,
        x => Cell::Num(x - b'0'),
    }
}

fn shift(acc: u64, cell: &Cell) -> u64 {
    if let Cell::Num(n) = cell && *n != 0 {
        acc * 10 + *n as u64
    } else {
        acc
    }
}

pub fn part2(input: &str) -> u64 {
    let grid = Grid2::from_str_1(input, from_byte);

    let mut outer_accu = 0;
    let mut inner_accu = 0;

    let mut op_mul = false;
    for i in 0..(grid.cols as isize) {
        let first = &grid[Vec2(i, 0)];
        let second = &grid[Vec2(i, 1)];
        let third = &grid[Vec2(i, 2)];
        let fourth = &grid[Vec2(i, 3)];
        let op_i = &grid[Vec2(i, 4)];

        if first.is_empty() && second.is_empty() && third.is_empty() && fourth.is_empty() {
            outer_accu += inner_accu;
            inner_accu = 0;
            continue;
        }

        let n = 0;
        let n = shift(n, first);
        let n = shift(n, second);
        let n = shift(n, third);
        let n = shift(n, fourth);

        if !op_i.is_empty() {
            op_mul = matches!(op_i, Cell::Mul);
            inner_accu = n;
            continue;
        }

        if op_mul {
            inner_accu *= n;
        } else {
            inner_accu += n;
        }
    }

    outer_accu + inner_accu
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "000 000 000 000
123 328 051 640
045 640 387 230
006 980 215 314
*   +   *   +  ";
        assert_eq!(part2(input), 3263827);
    }
}
