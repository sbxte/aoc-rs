pub fn part1(input: &str) -> usize {
    input
        .trim()
        .split("\n\n")
        .map(|pair| pair.trim().split_once('\n').unwrap())
        .enumerate()
        .fold(0, |a, (i, (left, right))| {
            let (left, right) = (Data::new(left), Data::new(right));

            if matches!(compare(&left, &right), Order::Right) {
                a + i + 1
            } else {
                a
            }
        })
}

enum Order {
    Right,
    Wrong,
    Continue,
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

#[derive(Debug, Clone)]
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
        assert_eq!(part1(input), 13);
    }
}
