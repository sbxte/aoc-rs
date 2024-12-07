use std::collections::HashMap;
use std::path::PathBuf;

pub const SAMPLE: &str = "$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Dir {
    size: u32,
}

impl Dir {
    pub fn new() -> Self {
        Self { size: 0 }
    }
}

fn recurse_dirs(
    lines: &[&str],
    mut line_idx: usize,
    tree: &mut HashMap<String, Dir>,
    curr_dir: PathBuf,
) -> (u32, usize) {
    if line_idx >= lines.len() {
        return (0, line_idx);
    }
    let curr_dir_str = curr_dir.to_str().unwrap().to_string();
    tree.insert(curr_dir_str.clone(), Dir::new());

    let mut sum = 0;

    while let Some(line) = lines.get(line_idx) {
        if line == &"$ cd .." {
            line_idx += 1;
            return (sum, line_idx);
        }
        if line.starts_with("$ ls") {
            let mut i = 1;
            while let Some(l) = lines.get(line_idx + i) {
                if l.starts_with("$") {
                    break;
                }
                if l.starts_with("dir") {
                    let dir = &l[4..];
                    if !tree.contains_key(&curr_dir_str) {
                        tree.insert(curr_dir_str.clone(), Dir::new());
                    }
                    if !tree.contains_key(dir) {
                        let mut d = curr_dir.clone();
                        d.push(dir);
                        tree.insert(d.to_str().unwrap().to_string(), Dir::new());
                    }
                } else {
                    let size = l.split_once(' ').unwrap().0.parse::<u32>().unwrap();
                    tree.get_mut(&curr_dir_str).unwrap().size += size;
                    sum += size;
                }
                i += 1;
            }
            line_idx += i;
        } else if line.starts_with("$ cd") {
            let mut d = curr_dir.clone();
            d.push(&line[5..]);
            let (s, i) = recurse_dirs(lines, line_idx + 1, tree, d);
            tree.get_mut(&curr_dir_str).unwrap().size += s;
            sum += s;
            line_idx = i;
        } else {
            dbg!(line);
            panic!("Invalid cmd");
        }
    }

    (sum, line_idx)
}

pub fn part1(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    let mut tree = HashMap::new();
    let _ = recurse_dirs(&lines, 0, &mut tree, PathBuf::new());

    let mut sum = 0;
    for (_, v) in tree {
        if v.size <= 100000 {
            sum += v.size;
        }
    }

    sum
}

pub fn part2(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<_>>();
    let mut tree = HashMap::new();
    let (root_size, _) = recurse_dirs(&lines, 0, &mut tree, PathBuf::new());

    let used = 70000000 - root_size;
    let required = 30000000 - used;
    let mut min = u32::MAX;
    for (_, v) in tree {
        if v.size >= required {
            min = min.min(v.size);
        }
    }

    min
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 2: {}", part2(input));
    println!("Part 1: {}", part1(input));
}

#[cfg(test)]
mod d7 {
    use super::*;
    #[test]
    fn p1() {
        assert_eq!(part1(SAMPLE), 95437);
    }
}
