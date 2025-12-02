pub fn part2(input: &str) -> u64 {
    let mut sum = 0;
    let mut checked_id = vec![];
    for range in input.trim().split(',') {
        let range = range.trim();
        let (low_s, high_s) = range.split_once('-').expect("Unable to split range");
        let (low_s, high_s) = (low_s.trim(), high_s.trim());

        let l_digits = low_s.len();
        let h_digits = high_s.len();

        let (low, high) = (
            low_s.parse::<u64>().expect("Unable to parse u64"),
            high_s.parse::<u64>().expect("Unable to parse u64"),
        );

        let (top, half_top) = if h_digits & 1 == 1 {
            (10u64.pow((h_digits as u32 - 1) >> 1) - 1, 10u64.pow((h_digits as u32 - 1) >> 1) - 1)
        } else {
            let half_d = (h_digits) >> 1;
            let high_half = high_s[0..half_d].parse::<u64>().expect("Unable to parse half u64");
            let low_half = high_s[half_d..h_digits].parse::<u64>().expect("Unable to parse half u64");
            let half = low_half.max(high_half);
            (half * (10u64.pow(half_d as u32) + 1), half)
        };
        checked_id.clear();
        for x in 1..=half_top {
            let mut actual = 0;
            for i in 0..(h_digits as u32) {
                actual += x * 10u64.pow((x.ilog10() + 1) * i);
                if i < 1 {
                    continue;
                }
                if actual < low {
                    continue;
                }
                if actual > high {
                    break;
                }
                if checked_id.contains(&actual) {
                    continue;
                }
                checked_id.push(actual);
                sum += actual;
            }
        }
    }
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part2(input), 4174379265);
    }
}
