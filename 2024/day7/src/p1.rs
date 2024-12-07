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
    }

    pub fn part1(input: &str) -> u64 {
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
        if result > target {
            return false;
        }

        let curr = nums[idx];
        is_possible(target, nums, idx + 1, result * curr)
            || is_possible(target, nums, idx + 1, result + curr)
    }

    pub fn part1(input: &str) -> u64 {
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
        assert_eq!(naive::part1(SAMPLE), 3749);
    }

    #[test]
    fn reach_early() {
        assert_eq!(naive::part1("3: 2 1 2"), 0);
    }

    #[test]
    fn optim_naive() {
        let input = include_str!("input.txt");
        assert_eq!(naive::part1(input), optim::part1(input));
    }
}
