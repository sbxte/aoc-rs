#[derive(Debug)]
struct Sensor {
    pos: (i64, i64),
    range: u64,
}
impl Sensor {
    pub fn new(pos: (i64, i64), beacon: (i64, i64)) -> Self {
        Self {
            range: pos.0.abs_diff(beacon.0) + pos.1.abs_diff(beacon.1),
            pos,
        }
    }

    pub fn in_range(&self, pos: (i64, i64)) -> bool {
        self.pos.0.abs_diff(pos.0) + self.pos.1.abs_diff(pos.1) <= self.range
    }

    pub fn edge_iter(&self) -> impl Iterator<Item = (i64, i64)> {
        let edge = self.range as i64 + 1;
        (0..edge)
            .map(move |x| (x, edge - x))
            .chain((0..edge).map(move |x| (edge - x, -x)))
            .chain((0..edge).map(move |x| (-x, x - edge)))
            .chain((0..edge).map(move |x| (x - edge, x)))
            .map(|(x, y)| (x + self.pos.0, y + self.pos.1))
    }
}

pub fn part2(input: &str) -> i64 {
    let sensors: Vec<_> = input
        .lines()
        .map(|l| l.split_once(':').unwrap())
        .map(|(s, b)| {
            let (sx, sy) = s[s.find('x').unwrap()..].split_once(',').unwrap();
            let s = (
                sx.strip_prefix("x=").unwrap().parse::<i64>().unwrap(),
                sy.trim()
                    .strip_prefix("y=")
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
            );
            let (bx, by) = b[b.find('x').unwrap()..].split_once(',').unwrap();
            let b = (
                bx.strip_prefix("x=").unwrap().parse::<i64>().unwrap(),
                by.trim()
                    .strip_prefix("y=")
                    .unwrap()
                    .parse::<i64>()
                    .unwrap(),
            );

            (s, b)
        })
        .map(|(s, b)| Sensor::new(s, b))
        .collect();

    #[cfg(not(test))]
    const LIMIT: i64 = 4000000;
    #[cfg(test)]
    const LIMIT: i64 = 20;

    for (i, s1) in sensors.iter().enumerate() {
        for (x, y) in s1.edge_iter() {
            if x < 0 || x > LIMIT || y < 0 || y > LIMIT {
                continue;
            }
            if sensors
                .iter()
                .enumerate()
                .filter(|(j, _)| i != *j)
                .all(|(_, s2)| !s2.in_range((x, y)))
            {
                dbg!(x, y);
                return x * 4000000 + y;
            }
        }
    }

    panic!("Cannot find beacon!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

        assert_eq!(part2(input), 56000011);
    }
}
