
fn day10() {
    let cmds: Vec<(u32, i32)> = include_bytes!("../test.txt")
        .split(|f| f == &b'\n')
        .map(|f| {
            if f.contains(&b' ') {
                // Has a space = has arguments
                match (&f[0..4], atoi::atoi(&f[5..]).unwrap()) {
                    (b"addx", x) => (1, x),
                    (_, _) => (0, 0),
                }
            } else {
                (0, 0)
            }
        })
        .collect();

    part1(&cmds);
    part2(&cmds);
}

fn part2(cmds: &Vec<(u32, i32)>) {
    let mut sprite_position = 1; // addx register = sprite position
    let mut crt_position = 0;

    #[allow(unused)] let mut clock = 1;

    let mut draw_buffer: Vec<char> = Vec::with_capacity(40 * 6);
    let mut crt_buffer: Vec<char> = Vec::with_capacity(40);

    let mut ins_iter = cmds.iter();
    let mut add_reg: i32 = 0;
    loop {
        println!("{} {crt_position}", crt_buffer.iter().fold("".to_string(), |mut acc, x| {acc.push(*x); acc}));

        if crt_buffer.len() == crt_buffer.capacity() {
            draw_buffer.extend(crt_buffer.iter());
            crt_buffer.clear();
        }

        if add_reg != 0 {
            // crt
            crt_draw(&mut crt_position, &sprite_position, &mut crt_buffer);

            // end of cycle (finish adding)
            sprite_position = (sprite_position as i32 + add_reg) as u32;
            add_reg = 0;
            clock += 1;
            continue;
        }

        match ins_iter.next() {
            Some((ins, arg)) => {
                // begin instructions
                match ins {
                    &1 => {
                        // begin add
                        add_reg = *arg;
                    }
                    _ => {} // noop
                }
                // crt
                crt_draw(&mut crt_position, &sprite_position, &mut crt_buffer);
                // end cycle
                clock += 1;
            },
            _ => { break }
        }
    }

    // print out vec
    for r in 0..6 {
        for c in 0..40 {
            print!("{}", draw_buffer[r * 40 + c]);
        }
        print!("\n");
    }
    println!();
}

fn crt_draw(crt_position: &mut u32, sprite_position: &u32, crt_buffer: &mut Vec<char>) {
    if *crt_position >= sprite_position.max(&1) - 1 && *crt_position <= sprite_position + 1 {
        // within sprite boundaries
        crt_buffer.push('#');
    } else {
        crt_buffer.push('.');
    }
    *crt_position = (*crt_position + 1) % 40; // clamp at 40
}

fn part1(cmds: &Vec<(u32, i32)>) {
    let mut sum = 0;

    let mut reg_x = 1; // Register X (addx)
    let mut clock = 1;
    for (ins, arg) in cmds {
        clock += 1;
        match ins {
            &1 => {
                // Addx
                clock_sum(clock, reg_x, &mut sum);
                reg_x += arg;
                clock += 1; // Consume an aditional clock cycle
            }
            _ => { // Noop
            }
        }
        clock_sum(clock, reg_x, &mut sum);
    }
    println!("Sum: {}", sum);
}

#[inline]
fn clock_sum(clock: u32, reg_x: i32, sum: &mut i32) {
    if (clock + 20) % 40 == 0 {
        *sum += clock as i32 * reg_x;
    }
}