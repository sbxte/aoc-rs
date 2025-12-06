fn number(line: &str) -> impl Iterator<Item = u64> {
    line.split(' ').filter_map(|s| {
        if s.is_empty() {
            None
        } else {
            Some(s.trim().parse::<u64>().expect("Unable to parse number"))
        }
    })
}

fn operator(line: &str) -> impl Iterator<Item = char> {
    line.split(' ').filter_map(|s| {
        if s.is_empty() {
            None
        } else {
            Some(s.chars().next().unwrap())
        }
    })
}

pub fn part1(input: &str) -> u64 {
    let mut lines = input.lines();
    let first = number(lines.next().unwrap());
    let second = number(lines.next().unwrap());
    let third = number(lines.next().unwrap());
    let fourth = number(lines.next().unwrap());
    let operators = operator(lines.next().unwrap());

    let a = first.zip(second);
    let b = third.zip(fourth);
    let c = a.zip(b).zip(operators);
    c.fold(0, |acc, (((first, second), (third, fourth)), op)| {
        acc + match op {
            '*' => first * second * third * fourth,
            '+' => first + second + third + fourth,
            x => unreachable!("Invalid operator: {}", x),
        }
    })
}

// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn sample() {
//         let input = "123 328  51 64
//  45 64  387 23
//   6 98  215 314
// *   +   *   +  ";
//         assert_eq!(part1(input), 4277556);
//     }
// }
