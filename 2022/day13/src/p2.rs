use std::cmp::Ordering;

pub fn part2(input: &str) -> usize {
    let mut packets: Vec<_> = input
        .lines()
        .filter_map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(Data::new(l))
            }
        })
        .collect();

    let div1 = Data::new("[[2]]");
    let div2 = Data::new("[[6]]");
    packets.push(div1.clone());
    packets.push(div2.clone());

    packets.sort_unstable_by(|a, b| compare(a, b).to_ord());

    let div1 = packets
        .iter()
        .enumerate()
        .find(|(_, p)| p == &&div1)
        .unwrap()
        .0
        + 1;
    let div2 = packets
        .iter()
        .enumerate()
        .find(|(_, p)| p == &&div2)
        .unwrap()
        .0
        + 1;

    div1 * div2
}

enum Order {
    Right,
    Wrong,
    Continue,
}
impl Order {
    fn to_ord(self) -> Ordering {
        match self {
            Self::Right => Ordering::Less,
            Self::Wrong => Ordering::Greater,
            Self::Continue => Ordering::Equal,
        }
    }
}

fn compare(left: &Data, right: &Data) -> Order {
    use Data::*;

    match (left, right) {
        (Num(l), Num(r)) => {
            if l < r {
                Order::Right
            } else if l > r {
                Order::Wrong
            } else {
                Order::Continue
            }
        }
        (List(l), List(r)) => {
            let mut i = 0;
            loop {
                if i >= l.len() && i >= r.len() {
                    return Order::Continue;
                } else if i >= l.len() {
                    return Order::Right;
                } else if i >= r.len() {
                    return Order::Wrong;
                }
                let result = compare(&l[i], &r[i]);
                if matches!(result, Order::Continue) {
                    i += 1;
                } else {
                    return result;
                }
            }
        }
        (l, r) if matches!(l, List(_)) && matches!(r, Num(_)) => {
            compare(&l, &Data::List(vec![r.clone()]))
        }
        (l, r) if matches!(l, Num(_)) && matches!(r, List(_)) => {
            compare(&Data::List(vec![l.clone()]), &r)
        }
        (_, _) => unreachable!(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Data {
    Num(u32),
    List(Vec<Data>),
}

impl Data {
    pub fn new(input: &str) -> Self {
        match &input[0..=0] {
            "[" => Self::_new_list(input, 1).0,
            x if x.chars().next().unwrap().is_ascii_digit() => Self::_new_val(input, 1).0,
            _ => unreachable!("Invalid starting character"),
        }
    }

    fn _new_list(input: &str, index: usize) -> (Self, usize) {
        let mut v = vec![];
        let mut i = index;
        while i < input.len() {
            match &input[i..=i] {
                "[" => {
                    let result = Self::_new_list(input, i + 1);
                    v.push(result.0);
                    i = result.1;
                }
                "]" => {
                    return (Self::List(v), i + 1);
                }
                x if x.chars().next().unwrap().is_ascii_digit() => {
                    let result = Self::_new_val(input, i);
                    v.push(result.0);
                    i = result.1;
                }
                "," | " " => {
                    i += 1;
                }
                _ => unreachable!("Invalid character"),
            }
        }

        unreachable!("List parsing ended incorrectly")
    }

    fn _new_val(input: &str, start: usize) -> (Self, usize) {
        let end = input[start..].find(|c: char| !c.is_ascii_digit()).unwrap();
        let num = input[start..(start + end)].parse::<u32>().unwrap();
        (Self::Num(num), (start + end))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!(part2(input), 140);
    }
}
