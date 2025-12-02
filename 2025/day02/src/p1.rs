pub fn part1(input: &str) -> u64 {
    let mut sum = 0;
    for range in input.trim().split(',') {
        let range = range.trim();
        let (low_s, high_s) = range.split_once('-').expect("Unable to split range");
        let (low_s, high_s) = (low_s.trim(), high_s.trim());

        let l_digits = low_s.len();
        let h_digits = high_s.len();

        // Get next even digited number
        let (bottom, half_bottom) = if l_digits & 1 == 1 {
            (
                10u64.pow(l_digits as u32) + 10u64.pow(l_digits as u32 >> 1),
                10u64.pow(l_digits as u32 >> 1),
            )
        } else {
            let half_d = (l_digits) >> 1;
            let high_half = low_s[0..half_d]
                .parse::<u64>()
                .expect("Unable to parse half u64");
            let low_half = low_s[half_d..l_digits]
                .parse::<u64>()
                .expect("Unable to parse half u64");
            if low_half > high_half {
                let next = high_half + 1;
                (next * (10u64.pow(half_d as u32) + 1), next)
            } else {
                let half = low_half.max(high_half);
                (half * (10u64.pow(half_d as u32) + 1), half)
            }
        };
        let (top, half_top) = if h_digits & 1 == 1 {
            (
                10u64.pow(h_digits as u32 - 1) - 1,
                10u64.pow((h_digits as u32 - 1) >> 1) - 1,
            )
        } else {
            let half_d = (h_digits) >> 1;
            let high_half = high_s[0..half_d]
                .parse::<u64>()
                .expect("Unable to parse half u64");
            let low_half = high_s[half_d..h_digits]
                .parse::<u64>()
                .expect("Unable to parse half u64");
            if low_half >= high_half {
                let half = low_half.min(high_half);
                (half * (10u64.pow(half_d as u32) + 1), half)
            } else {
                let prev = high_half - 1;
                (prev * (10u64.pow(half_d as u32) + 1), prev)
            }
        };

        if bottom > top {
            // Impossible range
            continue;
        }

        for half in half_bottom..=half_top {
            let result = half * 10u64.pow(half.ilog10() + 1) + half;
            // dbg!(result);
            sum += result;
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
        assert_eq!(part1(input), 1227775554);
    }
}
