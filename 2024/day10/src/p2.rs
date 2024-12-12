pub mod naive {
    pub fn part2(input: &str) -> u32 {
        type Height = u8;

        let (cols, rows, grid, mut open) = {
            let c = input.find('\n').unwrap();
            let c1 = c + 1;
            let r = input.lines().count();
            let mut grid = vec![];
            let mut o = vec![];
            for (i, b) in input.as_bytes().iter().enumerate() {
                if b.is_ascii_digit() {
                    grid.push(b - b'0');
                } else if b == &b'.' {
                    grid.push(Height::MAX - 1);
                }
                if b == &b'9' {
                    o.push((i % c1, i / c1))
                }
            }
            (c, r, grid, o)
        };

        #[inline]
        fn hh(grid: &[Height], pos: (usize, usize), cols: usize) -> &Height {
            &grid[pos.1 * cols + pos.0]
        }

        let mut sum = 0;
        while let Some(pos) = open.pop() {
            let height = *hh(&grid, pos, cols);
            if height == 0 {
                sum += 1;
                continue;
            }

            if pos.0 > 0 && hh(&grid, (pos.0 - 1, pos.1), cols) + 1 == height {
                open.push((pos.0 - 1, pos.1));
            }
            if pos.1 > 0 && hh(&grid, (pos.0, pos.1 - 1), cols) + 1 == height {
                open.push((pos.0, pos.1 - 1));
            }
            if pos.0 < cols - 1 && hh(&grid, (pos.0 + 1, pos.1), cols) + 1 == height {
                open.push((pos.0 + 1, pos.1))
            }
            if pos.1 < rows - 1 && hh(&grid, (pos.0, pos.1 + 1), cols) + 1 == height {
                open.push((pos.0, pos.1 + 1));
            }
        }

        sum
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::*;

        #[test]
        fn sample() {
            assert_eq!(part2(SAMPLE), 81);
        }

        #[test]
        fn sample3() {
            let input = "..90..9
...1.98
...2..7
6543456
765.987
876....
987....";
            assert_eq!(part2(input), 13);
        }
    }
}

pub mod optim {
    #[inline]
    fn hh(grid: &[u8], pos: (usize, usize), cols: usize) -> u8 {
        grid[pos.1 * cols + pos.0] - b'0'
    }

    pub fn part2(input: &str) -> u32 {
        let (cols, rows, bytes, mut open) = {
            let c = input.find('\n').unwrap();
            let c1 = c + 1;
            let r = input.lines().count();
            let mut o = vec![];
            for (i, b) in input.as_bytes().iter().enumerate() {
                if b == &b'9' {
                    o.push((i % c1, i / c1))
                }
            }
            (c, r, input.as_bytes(), o)
        };

        let mut sum = 0;
        let c1 = cols + 1;
        while let Some(pos) = open.pop() {
            let height = hh(bytes, pos, c1);
            if height == 0 {
                sum += 1;
                continue;
            }

            if pos.0 > 0 && hh(bytes, (pos.0 - 1, pos.1), c1) + 1 == height {
                open.push((pos.0 - 1, pos.1));
            }
            if pos.1 > 0 && hh(bytes, (pos.0, pos.1 - 1), c1) + 1 == height {
                open.push((pos.0, pos.1 - 1));
            }
            if pos.0 < cols - 1 && hh(bytes, (pos.0 + 1, pos.1), c1) + 1 == height {
                open.push((pos.0 + 1, pos.1))
            }
            if pos.1 < rows - 1 && hh(bytes, (pos.0, pos.1 + 1), c1) + 1 == height {
                open.push((pos.0, pos.1 + 1));
            }
        }

        sum
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::*;

        #[test]
        fn indexing() {
            let input = "0123
3210
0123
3210";
            let grid = input.as_bytes();
            let cols = input.find('\n').unwrap() + 1;
            assert_eq!(hh(grid, (0, 0), cols), 0);
            assert_eq!(hh(grid, (1, 0), cols), 1);
            assert_eq!(hh(grid, (2, 0), cols), 2);
            assert_eq!(hh(grid, (0, 1), cols), 3);
            assert_eq!(hh(grid, (0, 2), cols), 0);
            assert_eq!(hh(grid, (2, 2), cols), 2);
        }

        #[test]
        fn sample() {
            assert_eq!(part2(SAMPLE), 81);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optim_naive() {
        let input = include_str!("input.txt");
        assert_eq!(naive::part2(input), optim::part2(input));
    }
}
