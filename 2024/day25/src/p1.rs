fn parse(input: &str) -> (Vec<[u8; 5]>, Vec<[u8; 5]>) {
    let input = input.trim();

    let mut locks = vec![];
    let mut keys = vec![];

    for thing in input.split("\n\n") {
        let thing = thing.trim();

        let mut heights = [0u8; 5];
        for line in thing.lines() {
            for (i, c) in line.char_indices() {
                if c as u8 == b'#' {
                    heights[i] += 1;
                }
            }
        }
        heights.iter_mut().for_each(|h| *h -= 1);

        if matches!(thing.lines().next().unwrap().trim(), "#####") {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    (locks, keys)
}

pub fn part1(input: &str) -> usize {
    let (locks, keys) = parse(input);
    let mut valids = 0;

    for lock in &locks {
        for key in &keys {
            let mut valid = true;
            for i in 0..5 {
                if lock[i] + key[i] > 5 {
                    valid = false;
                    break;
                }
            }
            if valid {
                valids += 1;
            }
        }
    }

    valids
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";
        assert_eq!(part1(input), 3);
    }
}
