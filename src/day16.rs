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
    pub fn solve(&self, registers: &mut Vec<i32>) {
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
        }
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

fn extract_sample_entry() {}

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
    }
}
