pub fn part1(input: &str) -> u32 {
    let sections = input.split("\n\n");
    // let pieces = sections.clone().take(6);
    let requirements = sections.last().unwrap().trim();
    let mut fits = 0;
    for requirement in requirements.lines() {
        let (dimension, pieces) = requirement.split_once(": ").unwrap();
        let (width, height) = dimension.trim().split_once('x').unwrap();
        let width = width.parse::<u32>().unwrap();
        let height = height.parse::<u32>().unwrap();
        let space_required = pieces.trim().split(' ').map(|s| s.parse::<u32>().unwrap()).fold(0, |acc, x| acc + x * 9);
        if space_required <= width * height {
            fits += 1;
        }
    }
    fits
}

