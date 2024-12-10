// Naive solutions really need not be super optimized lmfao
// Or maybe I'm just lazy to think of a good impl rn...

pub mod naive {
    pub fn part1(input: &str) -> u32 {
        type AccessID = usize;
        type Height = u8;

        #[derive(Debug, PartialEq, Eq, Clone)]
        struct Cell {
            height: Height,
            acc_id: AccessID,
        }
        impl Cell {
            fn new(height: Height) -> Self {
                Self { height, acc_id: 0 }
            }
        }

        let (cols, rows, mut grid, mut open) = {
            let c = input.find('\n').unwrap();
            let c1 = c + 1;
            let r = input.lines().count();
            let mut grid = vec![];
            let mut o = vec![];
            for (i, b) in input.as_bytes().iter().enumerate() {
                if b.is_ascii_digit() {
                    grid.push(Cell::new(b - b'0'));
                } else if b == &b'.' {
                    grid.push(Cell::new(Height::MAX - 1));
                }
                if b == &b'9' {
                    o.push((i % c1, i / c1))
                }
            }
            (c, r, grid, o)
        };

        #[inline]
        fn hh(grid: &[Cell], pos: (usize, usize), cols: usize) -> &Cell {
            &grid[pos.1 * cols + pos.0]
        }

        #[inline]
        fn hhm(grid: &mut [Cell], pos: (usize, usize), cols: usize) -> &mut Cell {
            &mut grid[pos.1 * cols + pos.0]
        }

        let mut sum = 0;
        let mut acc_id = 0;
        while let Some(pos) = open.pop() {
            // dbg!(&open, pos);
            let height = hh(&grid, pos, cols).height;
            hhm(&mut grid, pos, cols).acc_id = acc_id;
            if height == 9 {
                acc_id += 1;
            } else if height == 0 {
                sum += 1;
                continue;
            }

            if pos.0 > 0
                && hh(&grid, (pos.0 - 1, pos.1), cols).height + 1 == height
                && hh(&grid, (pos.0 - 1, pos.1), cols).acc_id != acc_id
            {
                open.push((pos.0 - 1, pos.1));
            }
            if pos.1 > 0
                && hh(&grid, (pos.0, pos.1 - 1), cols).height + 1 == height
                && hh(&grid, (pos.0, pos.1 - 1), cols).acc_id != acc_id
            {
                open.push((pos.0, pos.1 - 1));
            }
            if pos.0 < cols - 1
                && hh(&grid, (pos.0 + 1, pos.1), cols).height + 1 == height
                && hh(&grid, (pos.0 + 1, pos.1), cols).acc_id != acc_id
            {
                open.push((pos.0 + 1, pos.1))
            }
            if pos.1 < rows - 1
                && hh(&grid, (pos.0, pos.1 + 1), cols).height + 1 == height
                && hh(&grid, (pos.0, pos.1 + 1), cols).acc_id != acc_id
            {
                open.push((pos.0, pos.1 + 1));
            }
        }

        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn sample() {
        assert_eq!(naive::part1(SAMPLE), 36);
    }

    #[test]
    fn sample1() {
        let input = "0123
1234
8765
9876";
        assert_eq!(naive::part1(input), 1);
    }

    #[test]
    fn sample2() {
        let input = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
        assert_eq!(naive::part1(input), 4);
    }
}
