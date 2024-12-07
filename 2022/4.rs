fn main() {
    let mut file = OpenOptions::new()
                .read(true)
                .open("./foo.txt").unwrap();

    let mut string = String::new();
    let _ = file.read_to_string(&mut string);

    // Solve here
    let mut total_part_1 = 0;
    let mut total_part_2 = 0;
    for s in string.split("\n") {
        if s.is_empty() { break; }

        // s1,s2
        let (s1, s2) = s.split_once(',').unwrap();

        // n11-n12,n21-n22
        let (n11, n12) = s1.split_once('-').unwrap();
        let (n21, n22) = s2.split_once('-').unwrap();

        let n11 = n11.parse::<u32>().unwrap();
        let n12 = n12.parse::<u32>().unwrap();
        let n21 = n21.parse::<u32>().unwrap();
        let n22 = n22.parse::<u32>().unwrap();

        if part1(n11, n12, n21, n22) {
            total_part_1 += 1;
        }

        if part2(n11, n12, n21, n22) {
            total_part_2 += 1;
        }
    }

    println!("Total: {} {}", total_part_1, total_part_2);
}

fn part1(n11: u32, n12: u32, n21: u32, n22: u32) -> bool {
    return (n11 <= n21 && n12 >= n22) || (n11 >= n21 && n12 <= n22);
}

fn part2(n11: u32, n12: u32, n21: u32, n22: u32) -> bool {
    return (n11 <= n22 && n12 >= n21) || (n11 >= n22 && n12 <= n21);
}
