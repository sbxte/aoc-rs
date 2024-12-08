pub mod naive {
    use std::cmp::Ordering;
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

    pub fn bool2order(b: Option<bool>) -> Ordering {
        match b {
            None => Ordering::Equal,
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
        }
    }

    pub fn part2(input: &str) -> u32 {
        let (map, mut updates) = parse_input(input);

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

        for update in &mut updates {
            update.sort_by(|a, b| bool2order(map.get(a).map(|x| x.contains(b))));
        }

        let mut sum2 = 0;
        for update in &updates {
            sum2 += update[update.len() >> 1];
        }
        sum2 - sum
    }
}

pub mod optim {
    use std::cmp::Ordering;
    use std::collections::HashMap;

    #[derive(Debug, PartialEq, Eq, Clone, Default)]
    pub struct Node {
        parents: Vec<u32>,
        children: Vec<u32>,
    }

    pub fn parse_input(input: &str) -> (HashMap<u32, Node>, Vec<Vec<u32>>) {
        let mut lines = input.lines();

        let mut map: HashMap<u32, Node> = HashMap::new();
        for line in lines.by_ref() {
            if line.is_empty() {
                break;
            }
            let (left, right) = line
                .split_once('|')
                .map(|(l, r)| {
                    (
                        l.trim().parse::<u32>().unwrap(),
                        r.trim().parse::<u32>().unwrap(),
                    )
                })
                .unwrap();
            map.entry(left).or_default().children.push(right);
            map.entry(right).or_default().parents.push(left);
        }

        let mut updates = vec![];
        for line in lines {
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

    pub fn get_order(map: &HashMap<u32, Node>, a: u32, b: u32) -> Ordering {
        map.get(&a)
            .map(|x| {
                if x.children.contains(&b) {
                    Ordering::Less
                } else if x.parents.contains(&b) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            })
            .unwrap_or(Ordering::Equal)
    }

    pub fn part2(input: &str) -> u32 {
        let (map, mut updates) = parse_input(input);

        let mut incorrect_updates = vec![];
        for (i, update) in updates.iter().enumerate() {
            for window in update.windows(2) {
                if map
                    .get(&window[1])
                    .map(|v| v.children.contains(&window[0]))
                    .unwrap_or(false)
                {
                    incorrect_updates.push(i);
                    break;
                }
            }
        }

        let mut sum = 0;
        for idx in incorrect_updates {
            updates[idx].sort_by(|a, b| get_order(&map, *a, *b));
            sum += updates[idx][updates[idx].len() >> 1];
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
        assert_eq!(optim::part2(SAMPLE), 123);
    }

    #[test]
    fn optim_naive() {
        let input = include_str!("input.txt");
        assert_eq!(optim::part2(input), naive::part2(input));
    }
}
