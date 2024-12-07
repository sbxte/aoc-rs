use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    combinator::{all_consuming, map},
    sequence::{preceded, tuple},
    Finish, IResult,
};

fn day18() {
    let input = include_str!("../foo.txt");
    let map = Map::parse(input);
    println!("Sides = {}", map.calc_sides(input));
}

#[derive(Debug)]
struct Map {
    lava: HashMap<u32, [bool; 6]>,
}

impl Map {
    fn parse(i: &str) -> Self {
        let lava = i
            .lines()
            .map(|l| all_consuming(Self::parse_line)(l).finish().unwrap().1)
            .map(|p| (p, [false; 6]))
            .collect::<HashMap<_, _>>();
        Self { lava }
    }

    fn parse_line(i: &str) -> IResult<&str, u32> {
        map(
            tuple((
                nom::character::complete::u8,
                preceded(tag(","), nom::character::complete::u8),
                preceded(tag(","), nom::character::complete::u8),
            )),
            |p| Self::tuple_to_u32(p),
        )(i)
    }

    fn tuple_to_u32(p: (u8, u8, u8)) -> u32 {
        ((p.0 as u32) << 16) + ((p.1 as u32) << 8) + (p.2 as u32)
    }

    fn u32_to_tuple(p: u32) -> (u8, u8, u8) {
        (
            (p >> 16) as u8,
            ((p % (1 << 16)) >> 8) as u8,
            (p % (1 << 8)) as u8,
        )
    }

    fn calc_sides(mut self, i: &str) -> u32 {
        let entries = i
            .lines()
            .map(|l| all_consuming(Self::parse_line)(l).finish().unwrap().1)
            .collect::<Vec<_>>();
        for entry in entries.iter() {
            self.update_side(*entry, (1, 0, 0), 0);
            self.update_side(*entry, (-1, 0, 0), 1);
            self.update_side(*entry, (0, 1, 0), 2);
            self.update_side(*entry, (0, -1, 0), 3);
            self.update_side(*entry, (0, 0, 1), 4);
            self.update_side(*entry, (0, 0, -1), 5);
        }

        let mut total_sides = 0;
        for lava in self.lava.values() {
            total_sides += lava.iter().fold(0, |acc, x| acc + if *x { 0 } else { 1 });
        }

        let mut air_bubbles = HashMap::new();
        for entry in entries.iter() {
            self.update_air(*entry, &mut air_bubbles);
        }
        for air_pos in air_bubbles.keys() {
            println!(
                "{:?} {:?}",
                Self::u32_to_tuple(*air_pos),
                air_bubbles.get(air_pos).unwrap()
            );
        }
        let mut trapped_air = 0;
        for air in air_bubbles.values() {
            trapped_air += 1 - air
                .iter()
                .fold(0, |acc, x| acc + if *x { 0 } else { 1 })
                .min(1);
        }
        total_sides - trapped_air * 6
    }

    fn update_air(&mut self, lava_pos: u32, mut air: &mut HashMap<u32, [bool; 6]>) {
        let lava = self.lava.get(&lava_pos).unwrap();

        let mut set = |air_pos: u32, i: usize| {
            if let Some(air) = (&mut air).get_mut(&air_pos) {
                air[i] = true;
            } else {
                let mut sides = [false; 6];
                sides[i] = true;
                air.insert(air_pos, sides);
            };
        };

        if !lava[0] {
            set(Self::shift(lava_pos, (1, 0, 0)), 0);
        }
        if !lava[1] {
            set(Self::shift(lava_pos, (-1, 0, 0)), 1);
        }
        if !lava[2] {
            set(Self::shift(lava_pos, (0, 1, 0)), 2);
        }
        if !lava[3] {
            set(Self::shift(lava_pos, (0, -1, 0)), 3);
        }
        if !lava[4] {
            set(Self::shift(lava_pos, (0, 0, 1)), 4);
        }
        if !lava[5] {
            set(Self::shift(lava_pos, (0, 0, -1)), 5);
        }
    }

    fn update_side(&mut self, lava: u32, s: (i8, i8, i8), i: usize) {
        if let Some(lava) = self.lava.get_mut(&Self::shift(lava, s)) {
            lava[i] = true;
        }
    }

    fn shift(p: u32, s: (i8, i8, i8)) -> u32 {
        let (x, y, z) = Self::u32_to_tuple(p);
        Self::tuple_to_u32((
            (x as i8 + s.0) as u8,
            (y as i8 + s.1) as u8,
            (z as i8 + s.2) as u8,
        ))
    }
}
