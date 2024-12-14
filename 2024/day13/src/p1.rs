pub mod naive {
    pub fn compute(v1: (i32, i32), v2: (i32, i32), target: (i32, i32)) -> Option<(i32, i32)> {
        // Linear algebra this thing
        let recip_det = v1.0 * v2.1 - v1.1 * v2.0;
        let x = target.0 * v2.1 - target.1 * v2.0;
        let y = -target.0 * v1.1 + target.1 * v1.0;
        if x % recip_det != 0 || y % recip_det != 0 {
            return None;
        }
        Some((x / recip_det, y / recip_det))
    }

    pub fn part1(input: &str) -> i32 {
        // Im sleepy and I frankly dont care enough about how bad this parsing looks
        let x = input
            .split("\n\n")
            .map(|x| {
                fn parse(inp: &str) -> (i32, i32) {
                    let (a, b) = inp.split_once(':').unwrap().1.split_once(',').unwrap();
                    let f = |e: &str| {
                        e.split_once('+')
                            .unwrap_or_else(|| e.split_once('=').unwrap())
                            .1
                            .trim()
                            .parse()
                            .unwrap()
                    };
                    (f(a), f(b))
                }
                let mut y = x.split('\n').map(parse);
                let a = y.next().unwrap();
                let b = y.next().unwrap();
                let c = y.next().unwrap();
                (a, b, c)
            })
            .map(|(a, b, c)| compute(a, b, c))
            .map(|res| res.map(|(x, y)| 3 * x + y).unwrap_or(0))
            .sum::<i32>();
        x
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::*;

        #[test]
        fn sample() {
            assert_eq!(part1(SAMPLE), 480);
        }
    }
}
