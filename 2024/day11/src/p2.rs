pub mod naive {
    use std::collections::HashMap;

    pub fn part2(input: &str, times: usize) -> u64 {
        let mut map1: HashMap<u64, u64> = input
            .split(' ')
            .map(|e| e.trim().parse::<u64>().unwrap())
            .fold(HashMap::new(), |mut map, x| {
                *map.entry(x).or_default() += 1;
                map
            });

        let mut map2 = HashMap::new();

        #[inline]
        fn m(m1: &mut HashMap<u64, u64>, m2: &mut HashMap<u64, u64>) {
            m1.iter().for_each(|(k, v)| {
                if *k == 0 {
                    *m2.entry(1).or_default() += *v;
                    return;
                }

                let ef = *k as f64;
                let d = ef.log10().floor();
                if d % 2.0 == 1.0 {
                    let mul = 10f64.powi((d as i32 + 1) >> 1);
                    *m2.entry((ef / mul) as u64).or_default() += *v;
                    *m2.entry((ef % mul) as u64).or_default() += *v;
                    return;
                }
                *m2.entry(*k * 2024).or_default() += *v;
            });
            m1.clear();
        }

        let mut map2to1 = false;
        for _ in 0..times {
            if map2to1 {
                m(&mut map2, &mut map1);
            } else {
                m(&mut map1, &mut map2);
            }
            map2to1 = !map2to1;
        }

        #[inline]
        fn count(m: HashMap<u64, u64>) -> u64 {
            m.values().sum()
        }

        if map2to1 { count(map2) } else { count(map1) }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn sample25() {
        assert_eq!(naive::part2(SAMPLE, 25), 55312);
    }
}
