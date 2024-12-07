pub mod naive {
    pub fn count_combs(target: u64, nums: &[u64], idx: usize, result: u64) -> u64 {
        // Last case
        if idx >= nums.len() {
            if result == target {
                return 1;
            }
            return 0;
        }

        let curr = nums[idx];
        count_combs(target, nums, idx + 1, result * curr)
            + count_combs(target, nums, idx + 1, result + curr)
            + count_combs(
                target,
                nums,
                idx + 1,
                format!("{}{}", result, curr).parse::<u64>().unwrap(),
            )
    }

    pub fn part2(input: &str) -> u64 {
        let mut sum = 0;
        for line in input.trim().lines() {
            let (target, rest) = line.split_once(':').unwrap();
            let (target, numbers): (u64, Vec<u64>) = (
                target.trim().parse::<u64>().unwrap(),
                rest.trim()
                    .split(' ')
                    .map(|x| x.trim().parse::<u64>().unwrap())
                    .collect(),
            );
            let count = count_combs(target, &numbers, 1, numbers[0]);

            if count > 0 {
                sum += target;
            }
        }
        sum
    }
}

pub mod optim {
    pub fn is_possible(target: u64, nums: &[u64], idx: usize, result: u64) -> bool {
        // Last case
        if idx >= nums.len() {
            return result == target;
        }

        let curr = nums[idx];
        is_possible(target, nums, idx + 1, result * curr)
            || is_possible(target, nums, idx + 1, result + curr)
            || {
                let digits = (curr as f64).log10().floor() as i32 + 1;
                let pow = 10f64.powi(digits);
                is_possible(
                    target,
                    nums,
                    idx + 1,
                    (result as f64 * pow + curr as f64) as u64,
                )
            }
    }

    pub fn part2(input: &str) -> u64 {
        let mut sum = 0;
        for line in input.trim().lines() {
            let (target, rest) = line.split_once(':').unwrap();
            let (target, numbers): (u64, Vec<u64>) = (
                target.trim().parse::<u64>().unwrap(),
                rest.trim()
                    .split(' ')
                    .map(|x| x.trim().parse::<u64>().unwrap())
                    .collect(),
            );
            if is_possible(target, &numbers, 1, numbers[0]) {
                sum += target;
            }
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn sample() {
        assert_eq!(naive::part2(SAMPLE), 11387);
        assert_eq!(optim::part2(SAMPLE), 11387);
    }

    #[test]
    fn optim_naive() {
        let input = include_str!("input.txt");
        assert_eq!(naive::part2(input), optim::part2(input));
    }
}
