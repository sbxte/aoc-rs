pub const SAMPLE: &str = "Time:      7  15   30
Distance:  9  40  200";

pub const INPUT: &str = "Time:        42     68     69     85
Distance:   284   1005   1122   1341";

pub fn part1(input: &[(u32, u32)]) -> u32 {
    let mut prod = 1;
    for record in input {
        let (t, a) = record;
        // x * x - x * t + a < 0 solve for int x
        let (t, a) = (*t as f64, *a as f64);
        let mid = t / 2.0;
        let d = (t * t - 4.0 * a).sqrt() / 2.0;
        let x1 = mid - d;
        let x2 = mid + d;
        let n1 = (x1 + 1.0).floor() as u32;
        let n2 = (x2 - 1.0).ceil() as u32;
        let range = n2 - n1 + 1;
        prod *= range;
    }
    prod
}

pub fn part2(input: (u64, u64)) -> u64 {
    let mut prod = 1;
    let (t, a) = input;
    // x * x - x * t + a < 0 solve for int x
    let (t, a) = (t as f64, a as f64);
    let mid = t / 2.0;
    let d = (t * t - 4.0 * a).sqrt() / 2.0;
    let x1 = mid - d;
    let x2 = mid + d;
    let n1 = (x1 + 1.0).floor() as u64;
    let n2 = (x2 - 1.0).ceil() as u64;
    let range = n2 - n1 + 1;
    prod *= range;
    prod
}

fn main() {
    // println!("Part 1: {}", part1(&[(7, 9), (15, 40), (30, 200)]));
    println!(
        "Part 1: {}",
        part1(&[(42, 284), (68, 1005), (69, 1122), (85, 1341)])
    );
    println!("Part 2: {}", part2((42686985, 284100511221341)));
}
