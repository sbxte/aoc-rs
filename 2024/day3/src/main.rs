use nom::IResult;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, digit1};
use nom::sequence::tuple;

pub const SAMPLE: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
pub const SAMPLE2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

enum ParseResult<'a> {
    LeftRight(u32, u32),
    Next(&'a str),
}

fn parse_mul(input: &str) -> IResult<&str, ParseResult> {
    let (attempt, _) = take_until("mul(")(input)?;
    let (attempt, _) = tag("mul(")(attempt)?;
    let x: Result<_, nom::Err<nom::error::Error<&str>>> =
        tuple((digit1, char(','), digit1, char(')')))(attempt);
    let (remainder, (left, _, right, _)) = match x {
        Err(_) => {
            return Ok((attempt, ParseResult::Next(take_until("mul(")(attempt)?.0)));
        }
        Ok(x) => x,
    };
    let (left, right) = (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap());
    Ok((remainder, ParseResult::LeftRight(left, right)))
}

pub fn part1(input: &str) -> u32 {
    let mut input = input;
    let mut sum = 0;
    while !input.is_empty() {
        let (inp, x) = match parse_mul(input) {
            Ok(res) => res,
            Err(_) => break,
        };
        match x {
            ParseResult::LeftRight(left, right) => {
                input = inp;
                sum += left * right;
            }
            ParseResult::Next(next) => {
                input = next;
            }
        }
    }
    sum
}

// Imma just not use nom cuz lazy and I have skill issue :p
pub fn part2(input: &str) -> u32 {
    let mut i = 0;

    let mut enabled = true;
    let mut sum = 0;
    // Shortest mul operation is mul(x,x)
    // which takes a i..=i+7 window
    'out: while i + 7 < input.len() {
        if &input[i..i + 2] == "do" {
            if &input[i..i + 7] == "don't()" {
                enabled = false;
                i += 7;
                continue;
            }
            if &input[i..i + 4] == "do()" {
                enabled = true;
                i += 4;
                continue;
            }
        }
        if enabled && &input[i..i + 4] == "mul(" {
            let mut j = i + 4;
            let mut comma = 0;
            while j < input.len() {
                let c = &input[j..j + 1];
                match c {
                    "," => comma = j,
                    "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {}
                    ")" => break,
                    _ => {
                        i += 1;
                        continue 'out;
                    }
                }
                j += 1;
            }
            if comma == 0 {
                // Invalid input
                i += 1;
                continue;
            }

            let left = &input[i + 4..comma].parse::<u32>().unwrap();
            let right = &input[comma + 1..j].parse::<u32>().unwrap();
            sum += left * right;
        }

        i += 1;
    }
    sum
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 3 part 2: {}", part2(input));
    println!("Day 3 part 1: {}", part1(input));
}

#[cfg(test)]
mod d5 {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(part1(SAMPLE), 161);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(SAMPLE2), 48);
    }
}
