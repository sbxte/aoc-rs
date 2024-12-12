pub mod naive {
    #[inline]
    fn hh<T: Copy>(grid: &[T], pos: (i32, i32), cols: usize) -> T {
        grid[pos.1 as usize * cols + pos.0 as usize]
    }

    #[inline]
    fn hhm<T>(grid: &mut [T], pos: (i32, i32), cols: usize) -> &mut T {
        &mut grid[pos.1 as usize * cols + pos.0 as usize]
    }

    #[inline]
    fn hhc<T: Copy + PartialEq>(
        grid: &[T],
        pos: (i32, i32),
        cols: usize,
        bcols: usize,
        brows: usize,
        comp: T,
        oob: bool,
    ) -> bool {
        if !(0..bcols as i32).contains(&pos.0) || !(0..brows as i32).contains(&pos.1) {
            return oob;
        }
        grid[pos.1 as usize * cols + pos.0 as usize] == comp
    }

    pub fn part2(input: &str) -> u32 {
        let (cols, rows, bytes) = {
            let c = input.find('\n').unwrap();
            let r = input.lines().count();
            (c, r, input.as_bytes())
        };

        let mut access_map = vec![false; cols * rows];
        let mut regions = vec![];
        regions.push((0, 0));
        let c1 = cols + 1;

        #[cfg(feature = "vis2")]
        print!("{}[2J", 27 as char);

        let mut open = vec![];
        let mut sum = 0;
        while let Some(reg_pos) = regions.pop() {
            let accessed = hh(&access_map, reg_pos, cols);
            if accessed {
                continue;
            }

            let cur_reg = hh(bytes, reg_pos, c1);
            open.push(reg_pos);

            let mut sides = 0;
            let mut area = 0;
            while let Some(pos) = open.pop() {
                let accessed = hh(&access_map, pos, cols);
                if accessed {
                    continue;
                }

                #[cfg(feature = "vis2")]
                {
                    // use std::thread::sleep;
                    // use std::time::Duration;
                    // sleep(Duration::from_millis(100));
                    disp_acc_map(&access_map, input, cols, rows);
                }

                *hhm(&mut access_map, pos, cols) = true;
                area += 1;

                // Explore
                if pos.0 > 0 {
                    let p = (pos.0 - 1, pos.1);
                    if hh(bytes, p, c1) == cur_reg {
                        open.push(p);
                    } else {
                        regions.push(p);
                    }
                }
                if pos.1 > 0 {
                    let p = (pos.0, pos.1 - 1);
                    if hh(bytes, p, c1) == cur_reg {
                        open.push(p);
                    } else {
                        regions.push(p);
                    }
                }
                if pos.0 < cols as i32 - 1 {
                    let p = (pos.0 + 1, pos.1);
                    if hh(bytes, p, c1) == cur_reg {
                        open.push(p);
                    } else {
                        regions.push(p);
                    }
                }
                if pos.1 < rows as i32 - 1 {
                    let p = (pos.0, pos.1 + 1);
                    if hh(bytes, p, c1) == cur_reg {
                        open.push(p);
                    } else {
                        regions.push(p);
                    }
                }

                // Sides
                if !hhc(bytes, (pos.0 - 1, pos.1), c1, cols, rows, cur_reg, false)
                    && (!hhc(bytes, (pos.0, pos.1 - 1), c1, cols, rows, cur_reg, false)
                        || (hhc(bytes, (pos.0, pos.1 - 1), c1, cols, rows, cur_reg, false)
                            && hhc(
                                bytes,
                                (pos.0 - 1, pos.1 - 1),
                                c1,
                                cols,
                                rows,
                                cur_reg,
                                false,
                            )))
                {
                    sides += 1;
                }
                if !hhc(bytes, (pos.0 + 1, pos.1), c1, cols, rows, cur_reg, false)
                    && (!hhc(bytes, (pos.0, pos.1 + 1), c1, cols, rows, cur_reg, false)
                        || (hhc(bytes, (pos.0, pos.1 + 1), c1, cols, rows, cur_reg, false)
                            && hhc(
                                bytes,
                                (pos.0 + 1, pos.1 + 1),
                                c1,
                                cols,
                                rows,
                                cur_reg,
                                false,
                            )))
                {
                    sides += 1;
                }

                if !hhc(bytes, (pos.0, pos.1 - 1), c1, cols, rows, cur_reg, false)
                    && (!hhc(bytes, (pos.0 + 1, pos.1), c1, cols, rows, cur_reg, false)
                        || (hhc(bytes, (pos.0 + 1, pos.1), c1, cols, rows, cur_reg, false)
                            && hhc(
                                bytes,
                                (pos.0 + 1, pos.1 - 1),
                                c1,
                                cols,
                                rows,
                                cur_reg,
                                false,
                            )))
                {
                    sides += 1;
                }
                if !hhc(bytes, (pos.0, pos.1 + 1), c1, cols, rows, cur_reg, false)
                    && (!hhc(bytes, (pos.0 - 1, pos.1), c1, cols, rows, cur_reg, false)
                        || (hhc(bytes, (pos.0 - 1, pos.1), c1, cols, rows, cur_reg, false)
                            && hhc(
                                bytes,
                                (pos.0 - 1, pos.1 + 1),
                                c1,
                                cols,
                                rows,
                                cur_reg,
                                false,
                            )))
                {
                    sides += 1;
                }
            }
            sum += area * sides;
        }
        sum

        // I really do need to make an AOC utility library...
    }

    #[cfg(feature = "vis")]
    pub fn disp_acc_map(v: &[bool], input: &str, cols: usize, rows: usize) {
        use colored::Colorize;

        print!("\u{001b}[{};{}H", 0, 0);
        for r in 0..rows {
            for c in 0..cols {
                let i = r * (cols + 1) + c;
                let d = &input[i..=i];
                if v[r * cols + c] {
                    print!("{}{}", d.bright_green(), d.bright_green());
                } else {
                    print!("{}{}", d.white(), d.white());
                }
            }
            println!();
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::*;

        #[test]
        fn sample() {
            assert_eq!(part2(SAMPLE), 1206);
        }

        #[test]
        fn simple() {
            let input = "AAAA
BBCD
BBCC
EEEC";
            assert_eq!(part2(input), 80);
        }

        #[test]
        fn e() {
            let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE";
            assert_eq!(part2(input), 236);
        }
    }
}
