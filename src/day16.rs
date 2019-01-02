use fnv::FnvHashMap;
use std::str::Lines;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

pub struct Instruction {
    opcode: Opcode,
    input_a: usize,
    input_b: usize,
    output: usize,
}

impl Instruction {
    pub fn solve(&self, mut registers: Vec<i32>) -> Vec<i32> {
        registers[self.output] = match self.opcode {
            Opcode::Addr => registers[self.input_a] + registers[self.input_b],
            Opcode::Addi => registers[self.input_a] + self.input_b as i32,
            Opcode::Mulr => registers[self.input_a] * registers[self.input_b],
            Opcode::Muli => registers[self.input_a] * self.input_b as i32,
            Opcode::Banr => registers[self.input_a] & registers[self.input_b],
            Opcode::Bani => registers[self.input_a] & self.input_b as i32,
            Opcode::Borr => registers[self.input_a] | registers[self.input_b],
            Opcode::Bori => registers[self.input_a] | self.input_b as i32,
            Opcode::Setr => registers[self.input_a],
            Opcode::Seti => self.input_a as i32,
            Opcode::Gtir if self.input_a as i32 > registers[self.input_b] => 1,
            Opcode::Gtri if registers[self.input_a] > self.input_b as i32 => 1,
            Opcode::Gtrr if registers[self.input_a] > registers[self.input_b] => 1,
            Opcode::Eqir if self.input_a as i32 == registers[self.input_b] => 1,
            Opcode::Eqri if registers[self.input_a] == self.input_b as i32 => 1,
            Opcode::Eqrr if registers[self.input_a] == registers[self.input_b] => 1,
            _ => 0,
        };
        registers
    }

    pub fn new(opcode: Opcode, a: usize, b: usize, output: usize) -> Instruction {
        Instruction {
            opcode,
            input_a: a,
            input_b: b,
            output,
        }
    }

    pub fn all(a: usize, b: usize, output: usize) -> Vec<Instruction> {
        vec![
            Instruction::new(Opcode::Addr, a, b, output),
            Instruction::new(Opcode::Addi, a, b, output),
            Instruction::new(Opcode::Mulr, a, b, output),
            Instruction::new(Opcode::Muli, a, b, output),
            Instruction::new(Opcode::Banr, a, b, output),
            Instruction::new(Opcode::Bani, a, b, output),
            Instruction::new(Opcode::Borr, a, b, output),
            Instruction::new(Opcode::Bori, a, b, output),
            Instruction::new(Opcode::Setr, a, b, output),
            Instruction::new(Opcode::Seti, a, b, output),
            Instruction::new(Opcode::Gtir, a, b, output),
            Instruction::new(Opcode::Gtri, a, b, output),
            Instruction::new(Opcode::Gtrr, a, b, output),
            Instruction::new(Opcode::Eqri, a, b, output),
            Instruction::new(Opcode::Eqir, a, b, output),
            Instruction::new(Opcode::Eqrr, a, b, output),
        ]
    }
}

pub struct Registers;

impl Registers {
    pub fn from_blank_line(input: &str) -> Vec<i32> {
        input
            .split(" ")
            .filter_map(|x| x.parse::<i32>().ok())
            .collect()
    }

    pub fn from_before(input: &str) -> Vec<i32> {
        Registers::from_blank_line(
            &input
                .replace("Before: [", "")
                .replace("]", "")
                .replace(",", ""),
        )
    }

    pub fn from_after(input: &str) -> Vec<i32> {
        Registers::from_blank_line(
            &input
                .replace("After:  [", "")
                .replace("]", "")
                .replace(",", ""),
        )
    }

    pub fn parse_line(input: &str) -> Vec<i32> {
        match input {
            ll if ll.starts_with("Before") => Registers::from_before(ll),
            ll if ll.starts_with("After") => Registers::from_after(ll),
            _ => Registers::from_blank_line(input),
        }
    }
}

