pub type Reg = u64;
pub fn parse(input: &str) -> ([Reg; 3], Vec<u8>) {
    let mut lines = input.lines();
    let (_, ra) = lines.next().unwrap().split_once(':').unwrap();
    let (_, rb) = lines.next().unwrap().split_once(':').unwrap();
    let (_, rc) = lines.next().unwrap().split_once(':').unwrap();
    let (ra, rb, rc) = (
        ra.trim().parse::<Reg>().unwrap(),
        rb.trim().parse::<Reg>().unwrap(),
        rc.trim().parse::<Reg>().unwrap(),
    );

    let (_, prog) = lines.nth(1).unwrap().split_once(':').unwrap();
    let prog: Vec<_> = prog
        .trim()
        .split(',')
        .map(|e| e.parse::<u8>().unwrap())
        .collect();
    ([ra, rb, rc], prog)
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

pub fn part1(input: &str) -> String {
    let (mut reg, program) = parse(input);
    let mut ins_ptr = 0;

    let mut output = vec![];

    while ins_ptr < program.len() {
        let opcode = program[ins_ptr];
        // Istg if this panics because OOB
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
            5 => output.push(mod8(combo_operand(reg, operand)) as u8),
            // bdv
            6 => reg[1] = reg[0] >> combo_operand(reg, operand),
            // cdv
            7 => reg[2] = reg[0] >> combo_operand(reg, operand),
            _ => unreachable!("invalid opcode"),
        }
        ins_ptr += 2;
    }

    output
        .iter()
        .map(|e| format!("{}", *e))
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!(part1(input), "4,6,3,5,6,3,5,2,1,0");
    }

    /// This is just to make sure I do not break the algorithm despite refactorings.
    /// This test is written after my completion of part 1.
    #[test]
    fn actual_result() {
        let input = include_str!("input.txt");
        assert_eq!(part1(input), "1,3,5,1,7,2,5,1,6");
    }
}
