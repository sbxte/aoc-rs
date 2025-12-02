pub fn part2(input: &str) -> u32 {
    let mut password = 0;
    let mut position = 50;
    for line in input.lines() {
        let right = matches!(&line[0..1], "R");
        let amount = line[1..].parse::<i32>().expect("Invalid number found");
        for _ in 0..amount {
            let dir: i32 = if right { 1 } else { -1 };
            position = (position + dir).rem_euclid(100);
            if position == 0 {
                password += 1
            }
        }
    }
    password
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let sample = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
        assert_eq!(part2(sample), 6);
    }
}