fn extract_sample_entry_part_one(iter_lines: &mut Lines) -> Option<Vec<Vec<i32>>> {
    let line = iter_lines.next();
    if line.is_none() {
        None
    } else {
        if let Some(l) = line {
            if l == "" {
                return None;
            }
        }
        let line_two = iter_lines.next();
        let line_three = iter_lines.next();
        Some(vec![
            Registers::parse_line(line.unwrap()),
            Registers::parse_line(line_two.unwrap()),
            Registers::parse_line(line_three.unwrap()),
        ])
    }
}

fn find_opcodes(input: Vec<Vec<Vec<i32>>>) -> FnvHashMap<usize, Opcode> {
    let mut matched: Vec<usize> = vec![];
    Instruction::all(0, 0, 0)
        .into_iter()
        .map(|r| {
            (
                (0..16usize)
                    .find(|ii| {
                        if matched.contains(ii) {
                            return false;
                        }

                        let val = input
                            .iter()
                            .filter(|inp| inp[1][0] as usize == *ii)
                            .all(|inp| {
                                Instruction::new(
                                    r.opcode,
                                    inp[1][1] as usize,
                                    inp[1][2] as usize,
                                    inp[1][3] as usize,
                                )
                                .solve(inp[0].clone())
                                    == inp[2]
                            });
                        if val {
                            matched.push(*ii);
                        }
                        val
                    })
                    .expect(&format!(
                        "Failed to find corresponding entry for {:?}",
                        r.opcode
                    )),
                r.opcode,
            )
        })
        .collect()
}

#[aoc(day16, part1)]
fn part_one(input: &str) -> usize {
    let mut lines = input.lines();
    let mut registers: Vec<Vec<Vec<i32>>> = vec![];

    (0..)
        .find(|_| match extract_sample_entry_part_one(&mut lines) {
            Some(regs) => {
                registers.push(regs);
                let _line = lines.next();
                false
            }
            _ => true,
        })
        .unwrap();

    registers
        .iter()
        .filter(|r| {
            Instruction::all(r[1][1] as usize, r[1][2] as usize, r[1][3] as usize)
                .iter()
                .filter(|rr| rr.solve(r[0].clone()) == r[2])
                .count()
                >= 3
        })
        .count()
}

#[aoc(day16, part2)]
fn part_two(input: &str) -> i32 {
    let mut lines = input.lines();
    let mut registers: Vec<Vec<Vec<i32>>> = vec![];

    (0..)
        .find(|_| match extract_sample_entry_part_one(&mut lines) {
            Some(regs) => {
                registers.push(regs);
                let _line = lines.next();
                false
            }
            _ => true,
        })
        .unwrap();

    let rules = find_opcodes(registers);
    println!("{:?}", rules);
    lines
        .filter_map(|l| {
            let rgs = Registers::parse_line(l);
            match rgs.len() {
                4 => Some(Instruction::new(
                    *rules
                        .get(&(rgs[0] as usize))
                        .expect(&format!("Failed to find rule for {}", rgs[0])),
                    rgs[1] as usize,
                    rgs[2] as usize,
                    rgs[3] as usize,
                )),
                _ => None,
            }
        })
        .fold(vec![0, 0, 0, 0], |acc, i| i.solve(acc))[0]
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn day16_test_parsing() {
        let input: &str = "Before: [0, 1, 2, 1]\n12 3 2 2\nAfter:  [0, 1, 1, 1]";
        let registers_stack: Vec<Vec<i32>> = input.lines().map(Registers::parse_line).collect();

        assert_eq!(registers_stack[0], vec![0, 1, 2, 1]);
        assert_eq!(registers_stack[1], vec![12, 3, 2, 2]);
        assert_eq!(registers_stack[2], vec![0, 1, 1, 1]);

        let mut lines = input.lines();
        let registers_stack = extract_sample_entry_part_one(&mut lines).expect("Failed to extract");

        assert_eq!(registers_stack[0], vec![0, 1, 2, 1]);
        assert_eq!(registers_stack[1], vec![12, 3, 2, 2]);
        assert_eq!(registers_stack[2], vec![0, 1, 1, 1]);
    }
}
