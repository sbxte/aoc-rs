pub fn part2(input: &str) -> usize {
    let grid: Vec<_> = input
        .chars()
        .filter_map(|c| {
            // skip whitespace
            // map numbers 0 to 9
            if c.is_whitespace() {
                None
            } else {
                Some((c as u8 - b'0') as u8)
            }
        })
        .collect();

    let cols = input.lines().next().unwrap().len();
    let rows = input.lines().count();

    let mut max = 0;
    for r in 1..(rows - 1) {
        for c in 1..(cols - 1) {
            let tree = grid[r * cols + c];

            // Scan left
            let mut left = 0;
            for c1 in (0..c).rev() {
                let check = grid[r * cols + c1];
                left += 1;
                if check >= tree {
                    break;
                }
            }
            // Scan right
            let mut right = 0;
            for c1 in (c + 1)..cols {
                let check = grid[r * cols + c1];
                right += 1;
                if check >= tree {
                    break;
                }
            }
            // Scan up
            let mut up = 0;
            for r1 in (0..r).rev() {
                let check = grid[r1 * cols + c];
                up += 1;
                if check >= tree {
                    break;
                }
            }
            // Scan down
            let mut down = 0;
            for r1 in (r + 1)..rows {
                let check = grid[r1 * cols + c];
                down += 1;
                if check >= tree {
                    break;
                }
            }

            let score = left * right * up * down;
            if score > max {
                max = score;
                dbg!((c, r));
                dbg!((left, right, up, down));
            }
        }
    }

    max
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "30373
25512
65332
33549
35390";
        assert_eq!(part2(input), 8);
    }
}
