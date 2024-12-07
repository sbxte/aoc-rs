pub const SAMPLE: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand<'a> {
    kind: Kind,
    hand: &'a str,
    bid: u32,
}

impl PartialOrd for Hand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind.partial_cmp(&other.kind) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord.unwrap(),
        }
        let mut our = self.hand.chars();
        let mut their = other.hand.chars();
        for _ in 0..5 {
            let our = our.next().unwrap();
            let their = their.next().unwrap();
            match card_to_idx(our).cmp(&card_to_idx(their)) {
                core::cmp::Ordering::Equal => {}
                ord => {
                    return ord;
                }
            }
        }
        panic!("OH NO");
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Kind {
    FiveK,
    FourK,
    FullH,
    ThreeK,
    TwoP,
    OneP,
    HighC,
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.to_idx().cmp(&other.to_idx()))
    }
}

impl Ord for Kind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Kind {
    pub fn from_hand(hand: &str) -> Self {
        let mut counts = [0; 13];
        for c in hand.chars() {
            let idx = card_to_idx(c);
            counts[idx] += 1;
        }
        let mut threes = 0;
        let mut twos = 0;
        for n in counts {
            match n {
                5 => return Self::FiveK,
                4 => return Self::FourK,
                3 => threes += 1,
                2 => twos += 1,
                _ => {}
            }
        }
        if threes == 1 && twos == 1 {
            Self::FullH
        } else if threes == 1 {
            Self::ThreeK
        } else if twos == 2 {
            Self::TwoP
        } else if twos == 1 {
            Self::OneP
        } else {
            Self::HighC
        }
    }

    pub fn to_idx(&self) -> usize {
        match self {
            Self::HighC => 0,
            Self::OneP => 1,
            Self::TwoP => 2,
            Self::ThreeK => 3,
            Self::FullH => 4,
            Self::FourK => 5,
            Self::FiveK => 6,
        }
    }
}

pub fn card_to_idx(card: char) -> usize {
    12 - match card {
        'A' => 0,
        'K' => 1,
        'Q' => 2,
        'J' => 3,
        'T' => 4,
        '9' => 5,
        '8' => 6,
        '7' => 7,
        '6' => 8,
        '5' => 9,
        '4' => 10,
        '3' => 11,
        '2' => 12,
        _ => panic!("wtf"),
    }
}

pub fn part1(input: &str) -> u32 {
    let mut hands = vec![];
    for line in input.lines() {
        let (l, r) = line.split_once(' ').unwrap();
        hands.push(Hand {
            hand: l.trim(),
            kind: Kind::from_hand(l.trim()),
            bid: r.trim().parse::<u32>().unwrap(),
        });
    }
    hands.sort();

    let mut sum = 0;
    for (i, hand) in hands.iter().enumerate() {
        sum += (i as u32 + 1) * hand.bid;
    }

    sum
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part1(input));
}

#[cfg(test)]
mod d7 {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(part1(SAMPLE), 6440);
    }
}
