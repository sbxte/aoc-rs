use aocutils::cartes::dim2::vec::Vec2;

pub fn part1(input: &str) -> i64 {
    let mut highest_area = 0;
    let mut positions = Vec::with_capacity(input.lines().count());
    for line in input.lines() {
        let (left, right) = line.split_once(',').expect("Unable to parse vector");
        let pos = Vec2::from((left.parse::<i64>().unwrap(), right.parse::<i64>().unwrap()));
        for pos2 in &positions {
            let delta = pos - *pos2;
            let area = ((delta.0 + delta.0.signum()) * (delta.1 + delta.1.signum())).abs();
            highest_area = highest_area.max(area);
        }
        positions.push(pos);
    }
    highest_area
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
        assert_eq!(part1(input), 50);
    }
}
