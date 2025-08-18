pub fn part1(input: &str) -> usize {
    let input: Vec<_> = input
        .lines()
        .map(|s| {
            match (
                s.as_bytes()[0],
                atoi::atoi::<u32>(s[2..].as_bytes()).unwrap(),
            ) {
                (b'R', n) => ((1, 0), n),
                (b'L', n) => ((-1, 0), n),
                (b'U', n) => ((0, 1), n),
                (_, n) => ((0, -1), n),
            }
        })
        .collect();
    let mut visited: rustc_hash::FxHashSet<_> = Default::default(); // Visited positions
    let mut pointer = (0, 0); // Relative position of tail to head
    let mut position = (0, 0); // Absolute position of start to tail

    for (direction, amount) in input {
        // println!("({}) - {}", hash_set.len(), line);
        (pointer, position) =
            handle_movement_part_1(pointer, position, direction, amount, &mut visited, true);
    }

    visited.len()
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
