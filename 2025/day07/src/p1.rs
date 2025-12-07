pub fn part1(input: &str) -> u32 {
    let first_line = input.lines().next().expect("Unable to get first line").trim();

    let columns = first_line.len();
    let mut beams = vec![false; columns];
    let start_idx = first_line.find('S').expect("Unable to find starting beam position");
    beams[start_idx] = true;

    let mut splits = 0;
    for line in input.lines().skip(1) {
        for (i, c) in line.as_bytes().iter().enumerate() {
            if !beams[i] {
                continue;
            }
            if *c == b'^' {
                beams[i - 1] = true;
                beams[i + 1] = true;
                beams[i] = false;
                splits += 1;
            }
        }
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
