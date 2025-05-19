type Int = u64;
fn search(target: &str, towels: &[&str], buffer: &mut Vec<Int>) -> Int {
    buffer.clear();
    #[allow(clippy::same_item_push)]
    for _ in target.chars() {
        buffer.push(0);
    }
    buffer.push(0);
    buffer[0] = 1;

    let mut i = 0;
    while i < target.len() {
        if buffer[i] == 0 {
            i += 1;
            continue;
        }
        for towel in towels {
            if target[i..].starts_with(towel) {
                buffer[i + towel.len()] += buffer[i];
            }
        }
        i += 1;
    }
    buffer[target.len()]
}
pub fn part2(input: &str) -> Int {
    let mut input = input.split("\n\n");
    let towels: Vec<&str> = input
        .next()
        .unwrap()
        .trim()
        .split(',')
        .map(|s| s.trim())
        .collect();
    let targets = input.next().unwrap().lines();
    let mut buffer = vec![];
    let mut possible = 0;
    for target in targets {
        possible += search(target, &towels, &mut buffer);
    }
    possible
}

#[cfg(test)]
mod tests {
    use crate::SAMPLE;

    use super::*;

    #[test]
    fn sample() {
        assert_eq!(part2(SAMPLE), 16);
    }
}
