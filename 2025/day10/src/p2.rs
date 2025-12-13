use std::collections::{BinaryHeap, HashSet};

use aocutils::linalg::matrix::Matrix;
use aocutils::linalg::matrix::row_operations::{div, mul_sub, swap};
use smallvec::{SmallVec, smallvec};

fn parse_button(button: &str) -> impl Iterator<Item = usize> {
    button[1..button.len() - 1]
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
}
fn parse_joltage(joltage_s: &str) -> impl Iterator<Item = i32> {
    joltage_s[1..joltage_s.len() - 1]
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
}

#[derive(Debug, PartialEq, Eq)]
pub enum GaussElimResult<N> {
    NoSolution,
    OneSolution(Matrix<N>),
    Multisolution(Matrix<N>, Matrix<N>),
}

/// Perform gaussian elimination using row echelon forms
/// If infinite solutions are found, it will be collapsed into one solution and returned
/// `Rhs` is a one column matrix
pub fn gauss_elim(mut matrix: Matrix<f64>, mut rhs: Matrix<f64>) -> GaussElimResult<f64> {
    assert_eq!(matrix.rows(), rhs.rows());
    assert_eq!(rhs.cols(), 1);

    // Forward elimination
    let mut curr_row = 0;
    let mut col_range_end = 0; // inclusive
    for curr_col in 0..matrix.cols() {
        if curr_row >= matrix.rows() {
            break;
        }
        col_range_end = curr_col;

        let non_zero_row = (curr_row..matrix.rows()).find_map(|r| {
            if *matrix.get(r, curr_col) == 0. {
                None
            } else {
                Some(r)
            }
        });
        if let Some(row) = non_zero_row {
            swap(&mut matrix, row, curr_row);
            swap(&mut rhs, row, curr_row);

            for row in (curr_row + 1)..matrix.rows() {
                let multiplier = *matrix.get(row, curr_col) / *matrix.get(curr_row, curr_col);
                mul_sub(&mut matrix, curr_row, row, multiplier);
                mul_sub(&mut rhs, curr_row, row, multiplier);
            }
            curr_row += 1;
        } // Otherwise, entire column is already zeros
    }
    matrix.map(|x| {
        if (x.round() - x).abs() <= 0.00001 {
            x.round()
        } else {
            x
        }
    });
    rhs.map(|x| {
        if (x.round() - x).abs() <= 0.00001 {
            x.round()
        } else {
            x
        }
    });

    if col_range_end + 1 >= matrix.cols() {
        // there exists be a solution
        // if and only if all zero-rows have zeros on the RHS
        let all_zeros = (curr_row..matrix.rows())
            .map(|r| rhs.get(r, 0))
            .all(|c| *c == 0.);
        if !all_zeros {
            return GaussElimResult::NoSolution;
        }
    }

    if curr_row == 0 {
        // The entire matrix is zero...
        let all_zeros = (0..matrix.rows()).map(|r| rhs.get(r, 0)).all(|c| *c == 0.);
        if !all_zeros {
            return GaussElimResult::NoSolution;
        } else {
            panic!("Zero matrix");
        }
    }

    let row_range_end = curr_row - 1; // inclusive

    // Back substitution
    for row in (0..=row_range_end).rev() {
        let non_zero_col = (0..=col_range_end).find_map(|c| {
            if matrix.get(row, c) == &0. {
                None
            } else {
                Some(c)
            }
        });

        if let Some(col) = non_zero_col {
            for r in 0..row {
                let multiplier = *matrix.get(r, col) / *matrix.get(row, col);
                mul_sub(&mut matrix, row, r, multiplier);
                mul_sub(&mut rhs, row, r, multiplier);
            }
            // Normalize
            let divisor = *matrix.get(row, col);
            div(&mut matrix, row, divisor);
            div(&mut rhs, row, divisor);
        } else {
            // already reached zero-row,
            // so stop
            break;
        }
    }
    matrix.map(|x| {
        if (x.round() - x).abs() <= 0.00001 {
            x.round()
        } else {
            x
        }
    });
    rhs.map(|x| {
        if (x.round() - x).abs() <= 0.00001 {
            x.round()
        } else {
            x
        }
    });

    // One solution
    if (0..matrix.cols()).all(|c| {
        (0..matrix.rows())
            .filter(|r| matrix.get(*r, c) == &0.)
            .count()
            + 1
            >= matrix.rows()
    }) {
        let mut m = Matrix::new(0., matrix.cols(), 1);
        for c in 0..matrix.cols() {
            if let Some(r) = (0..matrix.rows()).find(|r| matrix.get(*r, c) != &0.) {
                *m.get_mut(c, 0) = *rhs.get(r, 0);
            } else {
                *m.get_mut(c, 0) = 0.;
            }
        }
        return GaussElimResult::OneSolution(m);
    }
    GaussElimResult::Multisolution(matrix, rhs)
}

