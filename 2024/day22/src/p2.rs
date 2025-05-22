use std::collections::HashMap;
use std::collections::hash_map::Entry;

type Int = i64;

fn generate(mut s: Int) -> Int {
    s = ((s << 6) ^ s) & 0xffffff;
    s = (s >> 5) ^ s;
    s = ((s << 11) ^ s) & 0xffffff;
    s
}

struct Generator {
    s: Int,
}
impl Generator {
    fn new(s: Int) -> Self {
        Self { s }
    }
}

impl Iterator for Generator {
    type Item = Int;

    fn next(&mut self) -> Option<Self::Item> {
        let out = generate(self.s);
        self.s = out;
        Some(out)
    }
}

pub fn part2(input: &str) -> i64 {
    let mut table = HashMap::new();
    let buyers = input.lines().count();
    for (i, s) in input.lines().map(|s| s.parse().unwrap()).enumerate() {
        let g = Generator::new(s);
        let g = g.take(2000);
        let mut g = g.map(|x| x % 10).map_windows(|[x, y]| (y - x, *y));
        let mut a = g.next().unwrap().0;
        let mut b = g.next().unwrap().0;
        let mut c = g.next().unwrap().0;
        let (mut d, price) = g.next().unwrap();
        match table.entry([a, b, c, d]) {
            Entry::Vacant(e) => {
                let mut v = vec![false; buyers];
                v[i] = true;
                e.insert((price, v));
            }
            Entry::Occupied(mut e) => {
                if !e.get().1[i] {
                    e.get_mut().0 += price;
                    e.get_mut().1[i] = true;
                }
            }
        }
        for (delta, price) in g {
            a = b;
            b = c;
            c = d;
            d = delta;
            match table.entry([a, b, c, d]) {
                Entry::Vacant(e) => {
                    let mut v = vec![false; buyers];
                    v[i] = true;
                    e.insert((price, v));
                }
                Entry::Occupied(mut e) => {
                    if !e.get().1[i] {
                        e.get_mut().0 += price;
                        e.get_mut().1[i] = true;
                    }
                }
            }
        }
    }

    table.values().map(|(b, _)| *b).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "1
2
3
2024";
        assert_eq!(part2(input), 23);
    }
}
