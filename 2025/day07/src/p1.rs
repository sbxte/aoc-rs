use std::mem::swap;

pub fn part1(input: &str) -> u32 {
    let first_line = input.lines().next().expect("Unable to get first line").trim();
    let columns = first_line.len();
    let mut tachyon_slice_1 = vec![false; columns];
    for (i, b) in first_line.as_bytes().iter().enumerate() {
        if *b == b'S' {
            tachyon_slice_1[i] = true;
        }
    }
    let mut tachyon_slice_2 = vec![false; columns];
    let mut splits = 0;
    for line in input.lines().skip(1) {
        for (i, c) in line.as_bytes().iter().enumerate() {
            if !tachyon_slice_1[i] {
                continue;
            }
            tachyon_slice_1[i] = false;
            if *c == b'^' {
                tachyon_slice_2[i - 1] = true;
                tachyon_slice_2[i + 1] = true;
                splits += 1;
            } else {
                tachyon_slice_2[i] = true;
            }
        }
        swap(&mut tachyon_slice_1, &mut tachyon_slice_2);
    }
    splits
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        assert_eq!(part1(input), 21);
    }
}
