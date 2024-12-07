pub const SAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4

";

#[derive(Debug, PartialEq, Eq)]
pub struct Mapper {
    dest: usize,
    source: usize,
    size: usize,
}

impl Mapper {
    pub fn map(&self, index: usize) -> usize {
        if (self.source..self.source + self.size).contains(&index) {
            return (index - self.source) + self.dest;
        }
        index
    }

    pub fn rev(&self, index: usize) -> usize {
        if (self.dest..self.dest + self.size).contains(&index) {
            return (index - self.dest) + self.source;
        }
        index
    }
}

pub fn parse_maps(mut lines: std::str::Lines) -> Vec<Vec<Mapper>> {
    let _ = lines.next();
    let mut maps = vec![];
    let mut map = vec![];
    for line in lines {
        if line.trim().find(':').is_some() {
            continue;
        }
        if line.trim().is_empty() {
            maps.push(map);
            map = vec![];
            continue;
        }
        let (dest, rest) = line.split_once(' ').unwrap();
        let (source, size) = rest.split_once(' ').unwrap();
        let mapper = Mapper {
            dest: dest.trim().parse().unwrap(),
            source: source.trim().parse().unwrap(),
            size: size.trim().parse().unwrap(),
        };
        map.push(mapper);
    }
    if !map.is_empty() {
        maps.push(map);
    }
    maps
}

pub fn part1(input: &str) -> u32 {
    let mut lines = input.lines();
    let mut seeds: Vec<_> = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let maps = parse_maps(lines);

    let mut seed_mapped = Vec::with_capacity(seeds.len());
    for map in maps.iter() {
        'seed: for seed in seeds.iter() {
            for mapper in map {
                let s = mapper.map(*seed);
                if s != *seed {
                    seed_mapped.push(s);
                    continue 'seed;
                }
            }
            seed_mapped.push(*seed);
        }
        seeds.clear();
        seeds.append(&mut seed_mapped);
    }

    *seeds.iter().min().unwrap() as u32
}

pub fn part2_naive(input: &str) -> u32 {
    let mut lines = input.lines();

    let seed_ranges_str = lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .collect::<Vec<_>>();
    let mut i = 0;
    let mut seed_ranges = Vec::with_capacity(seed_ranges_str.len() >> 1);
    while i < seed_ranges_str.len() {
        let start = seed_ranges_str[i].parse::<usize>().unwrap();
        let size = seed_ranges_str[i + 1].parse::<usize>().unwrap();
        seed_ranges.push(start..start + size);
        i += 2;
    }

    let maps = parse_maps(lines);

    let mut i = 0;
    // TODO: Map seed ranges instead of brute forcing valid input from output
    loop {
        let mut j = i;
        let mut j_prev = j;
        for map in maps.iter().rev() {
            'map: for mapper in map {
                j = mapper.rev(j);
                if j_prev != j {
                    break 'map;
                }
            }
            j_prev = j;
        }
        if seed_ranges.iter().any(|r| r.contains(&j)) {
            return i as u32;
        }
        i += 1;
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 2: {}", part2_naive(input));
    println!("Part 1: {}", part1(input));
}

#[cfg(test)]
mod d5 {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(part1(SAMPLE), 35);
    }
}
