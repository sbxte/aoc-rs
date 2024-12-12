pub mod naive {

    #[inline]
    fn hh<T: Copy>(grid: &[T], pos: (usize, usize), cols: usize) -> T {
        grid[pos.1 * cols + pos.0]
    }

    #[inline]
    fn hhm<T>(grid: &mut [T], pos: (usize, usize), cols: usize) -> &mut T {
        &mut grid[pos.1 * cols + pos.0]
    }

    pub fn part1(input: &str) -> u32 {
        let (cols, rows, bytes) = {
            let c = input.find('\n').unwrap();
            let r = input.lines().count();
            (c, r, input.as_bytes())
        };

        let mut access_map = vec![false; cols * rows];
        let mut regions = vec![];
        regions.push((0, 0));
        let c1 = cols + 1;

        #[cfg(feature = "vis1")]
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

            let mut perimeter = 0;
            let mut area = 0;
            while let Some(pos) = open.pop() {
                let accessed = hh(&access_map, pos, cols);
                if accessed {
                    continue;
                }

                #[cfg(feature = "vis1")]
                {
                    // use std::thread::sleep;
                    // use std::time::Duration;
                    // sleep(Duration::from_millis(100));
                    disp_acc_map(&access_map, input, cols, rows);
                }

                *hhm(&mut access_map, pos, cols) = true;
                area += 1;

                if pos.0 > 0 {
                    let p = (pos.0 - 1, pos.1);
                    if hh(bytes, p, c1) == cur_reg {
                        open.push(p);
                    } else {
                        perimeter += 1;
                        regions.push(p);
                    }
                } else {
                    perimeter += 1;
                }
                if pos.1 > 0 {
                    let p = (pos.0, pos.1 - 1);
                    if hh(bytes, p, c1) == cur_reg {
                        open.push(p);
                    } else {
                        perimeter += 1;
                        regions.push(p);
                    }
                } else {
                    perimeter += 1;
                }
                if pos.0 < cols - 1 {
                    let p = (pos.0 + 1, pos.1);
                    if hh(bytes, p, c1) == cur_reg {
                        open.push(p);
                    } else {
                        perimeter += 1;
                        regions.push(p);
                    }
                } else {
                    perimeter += 1;
                }
                if pos.1 < rows - 1 {
                    let p = (pos.0, pos.1 + 1);
                    if hh(bytes, p, c1) == cur_reg {
                        open.push(p);
                    } else {
                        perimeter += 1;
                        regions.push(p);
                    }
                } else {
                    perimeter += 1;
                }
            }
            sum += area * perimeter;
        }
        sum
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
            assert_eq!(part1(SAMPLE), 1930);
        }
    }
}
