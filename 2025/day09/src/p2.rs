use std::iter::once;

use aocutils::cartes::dim2::vec::Vec2;

fn parse_v(line: &str) -> Vec2<i64> {
    let (left, right) = line.split_once(',').expect("Unable to parse vector");
    let pos = Vec2::from((left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap()));
    pos
}

#[derive(PartialEq, Eq, Debug)]
struct Rect {
    min: Vec2<i64>,
    max: Vec2<i64>,
}
impl Rect {
    #[inline]
    fn new(p1: Vec2<i64>, p2: Vec2<i64>) -> Self {
        Self {
            min: Vec2::from((p1.0.min(p2.0), p1.1.min(p2.1))),
            max: Vec2::from((p1.0.max(p2.0), p1.1.max(p2.1))),
        }
    }
    #[inline]
    fn contains(&self, point: Vec2<i64>) -> bool {
        ((self.min.0 + 1)..self.max.0).contains(&point.0) && ((self.min.1 + 1)..self.max.1).contains(&point.1)
    }
    #[inline]
    fn area(&self) -> i64 {
        (self.max.0 - self.min.0 + 1) * (self.max.1 - self.min.1 + 1)
    }
}

pub fn part2(input: &str) -> i64 {
    let mut rects = vec![];
    let mut points = vec![];

    let mut prev_point = parse_v(input.lines().next().unwrap());
    points.push(prev_point);

    for line in input.lines().skip(1) {
        let p = parse_v(line);
        for p2 in &points {
            let p2 = *p2;
            let rect = Rect::new(p, p2);
            rects.push(rect);
        }
        points.push(p);
    }

    // If point is inside rectangle, remove the rectangle
    // O(N^2) EUGH
    prev_point = points[0];
    for point in points.iter().skip(1).chain(once(&points[0])) {
        // Actually O(N) because we're actually just iterating through points in a straight line
        for x in (prev_point.0.min(point.0))..=(prev_point.0.max(point.0)) {
            for y in (prev_point.1.min(point.1))..=(prev_point.1.max(point.1)) {
                rects.retain(|b| !b.contains(Vec2::from((x, y))));
            }
        }
        prev_point = *point;
    }

    let largest_rect = rects
        .iter()
        .max_by(|r1, r2| r1.area().cmp(&r2.area()))
        .expect("No rectangle found!");
    largest_rect.area()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        assert_eq!(part2(input), 24);
    }
}
