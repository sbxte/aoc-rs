// Coord to index
fn c2i([cx, cy, cz]: [u32; 3], [_sx, sy, sz]: [u32; 3], [mx, my, mz]: [u32; 3]) -> usize {
    ((cx - mx) * sy * sz + (cy - my) * sz + (cz - mz)) as usize
}

pub fn part2(input: &str) -> usize {
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

    let minc = [minx, miny, minz];

    let mut blob = vec![false; sizex as usize * sizey as usize * sizez as usize];
    let mut seen = vec![false; sizex as usize * sizey as usize * sizez as usize];

    let mut surface_area = 0;
    for [cx, cy, cz] in cells {
        blob[c2i([cx, cy, cz], sizec, minc)] = true;

        // blob faces that are on the outer faces of the traversable space
        let touching_boundary = [
            cx == maxx,
            cx == minx,
            cy == maxy,
            cy == miny,
            cz == maxz,
            cz == minz,
        ];
        surface_area += touching_boundary.iter().filter(|b| **b).count();
    }

    let mut fill_queue = vec![];

    // All cells at the outer edge
    let ranges = [
        // Z face
        (minx..=maxx, miny..=maxy, minz..=minz),
        (minx..=maxx, miny..=maxy, maxz..=maxz),
        // Y face
        (minx..=maxx, miny..=miny, minz..=maxz),
        (minx..=maxx, maxy..=maxy, minz..=maxz),
        // X face
        (minx..=minx, miny..=maxy, minz..=maxz),
        (maxx..=maxx, miny..=maxy, minz..=maxz),
    ];
    for (rx, ry, rz) in ranges {
        for x in rx {
            for y in ry.clone() {
                for z in rz.clone() {
                    let c = [x, y, z];
                    let i = c2i(c, sizec, minc);
                    if !seen[i] && !blob[i] {
                        seen[i] = true;
                        fill_queue.push(c);
                    }
                }
            }
        }
    }

    while let Some(cell) = fill_queue.pop() {
        let [cx, cy, cz] = cell;

        let cc = [
            [cx + 1, cy, cz],
            [cx - 1, cy, cz],
            [cx, cy + 1, cz],
            [cx, cy - 1, cz],
            [cx, cy, cz + 1],
            [cx, cy, cz - 1],
        ];
        let ccheck = [
            cx < maxx,
            cx > minx,
            cy < maxy,
            cy > miny,
            cz < maxz,
            cz > minz,
        ];

        for ci in 0..6 {
            if !ccheck[ci] {
                continue;
            }

            let c = cc[ci];
            let i = c2i(c, sizec, minc);
            if blob[i] {
                surface_area += 1;
            } else if !seen[i] {
                fill_queue.push(c);
                seen[i] = true;
            }
        }
    }
    surface_area
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";
        assert_eq!(part2(input), 58);
    }
}
