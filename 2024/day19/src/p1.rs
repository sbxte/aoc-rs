pub mod naive {
    pub fn naive_search(target: &str, towels: &[&str]) -> bool {
        if target.is_empty() {
            return true;
        }
        for towel in towels {
            if target.starts_with(towel) && naive_search(&target[towel.len()..], towels) {
                return true;
            }
        }
        false
    }

    pub fn part1(input: &str) -> u32 {
        // Parsing
        let mut input = input.split("\n\n");
        let towels: Vec<_> = input.next().unwrap().split(',').map(|s| s.trim()).collect();
        let targets = input.next().unwrap().lines();
        let mut possible = 0;
        for target in targets {
            if naive_search(target.trim(), &towels) {
                possible += 1;
            }
        }

        possible
    }
}

pub mod dtree {
    /// Dense Tree
    #[derive(Debug)]
    pub struct Tree<'a> {
        children: Vec<(&'a str, Box<Self>)>,
    }
    impl<'a> Tree<'a> {
        fn empty() -> Self {
            Self { children: vec![] }
        }
    }

    fn append_tree<'a>(substr: &'a str, tree: &mut Tree<'a>) {
        if substr.is_empty() {
            return;
        }

        for child in &mut tree.children {
            // child contains substr,
            // split child
            if child.0.starts_with(substr) {
                let (left, right) = child.0.split_at(substr.len());
                child.0 = left;
                let mut new = Box::new(Tree::empty());
                ::std::mem::swap(&mut child.1, &mut new);
                child.1.children.push((right, new));
                return;
            }
            // substr contains child,
            // split substr, recurse
            else if let Some(substr) = substr.strip_prefix(child.0) {
                append_tree(substr, &mut child.1);
                return;
            }
        }
        // no match, add new child
        tree.children.push((substr, Box::new(Tree::empty())));
    }

    pub fn parse_tree<'a>(towels: impl Iterator<Item = &'a str>) -> Tree<'a> {
        let mut tree = Tree::empty();
        for towel in towels {
            append_tree(towel.trim(), &mut tree);
        }
        tree
    }

    pub fn parse(input: &str) -> (Tree, impl Iterator<Item = &str>) {
        let mut x = input.split("\n\n").take(2);

        let tree = parse_tree(x.next().unwrap().trim().split(',').map(|s| s.trim()));
        let targets = x.next().unwrap().lines();

        (tree, targets)
    }

    pub fn search_tree(substr: &str, subtree: &Tree, tree: &Tree) -> bool {
        if substr.is_empty() {
            return true;
        }
        if let Some(child) = subtree.children.iter().find(|s| substr.starts_with(s.0))
            && search_tree(&substr[child.0.len()..], &child.1, tree)
        {
            return true;
        }
        if let Some(child) = tree.children.iter().find(|s| substr.starts_with(s.0))
            && search_tree(&substr[child.0.len()..], &child.1, tree)
        {
            return true;
        }
        false
    }

    pub fn part1(input: &str) -> usize {
        let (tree, targets) = parse(input);

        let mut possible = 0;
        for target in targets {
            if target.is_empty() {
                continue; // precaution
            }

            if search_tree(target.trim(), &tree, &tree) {
                possible += 1;
            }
        }

        possible
    }
}

pub mod dp {
    pub fn search(target: &str, towels: &[&str], buffer: &mut Vec<bool>) -> bool {
        buffer.clear();
        #[allow(clippy::same_item_push)]
        for _ in target.chars() {
            buffer.push(false);
        }
        buffer.push(false);
        buffer[0] = true;

        let mut i = 0;
        while i < target.len() {
            if !buffer[i] {
                i += 1;
                continue;
            }
            for towel in towels {
                if target[i..].starts_with(towel) {
                    buffer[i + towel.len()] = true;
                }
            }
            i += 1;
        }
        buffer[target.len()]
    }
    pub fn part1(input: &str) -> usize {
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
            if search(target, &towels, &mut buffer) {
                possible += 1;
            }
        }
        possible
    }
}

#[cfg(test)]
mod tests {
    use crate::SAMPLE;

    use super::*;

    #[test]
    fn sample() {
        assert_eq!(naive::part1(SAMPLE), 6);
        assert_eq!(dtree::part1(SAMPLE), 6);
        assert_eq!(dp::part1(SAMPLE), 6);
    }

    #[test]
    fn backtrack() {
        let input = "rg, rgur, u

rgurg";
        assert_eq!(naive::part1(input), 1);
        assert_eq!(dtree::part1(input), 1);
        assert_eq!(dp::part1(input), 1);
    }

    #[test]
    fn regex() {
        let input = include_str!("input.txt");
        let mut input = input.split("\n\n");
        let towels = input.next().unwrap().trim();
        let towel_re = ::std::iter::once("^(")
            .chain(towels.split(',').map(|s| s.trim()).intersperse("|"))
            .chain(::std::iter::once(")+$"))
            .collect::<Vec<_>>()
            .join("");
        dbg!(&towel_re);
        let re = regex::Regex::new(&towel_re).unwrap();

        let targets = input.next().unwrap().trim().lines();
        let mut buffer = vec![];
        for target in targets {
            let target = target.trim();
            assert_eq!(
                re.is_match(target),
                dp::search(
                    target,
                    &towels.split(',').map(|s| s.trim()).collect::<Vec<_>>(),
                    &mut buffer
                )
            );
        }
    }
}
