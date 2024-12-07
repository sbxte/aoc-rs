pub const SAMPLE: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

pub fn isch(letters: &[char], index: usize, c: char) -> bool {
    letters[index] == c
}

pub fn part1(input: &str) -> u32 {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut letters: Vec<_> = input.chars().collect();
    letters.retain(|e| e != &'\n');

    let mut count = 0;
    for r in 0..rows {
        for c in 0..cols {
            if !isch(&letters, r * cols + c, 'X') {
                continue;
            }

            // Diagonals
            if r + 3 < rows
                && c + 3 < cols
                && isch(&letters, (r + 1) * cols + c + 1, 'M')
                && isch(&letters, (r + 2) * cols + c + 2, 'A')
                && isch(&letters, (r + 3) * cols + c + 3, 'S')
            {
                count += 1;
            }
            if r >= 3
                && c >= 3
                && isch(&letters, (r - 1) * cols + c - 1, 'M')
                && isch(&letters, (r - 2) * cols + c - 2, 'A')
                && isch(&letters, (r - 3) * cols + c - 3, 'S')
            {
                count += 1;
            }
            if r >= 3
                && c + 3 < cols
                && isch(&letters, (r - 1) * cols + c + 1, 'M')
                && isch(&letters, (r - 2) * cols + c + 2, 'A')
                && isch(&letters, (r - 3) * cols + c + 3, 'S')
            {
                count += 1;
            }
            if r + 3 < rows
                && c >= 3
                && isch(&letters, (r + 1) * cols + c - 1, 'M')
                && isch(&letters, (r + 2) * cols + c - 2, 'A')
                && isch(&letters, (r + 3) * cols + c - 3, 'S')
            {
                count += 1;
            }
            // Vertical
            if r + 3 < rows
                && isch(&letters, (r + 1) * cols + c, 'M')
                && isch(&letters, (r + 2) * cols + c, 'A')
                && isch(&letters, (r + 3) * cols + c, 'S')
            {
                count += 1;
            }
            if r >= 3
                && isch(&letters, (r - 1) * cols + c, 'M')
                && isch(&letters, (r - 2) * cols + c, 'A')
                && isch(&letters, (r - 3) * cols + c, 'S')
            {
                count += 1;
            }
            // Horizontal
            if c + 3 < cols
                && isch(&letters, r * cols + c + 1, 'M')
                && isch(&letters, r * cols + c + 2, 'A')
                && isch(&letters, r * cols + c + 3, 'S')
            {
                count += 1;
            }
            if c >= 3
                && isch(&letters, r * cols + c - 1, 'M')
                && isch(&letters, r * cols + c - 2, 'A')
                && isch(&letters, r * cols + c - 3, 'S')
            {
                count += 1;
            }
        }
    }

    count
}

pub fn part2(input: &str) -> u32 {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().len();
    let mut letters: Vec<_> = input.chars().collect();
    letters.retain(|e| e != &'\n');

    let mut count = 0;
    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if !isch(&letters, r * cols + c, 'A') {
                continue;
            }

            // Right
            if isch(&letters, (r - 1) * cols + c - 1, 'M')
                && isch(&letters, (r + 1) * cols + c - 1, 'M')
                && isch(&letters, (r - 1) * cols + c + 1, 'S')
                && isch(&letters, (r + 1) * cols + c + 1, 'S')
            {
                count += 1;
            }
            // Left
            if isch(&letters, (r - 1) * cols + c - 1, 'S')
                && isch(&letters, (r + 1) * cols + c - 1, 'S')
                && isch(&letters, (r - 1) * cols + c + 1, 'M')
                && isch(&letters, (r + 1) * cols + c + 1, 'M')
            {
                count += 1;
            }
            // Up
            if isch(&letters, (r - 1) * cols + c - 1, 'S')
                && isch(&letters, (r + 1) * cols + c - 1, 'M')
                && isch(&letters, (r - 1) * cols + c + 1, 'S')
                && isch(&letters, (r + 1) * cols + c + 1, 'M')
            {
                count += 1;
            }
            // Down
            if isch(&letters, (r - 1) * cols + c - 1, 'M')
                && isch(&letters, (r + 1) * cols + c - 1, 'S')
                && isch(&letters, (r - 1) * cols + c + 1, 'M')
                && isch(&letters, (r + 1) * cols + c + 1, 'S')
            {
                count += 1;
            }
        }
    }

    count
}

fn main() {
    let input = include_str!("input.txt");
    println!("Day 4 part 2: {}", part2(input));
    println!("Day 4 part 1: {}", part1(input));
}
