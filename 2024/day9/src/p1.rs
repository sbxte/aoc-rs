pub mod naive {
    pub fn parse_input(input: &str) -> Vec<Option<u64>> {
        let bytes = input.trim().as_bytes();

        let mut blocks = vec![];
        let mut is_file = true;
        let mut id = 0;
        let mut i = 0;
        while i < bytes.len() {
            if is_file {
                for _ in 0..(bytes[i] - b'0') {
                    blocks.push(Some(id));
                }
                id += 1;
            } else {
                for _ in 0..(bytes[i] - b'0') {
                    blocks.push(None);
                }
            }
            is_file = !is_file;
            i += 1;
        }
        blocks
    }

    pub fn defrag(mut blocks: Vec<Option<u64>>) -> Vec<u64> {
        let mut i = 0;
        let mut j = blocks.len() - 1;

        while i < j {
            let ib = blocks[i];
            if ib.is_some() {
                i += 1;
                continue;
            }
            let jb = blocks[j];
            if jb.is_none() {
                j -= 1;
                continue;
            }
            blocks[i] = Some(jb.unwrap());
            blocks[j] = None;
        }

        blocks.truncate(j);
        blocks.iter_mut().map(|e| e.unwrap()).collect()
    }

    pub fn checksum(blocks: Vec<u64>) -> u64 {
        let mut sum = 0;
        for (i, b) in blocks.iter().enumerate() {
            sum += i as u64 * b;
        }
        sum
    }
    pub fn part1(input: &str) -> u64 {
        let blocks = parse_input(input);
        let compacted = defrag(blocks);
        checksum(compacted)
    }
}

pub mod optim {
    pub fn part1(input: &str) -> u64 {
        let bytes = input.trim().as_bytes();
        let mut sum = 0;

        // Block segment pointers
        let mut i = 0;
        let mut j = bytes.len() - 1;

        // Block pointers
        let mut p = 0;
        let mut to = 0;
        let mut mv = bytes[j] - b'0';
        while i < j {
            if i % 2 == 0 {
                let size = (bytes[i] - b'0') as usize;
                let file_id = i >> 1;
                // Math magic
                // b, b + 1, b + 2, ... , b + (size - 1)
                // b + (size - 1) = a
                // size - 1 = a - b
                // p = b
                // (size - 1) + p = a
                // 1 / 2 * ( a * (a + 1) - (b-1) * b) = 1/2 * ( a^2 + a - b^2 + b)
                // 1/2 * ( (a-b)(a+b) + a+b)
                // 1/2 * (a+b)(a-b+1)
                // 1/2 * ((size-1) + p + p)(size - 1 + 1)
                // 1/2 * (size)((size-1) + 2p)
                let diff = (file_id * size * (size + 2 * p - 1)) >> 1;
                sum += diff;
                p += size;
                i += 1;
                to = bytes[i] - b'0';
                continue;
            }

            if mv <= to {
                to -= mv;
                let size = mv as usize;
                let file_id = (j + 1) >> 1;
                let diff = (file_id * size * (size + 2 * p - 1)) >> 1;
                sum += diff;
                p += mv as usize;
                j -= 2;
                mv = bytes[j] - b'0';
                if to == 0 {
                    i += 1;
                }
            } else {
                mv -= to;
                let size = to as usize;
                let file_id = (j + 1) >> 1;
                let diff = (file_id * size * (size + 2 * p - 1)) >> 1;
                sum += diff;
                p += to as usize;
                i += 1;
            }
        }

        if i == j {
            let size = mv as usize;
            let file_id = (j + 1) >> 1;
            let diff = (file_id * size * (size + 2 * p - 1)) >> 1;
            sum += diff;
        }
        sum as u64
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn sample() {
        assert_eq!(optim::part1(SAMPLE), 1928);
    }

    #[test]
    fn optim_naive() {
        let input = include_str!("input.txt");
        assert_eq!(naive::part1(input), optim::part1(input));
    }
}
