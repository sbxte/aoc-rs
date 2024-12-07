use parse::Record;

use self::parse::Point;

fn day15() {
    let input = include_str!("../foo.txt");
    // let y = 2_000_000;

    let map = Map::parse(input);

    map.test();
}

struct Map {
    records: Vec<Record>,
}

impl Map {
    fn test(&self) {
        for rec in &self.records {
            println!("{} {}", rec.beacon, rec.sensor.manhattan_dist(rec.beacon));
        }
    }

    fn find_beacon(&self, min: i64, max: i64) -> (i64, i64) {
        (0, 0)
    }

    fn parse(input: &str) -> Self {
        let records = input.lines().map(Record::must_parse).collect();
        Self { records }
    }

    fn dump(&self) {
        for record in &self.records {
            println!("{record:?}");
        }
    }

    fn ranges(&self, y: i64) -> impl Iterator<Item = RangeInclusive<i64>> {
        let mut ranges = vec![];
        for rec in &self.records {
            let radius = rec.sensor.manhattan_dist(rec.beacon);
            let y_dist = (y - rec.sensor.y).abs();
            if y_dist > radius {
                // coverage area doesn't touch line at `y`
                continue;
            }
            let d = radius - y_dist;
            let middle = rec.sensor.x;
            let start = middle - d;
            let end = middle + d;
            let range = start..=end;
            ranges.push(range);
        }
        ranges.sort_by_key(|r| *r.start());

        ranges.into_iter().coalesce(|a, b| {
            if b.start() - 1 <= *a.end() {
                if b.end() > a.end() {
                    Ok(*a.start()..=*b.end())
                } else {
                    Ok(a)
                }
            } else {
                Err((a, b))
            }
        })
    }

    fn num_impossible_positions(&self, y: i64) -> usize {
        let beacon_x_coords = self
            .records
            .iter()
            .filter(|rec| rec.beacon.y == y)
            .map(|rec| rec.beacon.x)
            .collect::<HashSet<_>>();

        self.ranges(y)
            .map(|r| {
                let range_size = (r.end() - r.start() + 1) as usize;
                let num_beacons_in_range = beacon_x_coords.iter().filter(|x| r.contains(x)).count();
                range_size - num_beacons_in_range
            })
            .sum::<usize>()
    }
}

mod parse {
    use std::fmt::Display;

    use nom::{
        bytes::complete::tag,
        combinator::{all_consuming, map},
        sequence::{preceded, separated_pair},
        Finish, IResult,
    };

    #[derive(Debug)]
    pub struct Record {
        pub sensor: Point,
        // Closest beacon to that sensor
        pub beacon: Point,
    }

    impl Record {
        pub fn must_parse(i: &str) -> Self {
            all_consuming(Self::parse)(i)
                .finish()
                .expect("Failed to parse input")
                .1
        }

        fn parse(i: &str) -> IResult<&str, Self> {
            // example line:
            // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
            map(
                separated_pair(
                    preceded(tag("Sensor at "), Point::parse),
                    tag(": closest beacon is at "),
                    Point::parse,
                ),
                |(sensor, beacon)| Record { sensor, beacon },
            )(i)
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Point {
        pub x: i64,
        pub y: i64,
    }

    impl Display for Point {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl Point {
        fn parse(i: &str) -> IResult<&str, Point> {
            map(
                separated_pair(
                    preceded(tag("x="), nom::character::complete::i64),
                    tag(", "),
                    preceded(tag("y="), nom::character::complete::i64),
                ),
                |(x, y)| Point { x, y },
            )(i)
        }

        pub fn manhattan_dist(self, other: Self) -> i64 {
            (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as i64
        }
    }
}
