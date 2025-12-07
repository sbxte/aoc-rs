pub fn part2(input: &str) -> u64 {
    let first_line = input.lines().next().expect("Unable to get first line").trim();

    let columns = first_line.len();
    let mut beams = vec![0; columns];
    let start_idx = first_line.find('S').expect("Unable to find starting beam position");
    beams[start_idx] = 1;

    for line in input.lines().skip(1) {
        for (i, c) in line.as_bytes().iter().enumerate() {
            // Ignore if there are no beams at this position
            if beams[i] == 0 {
                continue;
            }
            // Interestingly, there are no directly adjacent splitters, 
            // The next splitter is always at least 2 indices from the current splitter's index
            // Since new beams appear at i+1 and i-1, no splitter will be interfering with each
            // other and thus using a single vec is possible
            if *c == b'^' {
                beams[i - 1] += beams[i];
                beams[i + 1] += beams[i];
                beams[i] = 0;
            } 
        }
    }
    beams.iter().fold(0, |acc, x| acc + *x)
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
