use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

use aocutils::cartes::dim3::vec::Vec3;

#[derive(Eq, PartialEq, Debug)]
struct DistSort {
    dist: i64,
    v1: Vec3<i64>,
    v2: Vec3<i64>,
}
impl Ord for DistSort {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}
impl PartialOrd for DistSort {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq)]
struct Point {
    p: Vec3<i64>,
    circuit: Option<usize>,
}

pub fn compute_circuits(input: &str, max_conn: usize) -> Vec<Vec<Vec3<i64>>> {
    let mut points: HashMap<Vec3<i64>, Point> = HashMap::with_capacity(input.lines().count());
    let mut distances = BinaryHeap::new();
    for line in input.lines() {
        let (left, line) = line.split_once(',').unwrap();
        let (middle, right) = line.split_once(',').unwrap();
        let (left, middle, right) = (
            left.parse::<i64>().unwrap(),
            middle.parse::<i64>().unwrap(),
            right.parse::<i64>().unwrap(),
        );

        let v = Vec3::from((left, middle, right));
        for v_other in points.keys() {
            let v_other: Vec3<i64> = *v_other;
            let dist = v_other.euclidian_dist_sq(v);
            distances.push(Reverse(DistSort {
                dist,
                v1: v,
                v2: v_other,
            }));
        }

        points.insert(
            v,
            Point {
                p: v,
                circuit: None,
            },
        );
    }
    let mut circuits = vec![];
    let mut connections = 0;
    while let Some(dist) = distances.pop()
        && connections < max_conn
    {
        connections += 1;

        let dist = dist.0;
        let c1_some = points[&dist.v1].circuit.is_some();
        let c2_some = points[&dist.v2].circuit.is_some();
        match (c1_some, c2_some) {
            (false, false) => {
                points.get_mut(&dist.v1).unwrap().circuit = Some(circuits.len());
                points.get_mut(&dist.v2).unwrap().circuit = Some(circuits.len());
                circuits.push(vec![dist.v1, dist.v2]);
            }
            (true, false) => {
                let circ = points[&dist.v1].circuit.unwrap();
                points.get_mut(&dist.v2).unwrap().circuit = Some(circ);
                circuits[circ].push(dist.v2);
            }
            (false, true) => {
                let circ = points[&dist.v2].circuit.unwrap();
                points.get_mut(&dist.v1).unwrap().circuit = Some(circ);
                circuits[circ].push(dist.v1);
            }
            (true, true) => {
                // Move all points in circuit 2 into circuit 1
                let circ1_idx = points[&dist.v1].circuit.unwrap();
                let circ2_idx = points[&dist.v2].circuit.unwrap();
                if circ1_idx == circ2_idx {
                    continue;
                }
                for c in &circuits[points[&dist.v2].circuit.unwrap()] {
                    points.get_mut(c).unwrap().circuit = Some(circ1_idx);
                }
                let [circ1, circ2] = circuits.get_disjoint_mut([circ1_idx, circ2_idx]).unwrap();
                circ1.append(circ2);
            }
        }
    }
    // Sort biggest to smallest
    circuits.sort_by(|a, b| a.len().cmp(&b.len()).reverse());

    circuits
}

pub fn part1(input: &str) -> u64 {
    let max_conn = 1000;
    let circuits = compute_circuits(input, max_conn);
    let mut result = 1;
    for circ in circuits.iter().filter(|x| !x.is_empty()).take(3) {
        result *= circ.len();
    }
    result as u64
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let max_conn = 10;
        let circuits = compute_circuits(input, max_conn);
        let mut result = 1;
        for circ in circuits.iter().filter(|x| !x.is_empty()).take(3) {
            result *= circ.len();
        }
        assert_eq!(result, 40);
    }
}
