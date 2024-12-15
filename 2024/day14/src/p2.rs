pub mod naive {
    use std::fs::OpenOptions;
    use std::io::Write;

    type Int = i32;
    type Vec2 = aocutils::cartes::dim2::Vec2<Int>;

    fn parse_vector(input: &str) -> Vec2 {
        let (x, y) = input.split_once('=').unwrap().1.split_once(',').unwrap();
        From::from((x.trim().parse().unwrap(), y.trim().parse().unwrap()))
    }
    fn parse_line(input: &str) -> (Vec2, Vec2) {
        let (p, v) = input.split_once(' ').unwrap();
        let (p, v) = (parse_vector(p), parse_vector(v));
        (p, v)
    }
    fn parse_input(input: &str) -> (Vec<Vec2>, Vec<Vec2>) {
        let lines = input.lines().count();
        let mut positions = vec![Default::default(); lines];
        let mut velocities = vec![Default::default(); lines];

        for (i, line) in input.lines().enumerate() {
            let (p, v) = parse_line(line.trim());
            positions[i] = p;
            velocities[i] = v;
        }
        (positions, velocities)
    }
    pub fn part2(input: &str) {
        let (b1, b2): (Vec2, Vec2) = if cfg!(feature = "p1sample") {
            (From::from((0, 0)), From::from((11, 7)))
        } else {
            (From::from((0, 0)), From::from((101, 103)))
        };

        let (mut positions, velocities) = parse_input(input);

        let mut fpath = std::env::current_dir().unwrap();
        fpath.push("output.txt");
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(cfg!(feature = "p2writefile"))
            .open(fpath)
            .unwrap();

        let cols = b2.0 as usize;
        let rows = b2.1 as usize;
        let c1 = cols + 1;

        const TRIES: usize = 10_000;
        let mut buffer: Vec<u8> = vec![[vec![b'.'; cols], vec![b'\n']]; rows]
            .iter()
            .flatten()
            .flatten()
            .copied()
            .collect();
        for i in 1..=TRIES {
            positions
                .iter_mut()
                .zip(velocities.iter())
                .for_each(|(p, v)| *p = (*p + *v).bounds_wrap(b1, b2));

            place_positions(&mut buffer, &positions, c1, b'#');
            if cfg!(feature = "p2writefile") {
                let _ = write!(file, "\n{}\n", i);
                let _ = file.write_all(&buffer);
            } else {
                display_buf(&buffer, c1);
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            place_positions(&mut buffer, &positions, c1, b'.');
        }
    }

    pub fn display_buf(buf: &[u8], cols: usize) {
        print!("\u{001b}[{};{}H", 0, 0);
        for (i, b) in buf.iter().enumerate() {
            if i % cols == 0 {
                println!();
            }
            print!("{}", *b as char);
        }
    }

    fn place_positions(buf: &mut [u8], positions: &[Vec2], cols: usize, char: u8) {
        for pos in positions {
            buf[pos.1 as usize * cols + pos.0 as usize] = char;
        }
    }
}
