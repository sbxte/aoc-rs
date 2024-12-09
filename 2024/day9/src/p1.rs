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

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn sample() {
        assert_eq!(naive::part1(SAMPLE), 1928);
    }
}
