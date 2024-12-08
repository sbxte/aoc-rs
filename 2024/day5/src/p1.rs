pub mod naive {
    use std::collections::HashMap;
    pub fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
        let mut lines = input.lines();

        let mut map: HashMap<u32, Vec<u32>> = HashMap::new();
        #[allow(clippy::while_let_on_iterator)]
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            let (left, right) = line.split_once('|').unwrap();
            let (left, right) = (
                left.trim().parse::<u32>().unwrap(),
                right.trim().parse::<u32>().unwrap(),
            );
            if let Some(x) = map.get_mut(&left) {
                x.push(right);
            } else {
                map.insert(left, vec![right]);
            }
        }

        let mut updates = vec![];
        #[allow(clippy::while_let_on_iterator)]
        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }
            updates.push(
                line.split(',')
                    .map(|x| x.trim().parse::<u32>().unwrap())
                    .collect::<Vec<_>>(),
            );
        }

        (map, updates)
    }
    pub fn part1(input: &str) -> u32 {
        let (map, updates) = parse_input(input);
        let mut sum = 0;
        for update in &updates {
            let mut valid = true;
            'out: for i in 0..update.len() {
                let check = match map.get(&{ update[i] }) {
                    None => continue,
                    Some(x) => x,
                };
                #[allow(clippy::needless_range_loop)]
                for j in 0..i {
                    if check.contains(&update[j]) {
                        valid = false;
                        break 'out;
                    }
                }
                #[allow(clippy::needless_range_loop)]
                for j in i + 1..update.len() {
                    if !check.contains(&update[j]) {
                        valid = false;
                        break 'out;
                    }
                }
            }
            if valid {
                sum += update[update.len() >> 1];
            }
        }
        sum
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn sample() {
        assert_eq!(naive::part1(SAMPLE), 143);
    }
}
