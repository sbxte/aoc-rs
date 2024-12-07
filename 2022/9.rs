pub fn day09_alt() {
    let cmds = include_bytes!("../test.txt")
        .split(|b| b == &b'\n')
        .map(|l| match (l[0], atoi::atoi(&l[2..]).unwrap()) {
            (b'U', l) => ((0, -1), l),
            (b'D', l) => ((0, 1), l),
            (b'L', l) => ((-1, 0), l),
            (_, l) => ((1, 0), l),
        });
    let (mut knots, mut s): ([(i32, i32); 10], rustc_hash::FxHashSet<_>) = Default::default();
    s.insert((0, 0));

    for (d, l) in cmds {
        for _ in 0..l {
            knots[0].0 += d.0;
            knots[0].1 += d.1;

            for i in 1..10 {
                let (h, t) = knots.split_at_mut(i);
                let (h, t) = (h[i - 1], &mut t[0]);
                if h.0.abs_diff(t.0) > 1 || h.1.abs_diff(t.1) > 1 {
                    let d = (t.0 - h.0, t.1 - h.1);
                    let l = d.0.abs().max(d.1.abs());
                    let m = (d.0 / l, d.1 / l);
                    *t = (h.0 + m.0, h.1 + m.1);
                    (i == 9).then(|| s.insert(*t));
                } else {
                    break;
                }
            }
        }
    }

    println!("{}", s.len());
}

fn day09() {
    // let input = include_str!("../foo.txt")
    let inputs: Vec<_> = include_bytes!("../test.txt")
        .split(|b| b == &b'\n')
        .map(|s| match (&s[0], atoi::atoi::<u32>(&s[2..]).unwrap()) {
            (b'R', n) => ((1, 0), n),
            (b'L', n) => ((-1, 0), n),
            (b'U', n) => ((0, 1), n),
            (_, n) => ((0, -1), n),
        })
        .collect();

    part1(&inputs);
    part2(&inputs);
}

fn part1(inputs: &Vec<((i32, i32), u32)>) {
    let mut visited: rustc_hash::FxHashSet<_> = Default::default(); // Visited positions
    let mut pointer = (0, 0); // Relative position of tail to head
    let mut position = (0, 0); // Absolute position of start to tail

    for (direction, amount) in inputs {
        // println!("({}) - {}", hash_set.len(), line);
        (pointer, position) =
            handle_movement_part_1(pointer, position, *direction, *amount, &mut visited, true);
    }

    println!("Part 1 solution: {}", visited.len());
}

fn handle_movement_part_1(
    mut pointer: (i32, i32),
    mut position: (i32, i32),
    movement: (i32, i32),
    amount: u32,
    visited_positions: &mut rustc_hash::FxHashSet<String>,
    count_visits: bool,
) -> ((i32, i32), (i32, i32)) {
    for _ in 0..amount {
        pointer.0 += movement.0;
        pointer.1 += movement.1;

        if pointer.0.abs() >= 2 {
            pointer.0 -= pointer.0 >> 1;
            position.0 += pointer.0;
            position.1 += pointer.1;
            pointer.1 = 0;
        } else if pointer.1.abs() >= 2 {
            position.0 += pointer.0;
            pointer.0 = 0;
            pointer.1 -= pointer.1 >> 1;
            position.1 += pointer.1;
        }

        // println!("{:?} - {:?}", relative_pos, absolute_pos);

        if count_visits {
            visited_positions.insert(format!("{}:{}", position.0, position.1));
        }
    }
    (pointer, position)
}

fn part2(inputs: &Vec<((i32, i32), u32)>) {
    let mut knots = [(0, 0); 10];
    let mut pos = (0, 0);

    let mut visited: rustc_hash::FxHashSet<_> = Default::default(); // Visited positions

    for (direction, amount) in inputs {
        for _ in 0..*amount {
            let mut movement = *direction;
            for i in 0..10 {
                knots[i].0 += movement.0;
                knots[i].1 += movement.1;
                if dst_sq(knots[i]) >= 4 {
                    movement = norm(knots[i]);
                    knots[i].0 -= movement.0;
                    knots[i].1 -= movement.1;
                } else {
                    movement = (0,0);
                }
            }
            pos.0 += movement.0; pos.1 += movement.1;
            visited.insert(format!("{}:{}", pos.0, pos.1));
        }
        println!("{:?}", pos);
    }

    println!("Part 2 solution: {}", visited.len());
}


#[inline]
fn dst_sq(v: (i32, i32)) -> i32 {
    v.0 * v.0 + v.1 * v.1
}

#[inline]
fn norm(v: (i32, i32)) -> (i32, i32) {
    (if v.0 != 0 {1} else {0},
     if v.1 != 0 {1} else {0})
}