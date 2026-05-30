use std::collections::{HashMap, VecDeque};

type AXISINT = i64;
type COORD = (AXISINT, AXISINT);

#[derive(Hash, Clone, Copy, PartialEq, Eq)]
enum Rock {
    Horizontal,
    Plus,
    Corner,
    Vertical,
    Square,
}
impl Rock {
    fn next(self) -> Self {
        match self {
            Self::Horizontal => Self::Plus,
            Self::Plus => Self::Corner,
            Self::Corner => Self::Vertical,
            Self::Vertical => Self::Square,
            Self::Square => Self::Horizontal,
        }
    }

    fn get_shape(&self) -> &[COORD] {
        match self {
            Self::Horizontal => &[(0, 0), (1, 0), (2, 0), (3, 0)],
            Self::Plus => &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
            Self::Corner => &[(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Self::Vertical => &[(0, 0), (0, 1), (0, 2), (0, 3)],
            Self::Square => &[(0, 0), (0, 1), (1, 0), (1, 1)],
        }
    }

    fn can_fall(&self, (px, py): COORD, chambers: &Chambers, minh: AXISINT) -> bool {
        for (psx, psy) in self.get_shape() {
            if py + psy - minh == 0 || chambers[(py + psy - 1 - minh) as usize][(px + psx) as usize]
            {
                return false;
            }
        }
        true
    }

    fn can_move(
        &self,
        (px, py): COORD,
        chambers: &Chambers,
        nudge: AXISINT,
        minh: AXISINT,
    ) -> bool {
        for (psx, psy) in self.get_shape() {
            let newx = px + psx + nudge;
            if !(0..7).contains(&newx) {
                return false;
            }
            if chambers[(py + psy - minh) as usize][newx as usize] {
                return false;
            }
        }
        true
    }
}

type Chambers = VecDeque<[bool; 7]>;
const STATE_HEIGHT: usize = 128;

#[derive(Hash, PartialEq, Eq)]
struct MemoState {
    board: [bool; 7 * STATE_HEIGHT],
    rock: Rock,
    jeti: usize,
}

fn calc_jump_mul(delta: u64, remlen: u64) -> u64 {
    if delta >= remlen {
        return 0;
    }
    remlen / delta
}

pub fn part2(input: &str) -> AXISINT {
    let mut rock = Rock::Horizontal;
    let mut rocks: u64 = 0;

    let jets: Vec<_> = input.trim().chars().collect();
    let mut jeti = 0;

    let mut chambers: Chambers = VecDeque::new();
    let mut heights: [AXISINT; 7] = [0; 7];
    const CAPGROWTH: AXISINT = 10;
    let mut caph = CAPGROWTH;
    let mut minh = 0;
    for _ in 0..caph {
        chambers.push_back([false; 7]);
    }
    let mut pos: COORD = (2, 3);

    let mut maxlen = CAPGROWTH as usize;

    const MAXROCKS: u64 = 1000000000000;
    // const MAXROCKS: u64 = 2022;
    let mut memo = HashMap::new();
    let mut jumped_height = 0;

    while rocks < MAXROCKS {
        {
            let mut board: [bool; 7 * STATE_HEIGHT] = [false; 7 * STATE_HEIGHT];
            let mut collected = chambers.iter().copied().collect::<Vec<_>>();
            for _ in collected.len()..STATE_HEIGHT {
                collected.push([false; 7]);
            }
            collected.iter().enumerate().for_each(|(y, row)| {
                row.iter()
                    .enumerate()
                    .for_each(|(x, b)| board[y * 7 + x] = *b)
            });

            let state = MemoState { board, rock, jeti };

            let maxh = *heights.iter().max().unwrap();
            if let Some((rockp, maxhp)) = memo.get(&state) {
                let delta = rocks - rockp;
                let remlen = MAXROCKS - rocks;
                let jump_mul = calc_jump_mul(delta, remlen);
                // println!(
                //     "CYCLE!!!!!! {} from {}, delta {}, jump {}",
                //     rocks, rockp, delta, jump_mul
                // );
                rocks += jump_mul * delta;
                jumped_height += (maxh - maxhp) * jump_mul as i64;
            } else {
                memo.insert(state, (rocks, maxh));
            }
        }
        let nudge = if jets[jeti] == '>' { 1 } else { -1 };

        if rock.can_move(pos, &chambers, nudge, minh) {
            pos.0 += nudge;
        }

        if rock.can_fall(pos, &chambers, minh) {
            pos.1 -= 1;
        } else {
            for (psx, psy) in rock.get_shape() {
                heights[(pos.0 + psx) as usize] =
                    heights[(pos.0 + psx) as usize].max(pos.1 + psy + 1);
                chambers[(pos.1 + psy - minh) as usize][(pos.0 + psx) as usize] = true;
            }

            // Shrink
            let next_minh = *heights.iter().min().unwrap();
            for _ in minh..next_minh {
                chambers.pop_front();
            }
            minh = next_minh;

            let next_maxh = *heights.iter().max().unwrap();
            pos = (2, next_maxh + 3);

            // Expand
            for _ in caph..(next_maxh + CAPGROWTH) {
                chambers.push_back([false; 7]);
            }
            caph = next_maxh + CAPGROWTH;

            rock = rock.next();
            rocks += 1;
        }
        jeti = (jeti + 1) % jets.len();
        maxlen = maxlen.max(chambers.len());
    }

    *heights.iter().max().unwrap() + jumped_height
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(part2(input), 1514285714288);
    }
}
