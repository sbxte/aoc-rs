pub mod naive {
    pub fn part1(input: &str) -> u64 {
        // NOTE: Binary tree might be useful for preserving order
        // gonna just ignore that for this part since order doesnt matter, just the entry count
        let mut numbers: Vec<_> = input
            .split(' ')
            .map(|e| e.trim().parse::<u64>().unwrap())
            .collect();

        let mut buffer = vec![];
        for _ in 0..25 {
            numbers.iter_mut().for_each(|e| {
                if *e == 0 {
                    *e = 1;
                    return;
                }
                let d = e.ilog10() + 1;
                if d % 2 == 0 {
                    let mul = 10u64.pow(d >> 1);
                    buffer.push(*e / mul);
                    *e %= mul;
                    return;
                }
                *e *= 2024;
            });
            numbers.append(&mut buffer);
        }
        numbers.len() as u64
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn sample() {
        assert_eq!(naive::part1(SAMPLE), 55312);
    }
}
