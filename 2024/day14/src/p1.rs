pub mod naive {
    use aocutils::num::BitwiseAvg as _;

    type Int = i32;
    type Vec2 = aocutils::cartes::dim2::vec::Vec2<Int>;

    fn parse_vector(input: &str) -> Vec2 {
        let (x, y) = input.split_once('=').unwrap().1.split_once(',').unwrap();
        From::from((x.trim().parse().unwrap(), y.trim().parse().unwrap()))
    }
    fn parse_line(input: &str) -> (Vec2, Vec2) {
        let (p, v) = input.split_once(' ').unwrap();
        let (p, v) = (parse_vector(p), parse_vector(v));
        (p, v)
    }
    fn parse_input(input: &str) -> (Vec<Vec2>, Vec<Vec2>) {
        let lines = input.lines().count();
        let mut positions = vec![Default::default(); lines];
        let mut velocities = vec![Default::default(); lines];

        for (i, line) in input.lines().enumerate() {
            let (p, v) = parse_line(line.trim());
            positions[i] = p;
            velocities[i] = v;
        }
        (positions, velocities)
    }
    fn sum_quadrants(mid: Vec2, positions: &[Vec2]) -> Int {
        let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
        positions.iter().for_each(|p| {
            if p.0 < mid.0 && p.1 < mid.1 {
                q1 += 1;
            } else if p.0 > mid.0 && p.1 < mid.1 {
                q2 += 1;
            } else if p.0 < mid.0 && p.1 > mid.1 {
                q3 += 1;
            } else if p.0 > mid.0 && p.1 > mid.1 {
                q4 += 1;
            }
        });
        q1 * q2 * q3 * q4
    }
    pub fn part1(input: &str) -> Int {
        let (b1, b2) = if cfg!(feature = "p1sample") {
            (From::from((0, 0)), From::from((11, 7)))
        } else {
            (From::from((0, 0)), From::from((101, 103)))
        };

        let (mut positions, velocities) = parse_input(input);
        positions
            .iter_mut()
            .zip(velocities.iter())
            .for_each(|(p, v)| *p = (*p + *v * 100).bounds_wrap(b1, b2));
        let mid = Vec2::from((b1.0.bitwise_avg(b2.0), b1.1.bitwise_avg(b2.1)));
        sum_quadrants(mid, &positions)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::*;

        #[test]
        fn sample() {
            assert!(cfg!(feature = "p1sample"));
            assert_eq!(part1(SAMPLE), 12);
        }
    }
}
