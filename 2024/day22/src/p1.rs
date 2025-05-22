pub fn generate(mut s: u64) -> u64 {
    s = ((s << 6) ^ s) & 0xffffff;
    s = (s >> 5) ^ s;
    s = ((s << 11) ^ s) & 0xffffff;
    s
}
pub fn part1(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        let mut s = line.trim().parse().unwrap();
        for _ in 0..2000 {
            s = generate(s);
        }
        sum += s;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let sample = "1
10
100
2024";
        assert_eq!(part1(sample), 37327623);
    }
}
