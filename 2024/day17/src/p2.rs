pub type Reg = u64;

pub fn parse(input: &str) -> Vec<u8> {
    let mut lines = input.lines();
    lines.nth(2);

    let (_, prog) = lines.nth(1).unwrap().split_once(':').unwrap();
    let prog: Vec<_> = prog
        .trim()
        .split(',')
        .map(|e| e.parse::<u8>().unwrap())
        .collect();
    prog
}

#[inline]
fn combo_operand(reg: [Reg; 3], operand: u8) -> Reg {
    match operand {
        x @ 0..=3 => x as Reg,
        x @ 4..=6 => reg[x as usize - 4],
        _ => unreachable!("invalid operand"),
    }
}

#[inline]
fn mod8(n: Reg) -> Reg {
    n & 0b111
}
/// Returns whether the output of the machine matches the program
fn simulate(reg_a: Reg, program: &[u8]) -> bool {
    let mut reg = [reg_a, 0, 0];
    let mut ins_ptr = 0;
    let mut out_ptr = 0;

    while ins_ptr + 1 < program.len() && out_ptr < program.len() {
        let opcode = program[ins_ptr];
        let operand = program[ins_ptr + 1];

        match opcode {
            // adv
            0 => reg[0] >>= combo_operand(reg, operand),
            // bxl
            1 => reg[1] ^= operand as Reg,
            // bst
            2 => reg[1] = mod8(combo_operand(reg, operand)),
            // jnz
            3 => {
                if reg[0] != 0 {
                    ins_ptr = operand as usize;
                    continue;
                }
            }
            // bxc
            4 => reg[1] ^= reg[2],
            // out
            5 => {
                let output = mod8(combo_operand(reg, operand)) as u8;
                if program[out_ptr] != output {
                    return false;
                }
                out_ptr += 1;
            }
            // bdv
            6 => reg[1] = reg[0] >> combo_operand(reg, operand),
            // cdv
            7 => reg[2] = reg[0] >> combo_operand(reg, operand),
            _ => unreachable!("invalid opcode"),
        }
        ins_ptr += 2;
    }
    out_ptr == program.len()
}

fn search_manual(out_ptr: usize, program: &[u8], reg_a: Reg, found: &mut Vec<Reg>) {
    let out = program[out_ptr];

    for b in 0..8 {
        let a = reg_a << 3 | b;
        let b = b ^ 3;
        let c = a >> b;
        let b = b ^ c;
        let b = b ^ 5;
        let b = (b & 0b111) as u8; // Mod 8

        if out != b {
            continue;
        }

        if out_ptr == 0 {
            // No more to search
            found.push(a);
            continue;
        }
        search_manual(out_ptr - 1, program, a, found);
    }
}

pub fn part2_manual(input: &str) -> Reg {
    let program = parse(input);
    let mut output = vec![];
    search_manual(program.len() - 1, &program, 0, &mut output);
    *output.iter().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manual() {
        let input = include_str!("input.txt");
        // Value acquired after solving the puzzle
        let solution = part2_manual(input);
        dbg!(solution);
        assert!(simulate(solution, &parse(input)));
    }

    #[test]
    fn simul() {
        assert!(simulate(117440, &[0, 3, 5, 4, 3, 0]));
    }
}
