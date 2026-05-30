// Coord to index
fn c2i([cx, cy, cz]: [u32; 3], [_sx, sy, sz]: [u32; 3]) -> usize {
    (cx * sy * sz + cy * sz + cz) as usize
}

pub fn part1(input: &str) -> u32 {
    // Get bounding box
    let mut cells = vec![];

    let mut minx = u32::MAX;
    let mut maxx = u32::MIN;
    let mut miny = u32::MAX;
    let mut maxy = u32::MIN;
    let mut minz = u32::MAX;
    let mut maxz = u32::MIN;

    for line in input.lines() {
        let (x, line) = line.split_once(',').unwrap();
        let (y, z) = line.split_once(',').unwrap();
        let (x, y, z) = (
            x.parse::<u32>().unwrap(),
            y.parse::<u32>().unwrap(),
            z.parse::<u32>().unwrap(),
        );

        minx = minx.min(x);
        maxx = maxx.max(x);
        miny = minx.min(y);
        maxy = maxx.max(y);
        minz = minx.min(z);
        maxz = maxx.max(z);

        cells.push([x, y, z]);
    }

    let sizex = maxx - minx + 1;
    let sizey = maxy - miny + 1;
    let sizez = maxz - minz + 1;
    let sizec = [sizex, sizey, sizez];

    let mut space = vec![false; sizex as usize * sizey as usize * sizez as usize];

    let mut surface_area = 0;
    for [cx, cy, cz] in cells {
        let mut sides = 6;

        let xp = (cx < maxx && space[c2i([cx + 1, cy, cz], sizec)]) as u32;
        let xn = (cx > minx && space[c2i([cx - 1, cy, cz], sizec)]) as u32;
        let yp = (cy < maxy && space[c2i([cx, cy + 1, cz], sizec)]) as u32;
        let yn = (cy > miny && space[c2i([cx, cy - 1, cz], sizec)]) as u32;
        let zp = (cz < maxz && space[c2i([cx, cy, cz + 1], sizec)]) as u32;
        let zn = (cz > minz && space[c2i([cx, cy, cz - 1], sizec)]) as u32;

        let neighbours = xp + xn + yp + yn + zp + zn;
        surface_area -= neighbours;
        sides -= neighbours;
        surface_area += sides;

        space[c2i([cx, cy, cz], sizec)] = true;
    }

    surface_area
}
