pub fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        // First digit
        let trimmed = line.trim();
        let mut idx = 0;
        let mut first_digit = 0;
        for (i, chr) in trimmed[..trimmed.len() - 1].as_bytes().iter().enumerate() {
            let n = *chr - b'0';
            if n > first_digit {
                idx = i;
                first_digit = n;
            }
        }

        let mut second_digit = 0;
        for chr in trimmed[(idx + 1)..].as_bytes().iter() {
            let n = *chr - b'0';
            second_digit = second_digit.max(n);
        }

        let jolt = (first_digit * 10 + second_digit) as u32;
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

        assert_eq!(part1(input), 357);
    }
}
