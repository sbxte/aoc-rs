use aocutils::cartes::dim2::grid::Grid2;
use aocutils::cartes::grid::Grid as _;

#[derive(Eq, PartialEq, Debug)]
enum Cell {
    PaperRoll,
    Empty,
}

fn from_byte(b: u8) -> Option<Cell> {
    match b {
        b'@' => Some(Cell::PaperRoll),
        b'.' => Some(Cell::Empty),
        x => unreachable!("Invalid character found in input: {}", x as char),
    }
}

pub fn part2(input: &str) -> u32 {
    let mut grid = Grid2::from_str_2(input, from_byte);
    let mut sum = 0;
    let mut mark_removal = vec![];
    loop {
        for (pos, cell) in grid.iter() {
            let count = grid.get_neighbours_full(pos).fold(0, |acc, cell| {
                if matches!(cell, Cell::PaperRoll) {
                    acc + 1
                } else {
                    acc
                }
            });
            if count < 4 && matches!(cell, Cell::PaperRoll) {
                mark_removal.push(pos);
                sum += 1;
            }
        }

        if mark_removal.is_empty() {
            break;
        }

        for pos in &mark_removal {
            grid[*pos] = Cell::Empty;
        }
        mark_removal.clear();
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        assert_eq!(part2(input), 43);
    }
}
