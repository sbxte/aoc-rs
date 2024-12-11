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
                let ef = *e as f64;
                let d = ef.log10().floor();
                if d % 2.0 == 1.0 {
                    let mul = 10f64.powi((d as i32 + 1) >> 1);
                    buffer.push((ef / mul) as u64);
                    *e = (ef % mul) as u64;
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
