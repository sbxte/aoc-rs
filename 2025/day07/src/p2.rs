use std::mem::swap;

pub fn part2(input: &str) -> u64 {
    let first_line = input.lines().next().expect("Unable to get first line").trim();
    let columns = first_line.len();
    let mut tachyon_slice_1 = vec![0; columns];
    for (i, b) in first_line.as_bytes().iter().enumerate() {
        if *b == b'S' {
            tachyon_slice_1[i] = 1;
        }
    }
    let mut tachyon_slice_2 = vec![0; columns];
    for line in input.lines().skip(1) {
        for (i, c) in line.as_bytes().iter().enumerate() {
            if tachyon_slice_1[i] == 0 {
                continue;
            }
            let timelines = tachyon_slice_1[i];
            tachyon_slice_1[i] = 0;
            if *c == b'^' {
                tachyon_slice_2[i - 1] += timelines;
                tachyon_slice_2[i + 1] += timelines;
            } else {
                tachyon_slice_2[i] += timelines;
            }
        }
        swap(&mut tachyon_slice_1, &mut tachyon_slice_2);
    }
    tachyon_slice_1.iter().fold(0, |acc, x| acc + *x)
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
        assert_eq!(part2(input), 40);
    }
}