pub fn print_matrix<N: core::fmt::Debug>(matrix: &Matrix<N>) {
    for r in 0..matrix.rows() {
        for c in 0..matrix.cols() {
            print!("{:02?} ", matrix.get(r, c));
        }
        println!();
    }
}

fn matrix_ge(buttons: &str, jolt_counters: &mut Vec<i32>) -> GaussElimResult<f64> {
    let counters_n = jolt_counters.len();

    let buttons = buttons.trim().split(' ');
    let buttons_n = buttons.clone().count();

    // Target-joltage matrix
    let mut joltage_matrix = Matrix::new(0., counters_n, 1);
    for (idx, target_jolt) in jolt_counters.iter().enumerate() {
        *joltage_matrix.get_mut(idx, 0) = *target_jolt as f64;
    }

    // Button-system matrix
    let mut matrix = Matrix::new(0., counters_n, buttons_n);
    for (c, button) in buttons.enumerate() {
        for r in 0..counters_n {
            *matrix.get_mut(r, c) = 0.;
        }
        for jcounter_idx in parse_button(button) {
            *matrix.get_mut(jcounter_idx, c) = 1.;
        }
    }
    gauss_elim(matrix, joltage_matrix)
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
struct LegalitySearchState {
    free_btns: SmallVec<[i32; 10]>,
    depth: i32,
    negativity: i32,
}
impl LegalitySearchState {
    fn update_state(
        &mut self,
        dependent_cols: &[(usize, usize)],
        free_cols: &[usize],
        matrix: &Matrix<f64>,
        rhs: &Matrix<f64>,
    ) {
        let mut buttons: SmallVec<[f64; 10]> = smallvec![0.; 16];

        // Calculate state
        for (dcol, row) in dependent_cols.iter() {
            let mut value = *rhs.get(*row, 0);
            for (i, fcol) in free_cols.iter().enumerate() {
                value = -matrix
                    .get(*row, *fcol)
                    .mul_add(self.free_btns[i] as f64, -value);
            }

            buttons[*dcol] = value;
        }
        for (i, fcol) in free_cols.iter().enumerate() {
            buttons[*fcol] = self.free_btns[i] as f64;
        }

        let negativity = buttons
            .iter()
            .fold(0., |acc, x| if *x < 0. { acc - x } else { acc });

        self.negativity = negativity.ceil() as i32;
    }
}
impl Ord for LegalitySearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let neg = self.negativity.cmp(&other.negativity).reverse();
        let depth = self.depth.cmp(&other.depth).reverse();
        neg.then(depth)
    }
}
impl PartialOrd for LegalitySearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

#[derive(PartialEq, Eq, Clone, Debug, Default)]
struct MinPressesSearchState {
    free_btns: SmallVec<[i32; 10]>,
    depth: i32,
    whole: bool,
    presses: i32,
    positive: bool,
}
impl MinPressesSearchState {
    fn update_state(
        &mut self,
        dependent_cols: &[(usize, usize)],
        free_cols: &[usize],
        matrix: &Matrix<f64>,
        rhs: &Matrix<f64>,
    ) {
        let mut buttons: SmallVec<[f64; 10]> = smallvec![0.; 16];

        // Calculate state
        for (dcol, row) in dependent_cols.iter() {
            let mut value = *rhs.get(*row, 0);
            for (i, fcol) in free_cols.iter().enumerate() {
                value = -matrix
                    .get(*row, *fcol)
                    .mul_add(self.free_btns[i] as f64, -value);
            }

            buttons[*dcol] = value;
        }
        for (i, fcol) in free_cols.iter().enumerate() {
            buttons[*fcol] = self.free_btns[i] as f64;
        }
        buttons.iter_mut().for_each(|b| if (*b - b.round()) <= 0.01 { *b = b.round() });

        // println!("{:?}", &self);
        // println!("{:?}", &buttons);

        let presses = buttons.iter().fold(0., |acc, b| acc + b);
        let positive = buttons.iter().all(|b| *b >= 0.);
        let whole = buttons.iter().all(|b| (b.round() - b).abs() <= 0.01);

        self.presses = presses.ceil() as i32;
        self.positive = positive;
        self.whole = whole;
    }
}
impl Ord for MinPressesSearchState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let press = self.presses.cmp(&other.presses).reverse();
        let depth = self.depth.cmp(&other.depth).reverse();
        press.then(depth)
    }
}
impl PartialOrd for MinPressesSearchState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

