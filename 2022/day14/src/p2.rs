use std::collections::HashSet;

/// Naive approach of using hashset
/// Perhaps using a growable grid would be faster
/// But that's a puzzle for future me :)
pub fn part2(input: &str) -> usize {
    let (mut lx, mut hx, mut hy) = (500, 500, 0);
    let mut polylines: Vec<Polyline> = Default::default();
    for line in input.lines() {
        let polyline;
        (polyline, lx, hx, hy) = Polyline::parse(line, lx, hx, hy);
        polylines.push(polyline);
    }

    let mut max_y = 0;
    let mut grid = HashSet::new();
    for polyline in polylines {
        let mut points = polyline.points.iter();
        let mut prev = points.next().unwrap();
        for point in points {
            let hx = point.x.max(prev.x);
            let lx = point.x.min(prev.x);
            let hy = point.y.max(prev.y);
            let ly = point.y.min(prev.y);
            for x in lx..=hx {
                for y in ly..=hy {
                    grid.insert((x, y));
                    max_y = max_y.max(y);
                }
            }

            prev = point;
        }
    }

    let (mut sx, mut sy) = (500, 0);
    let mut counter = 0;

    while !grid.contains(&(500, 0)) {
        if sy == max_y + 1 {
            counter += 1;
            grid.insert((sx, sy));
            (sx, sy) = (500, 0);
            continue;
        }

        if !grid.contains(&(sx, sy + 1)) {
            sy += 1;
            continue;
        }
        if !grid.contains(&(sx - 1, sy + 1)) {
            sx -= 1;
            sy += 1;
            continue;
        }
        if !grid.contains(&(sx + 1, sy + 1)) {
            sx += 1;
            sy += 1;
            continue;
        }

        counter += 1;
        grid.insert((sx, sy));
        (sx, sy) = (500, 0);
    }

    counter
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn parse(s: &str) -> Point {
        let mut tokens = s.split(',');
        let (x, y) = (tokens.next().unwrap(), tokens.next().unwrap());
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(Debug, Default)]
struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    fn parse(s: &str, mut lx: i32, mut hx: i32, mut hy: i32) -> (Self, i32, i32, i32) {
        (
            Self {
                points: s
                    .split(" -> ")
                    .map(|s| {
                        let point = Point::parse(s);
                        lx = lx.min(point.x);
                        hx = hx.max(point.x);
                        hy = hy.max(point.y);
                        point
                    })
                    .collect(),
            },
            lx,
            hx,
            hy,
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
        assert_eq!(part2(input), 93);
    }
}
