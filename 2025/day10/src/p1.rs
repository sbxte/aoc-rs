use std::collections::BinaryHeap;

use aocutils::linalg::matrix::{Matrix, SquareMatrix};

fn parse_button(button: &str) -> impl Iterator<Item = usize> {
    button[1..button.len() - 1]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
}
fn parse_lights(lights: &str) -> impl Iterator<Item = bool> {
    lights[1..lights.len() - 1].chars().map(|c| c == '#')
}

fn try_matrix(buttons: &str, lights_buf: &mut Vec<i32>) -> Option<i32> {
    let lights_n = lights_buf.len();

    let buttons = buttons.trim().split(' ');
    let buttons_n = buttons.clone().count(); // Adjust sizes to make square matrix
    let mut extra_btns = 0;
    if lights_n < buttons_n {
        for _ in lights_n..buttons_n {
            lights_buf.push(0);
        }
    } else if buttons_n < lights_n {
        for _ in buttons_n..lights_n {
            extra_btns += 1;
        }
    }
    let extra_lights = lights_buf.len() - lights_n;
    assert_eq!(lights_n + extra_lights, buttons_n + extra_btns);

    // Target-lights matrix
    let mut light_matrix = Matrix::new(0, lights_n + extra_lights, 1);
    for (idx, light) in lights_buf.iter().enumerate() {
        *light_matrix.get_mut(idx, 0) = *light;
    }

    // Button-system matrix
    let mut matrix = SquareMatrix::from(Matrix::new(
        0,
        lights_n + extra_lights,
        buttons_n + extra_btns,
    ));
    for (i, button) in buttons.enumerate() {
        for j in 0..lights_n {
            *matrix.get_mut(j, i) = 0;
        }
        for light in parse_button(button) {
            *matrix.get_mut(light, i) = 1;
        }
    }
    for r in 0..lights_n {
        for c in buttons_n..buttons_n + extra_btns {
            *matrix.get_mut(r, c) = 1;
        }
    }
    for r in lights_n..lights_n + extra_lights {
        for c in 0..buttons_n + extra_btns {
            *matrix.get_mut(r, c) = 1;
        }
    }

    if let Some(inversion) = matrix.get_inverted() {
        let mut solution = inversion * light_matrix;
        solution.map(|x| x & 1); // modulo 2, binary 

        let mut presses = 0;
        for r in 0..lights_n {
            presses += solution.get(r, 0);
        }
        Some(presses)
    } else {
        None
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct BfsState {
    light_state: Vec<i32>,
    depth: usize,
}
impl PartialOrd for BfsState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}
impl Ord for BfsState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.depth.cmp(&other.depth).reverse()
    }
}
fn try_bfs(button: &str, lights: &mut Vec<i32>) -> i32 {
    let mut open = BinaryHeap::new();
    open.push(BfsState {
        light_state: vec![0; lights.len()],
        // Equivalent to number of button presses to get here
        depth: 0,
    });

    let button_itr = button.trim().split(' ').map(|b| parse_button(b));

    while let Some(state) = open.pop() {
        if state.light_state == lights.as_slice() {
            return state.depth as i32;
        }

        for button in button_itr.clone() {
            let mut next = state.clone();
            for light_toggled in button {
                // 0 -> 1 
                // 1 -> 0
                next.light_state[light_toggled] = 1 - next.light_state[light_toggled];
            }
            next.depth += 1;
            open.push(next);
        }
    }
    unreachable!("BFS somehow ran out of moves...");
}

pub fn part1(input: &str) -> i32 {
    let mut presses = 0;
    let mut lights = vec![];
    for line in input.lines() {
        let (lights_str, line) = line.split_once(' ').unwrap();
        let (buttons, _) = line.split_at(line.rfind(' ').unwrap());

        // Parse target light state
        lights.clear();
        for light in parse_lights(lights_str) {
            lights.push(light as i32);
        }
        if let Some(p) = try_matrix(buttons, &mut lights) {
            presses += p;
            continue;
        }

        // Non matrix approach
        // Brute force this thing?
        lights.clear();
        for light in parse_lights(lights_str) {
            lights.push(light as i32);
        }
        presses += try_bfs(buttons, &mut lights);
    }
    presses
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn sample() {
        let input = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
        assert_eq!(part1(input), 7);
    }
}
