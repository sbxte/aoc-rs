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
        let mut i2 = i + 1;
        let mut j = blocks.len() - 1;
        let mut j2 = j;

        #[cfg(feature = "debug")]
        display_blocks(&blocks);

        while j2 > 0 {
            if i2 > j + 1 || i > j2 + 1 {
                #[cfg(feature = "debug")]
                {
                    display_blocks(&blocks);
                    dbg!(i, i2, j, j2);
                }
                j = j2;
                i = 0;
                i2 = 1;
            }

            let jb = blocks[j];
            if jb.is_none() {
                j -= 1;
                j2 = j;
                continue;
            }
            let j2b = blocks[j2];
            if let Some(id) = j2b
                && id == jb.unwrap()
            {
                j2 -= 1;
                continue;
            }

            #[cfg(feature = "debug")]
            {
                dbg!(i, i2, j, j2);
                display_blocks(&blocks);
            }

            let ib = blocks[i];
            if ib.is_some() {
                i += 1;
                i2 = i + 1;
                continue;
            }
            let i2b = blocks[i2];
            if i2b.is_none() {
                i2 += 1;
                continue;
            }

            #[cfg(feature = "debug")]
            {
                dbg!(i, i2, j, j2);
                display_blocks(&blocks);
            }

            let mv = j - j2;
            let to = i2 - i;

            if mv <= to {
                unsafe {
                    let x = blocks.as_mut_ptr().add(i);
                    let y = blocks.as_mut_ptr().add(j2 + 1);
                    std::ptr::swap_nonoverlapping(x, y, mv);
                }
                i = 0;
                i2 = i + 1;
                j = j2;
            } else {
                i = i2;
            }
            #[cfg(feature = "debug")]
            {
                dbg!(i, i2, j, j2);
                display_blocks(&blocks);
            }
        }
        #[cfg(feature = "debug")]
        display_blocks(&blocks);

        blocks.iter_mut().map(|e| e.unwrap_or(0)).collect()
    }

    pub fn checksum(blocks: Vec<u64>) -> u64 {
        let mut sum = 0;
        for (i, b) in blocks.iter().enumerate() {
            #[cfg(feature = "debug")]
            dbg!(i as u64 * b);

            sum += i as u64 * b;
        }
        sum
    }

    pub fn part2(input: &str) -> u64 {
        let blocks = parse_input(input);
        let compacted = defrag(blocks);
        checksum(compacted)
    }

    pub fn display_blocks(blocks: &[Option<u64>]) {
        for b in blocks {
            match b.as_ref() {
                Some(x) => print!("{},", *x),
                None => print!(".,"),
            }
        }
        println!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    #[test]
    fn sample() {
        assert_eq!(naive::part2(SAMPLE), 2858);
    }

    #[test]
    fn none() {
        let input = "123456789";
        assert_eq!(
            naive::part2(input),
            naive::checksum(
                naive::parse_input(input)
                    .iter()
                    .map(|e| e.unwrap_or(0))
                    .collect()
            )
        );
    }

    #[test]
    fn descending() {
        let input = "321";
        assert_eq!(naive::part2(input), 3);
    }
}
