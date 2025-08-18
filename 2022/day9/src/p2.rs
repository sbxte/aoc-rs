use std::collections::HashSet;

#[inline]
fn movement(v: (i32, i32)) -> (i32, i32) {
    if v.0 == 0 {
        (0, v.1.signum())
    } else if v.1 == 0 {
        (v.0.signum(), 0)
    } else {
        (v.0.signum(), v.1.signum())
    }
}

pub fn part2(input: &str) -> usize {
    let input: Vec<_> = input
        .lines()
        .map(|s| {
            match (
                s.as_bytes()[0],
                atoi::atoi::<u32>(s[2..].as_bytes()).unwrap(),
            ) {
                (b'R', n) => ((1, 0), n),
                (b'L', n) => ((-1, 0), n),
                (b'U', n) => ((0, 1), n),
                (_, n) => ((0, -1), n),
            }
        })
        .collect();

    let mut knots = [(0i32, 0i32); 10];

    let mut visited: HashSet<_, _> = HashSet::new();
    for (direction, amount) in input {
        for _ in 0..amount {
            // Head
            knots[0].0 += direction.0;
            knots[0].1 += direction.1;

            for i in 1..knots.len() {
                let diff = (knots[i - 1].0 - knots[i].0, knots[i - 1].1 - knots[i].1);
                if diff.0.abs() >= 2 || diff.1.abs() >= 2 {
                    let m = movement(diff);
                    knots[i].0 += m.0;
                    knots[i].1 += m.1;
                }
            }

            let tail = knots[knots.len() - 1];
            visited.insert(tail);
        }
    }

    visited.len()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn sample_small() {
        let input = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn sample_large() {
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        assert_eq!(part2(input), 36);
    }
}