fn bfs_matrix(matrix: Matrix<f64>, rhs: Matrix<f64>) -> u32 {
    // Determine which variables (buttons) are free to run BFS on
    let mut dependent_cols: SmallVec<[(usize, usize); 10]> = smallvec![];
    let mut free_cols: SmallVec<[usize; 10]> = smallvec![];
    for c in 0..matrix.cols() {
        let zeros = (0..matrix.rows())
            .filter(|r| *matrix.get(*r, c) == 0.)
            .count();
        if zeros >= matrix.rows() {
            // Zeroed column
            // I.e. button is not necessary to press
            continue;
        } else if zeros + 1 >= matrix.rows() {
            // One non zero element
            let row = (0..matrix.rows())
                .find(|r| *matrix.get(*r, c) != 0.)
                .unwrap();
            dependent_cols.push((c, row));
        } else {
            free_cols.push(c);
        }
    }

    // Find legal state first, and then search from there
    let mut open = BinaryHeap::new();
    let mut init_state = LegalitySearchState {
        free_btns: smallvec![0; matrix.cols()],
        depth: 0,
        negativity: 0,
        ..Default::default()
    };
    init_state.update_state(&dependent_cols, &free_cols, &matrix, &rhs);
    open.push(init_state);

    let mut visited_states = HashSet::new();
    let mut min_presses = u32::MAX;

    let mut legal = None;
    while let Some(state) = open.pop() {
        // Ignore duplicates
        if visited_states.contains(&state.free_btns) {
            continue;
        }
        visited_states.insert(state.free_btns.clone());

        if state.negativity <= 0 {
            legal = Some(state);
            break;
        }

        for i in 0..free_cols.len() {
            let mut next = state.clone();
            next.free_btns[i] += 1;
            next.update_state(&dependent_cols, &free_cols, &matrix, &rhs);
            next.depth += 1;
            open.push(next);
        }
    }

    // Actual search
    let mut open = BinaryHeap::new();
    if let Some(state) = legal {
        let mut new_state = MinPressesSearchState {
            free_btns: state.free_btns, 
            depth: state.depth,
            ..Default::default()
        };
        new_state.update_state(&dependent_cols, &free_cols, &matrix, &rhs);
        open.push(new_state);
        visited_states.clear();
    } else {
        panic!("Unable to find legal state");
    }

    // Allow movement in all directions now that we're in legal (non negative) state space
    while let Some(state) = open.pop() {
        // Ignore duplicates
        if visited_states.contains(&state.free_btns) {
            continue;
        }
        visited_states.insert(state.free_btns.clone());

        if state.whole {
            min_presses = min_presses.min(state.presses as u32);
        }

        for i in 0..free_cols.len() {
            for j in [-1, 1] {
                let mut next = state.clone();
                next.free_btns[i] += j;
                if !(0..=300).contains(&next.free_btns[i]) {
                    continue;
                }

                next.update_state(&dependent_cols, &free_cols, &matrix, &rhs);

                if !next.positive {
                    // Do not go back into negatives
                    continue; 
                }

                next.depth += 1;
                open.push(next);
            }
        }
    }

    if min_presses > 10_000 {
        panic!("Found exorbitantly large min_presses: {} / {}", min_presses, u32::MAX);
    }
    min_presses
}

pub fn part2(input: &str) -> u64 {
    let mut presses = 0;
    let mut joltages = vec![];
    for (li, line) in input.lines().enumerate() {
        let (_lights_str, line) = line.split_once(' ').unwrap();
        let (buttons_str, joltage_str) = line.split_at(line.rfind(' ').unwrap());
        let joltage_str = joltage_str.trim();

        // Matrix approach
        joltages.clear();
        for jolt in parse_joltage(joltage_str) {
            joltages.push(jolt as i32);
        }

        let ge = matrix_ge(buttons_str, &mut joltages);
        let p = match ge {
            GaussElimResult::NoSolution => panic!("No solution found"),
            GaussElimResult::OneSolution(m) => {
                let mut sum = 0;
                println!("{:?}", m);
                println!("Gaussian Elimination found a unique solution!");
                for i in 0..m.rows() {
                    sum += m.get(i, 0).round() as u64;
                }
                sum
            }
            GaussElimResult::Multisolution(mat, rhs) => {
                // print_matrix(&mat);
                // print_matrix(&rhs);
                bfs_matrix(mat, rhs) as u64
            }
        };

        presses += p;
        println!("({}): p: {} s: {}", li, p, presses);
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
        assert_eq!(part2(input), 33);
    }
}
