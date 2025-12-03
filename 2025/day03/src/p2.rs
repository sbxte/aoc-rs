pub fn part2(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        // First digit
        let trimmed = line.trim();

        let mut jolt = 0;

        let mut idx = -1;
        for digit_idx in 0..12 {
            let mut max_digit = 0;
            let end = trimmed.len() - (12 - 1 - digit_idx);
            idx += 1;
            let start = idx as usize;
            for (i, chr) in trimmed[start..end].as_bytes().iter().enumerate() {
                let n = *chr - b'0' ;
                if n > max_digit {
                    idx = (start + i) as i32;
                    max_digit = n;
                }
            }

            jolt = jolt * 10 + max_digit as u64;
        }
        sum += jolt;
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        assert_eq!(part2(input), 3121910778619);
    }
}
