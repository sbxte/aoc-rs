use std::io::Read;
use std::fs::OpenOptions;

// Used in both parts
fn conv_ascii(c: char) -> u32 {
    return if (c as u32) < 97 { // Capital letters
        c as u32 - 39 // -65 + 26
    } else {
        c as u32 - 97
    }
}


fn find_duplicate_part1(comp1: &str, comp2: &str) -> u32 {
    let mut bmap: [bool; 52] = [false; 52];

    for c in comp1.chars() {
        bmap[conv_ascii(c) as usize] = true;
    }
    for c in comp2.chars() {
        let a = conv_ascii(c);
        if bmap[a as usize] {
            return a + 1;
        }
    }

    panic!("NOT SUPPOSED TO REACH HERE!");
}

fn part1() {
    let mut file = OpenOptions::new()
                .read(true)
                .open("./foo.txt").unwrap();

    let mut string = String::new();
    let _ = file.read_to_string(&mut string);

    // Solve here

    let mut total = 0;
    for s in string.split("\n") {
        if s.is_empty() { break; }

        // Compartments
        let comp1 = &s[..s.len() / 2];
        let comp2 = &s[s.len() / 2..];

        total += find_duplicate_part1(comp1, comp2);
    }

    println!("Total: {}", total);
}







fn find_duplicate_part2(comp1: &str, comp2: &str, comp3: &str) -> u32 {
    let mut bmap1: [bool; 52] = [false; 52];
    let mut bmap2: [bool; 52] = [false; 52];

    for c in comp1.chars() {
        bmap1[conv_ascii(c) as usize] = true;
    }
    for c in comp2.chars() {
        bmap2[conv_ascii(c) as usize] = true;
    }
    for c in comp3.chars() {
        let a = conv_ascii(c);
        if bmap1[a as usize] && bmap2[a as usize] {
            return a + 1;
        }
    }

    panic!("NOT SUPPOSED TO REACH HERE!");
}

fn part2() {
    let mut file = OpenOptions::new()
            .read(true)
            .open("./foo.txt").unwrap();

    let mut string = String::new();
    let _ = file.read_to_string(&mut string);

    // Solve here

    let mut total = 0;
    let mut s1: Option<&str> = None;
    let mut s2: Option<&str> = None;
    for s in string.split("\n") {
        if s.is_empty() { break; }

        // Compartments
        if s1.is_none() {
            s1 = Some(s);
        } else if s2.is_none() {
            s2 = Some(s);
        } else {
            total += find_duplicate_part2(s1.unwrap(), s2.unwrap(), s);
            s1 = None;
            s2 = None;
        }
    }

    println!("Total: {}", total);
}
