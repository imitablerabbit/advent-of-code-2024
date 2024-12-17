use itertools::Itertools;
use std::{fs::File, io::Read};

enum Instruction {
    Adv(i32),
    Bxl(i32),
    Bst(i32),
    Jnz(i32),
    Bxc(i32),
    Out(i32),
    Bdv(i32),
    Cdv(i32),
}

struct Computer {
    register_a: i32,
    register_b: i32,
    register_c: i32,

    program: Vec<i32>,
    ip: usize,

    output: Vec<i32>,
}

impl Computer {
    fn run(&mut self) -> Option<()> {
        while let Some(instruction) = self.next_instruction() {
            match instruction {
                Instruction::Adv(operand) => self.adv(operand)?,
                Instruction::Bxl(operand) => self.bxl(operand),
                Instruction::Bst(operand) => self.bst(operand)?,
                Instruction::Jnz(operand) => self.jnz(operand),
                Instruction::Bxc(operand) => self.bxc(operand),
                Instruction::Out(operand) => self.out(operand)?,
                Instruction::Bdv(operand) => self.bdv(operand)?,
                Instruction::Cdv(operand) => self.cdv(operand)?,
            }
        }
        Some(())
    }

    fn next_instruction(&mut self) -> Option<Instruction> {
        let opcode = self.program.get(self.ip)?;
        let operand = self.program.get(self.ip + 1)?;

        self.ip += 2;

        match opcode {
            0 => Some(Instruction::Adv(*operand)),
            1 => Some(Instruction::Bxl(*operand)),
            2 => Some(Instruction::Bst(*operand)),
            3 => Some(Instruction::Jnz(*operand)),
            4 => Some(Instruction::Bxc(*operand)),
            5 => Some(Instruction::Out(*operand)),
            6 => Some(Instruction::Bdv(*operand)),
            7 => Some(Instruction::Cdv(*operand)),
            _ => None,
        }
    }

    fn combo_operand_value(&self, operand: i32) -> Option<i32> {
        match operand {
            0 => Some(0),
            1 => Some(1),
            2 => Some(2),
            3 => Some(3),
            4 => Some(self.register_a),
            5 => Some(self.register_b),
            6 => Some(self.register_c),
            _ => None,
        }
    }

    fn adv(&mut self, operand: i32) -> Option<()> {
        let operand = self.combo_operand_value(operand)?;
        let numerator = self.register_a;
        let denominator = i32::pow(2, operand as u32);
        self.register_a = numerator / denominator; // integer division
        Some(())
    }

    fn bxl(&mut self, operand: i32) {
        let base = self.register_b;
        self.register_b = base ^ operand;
    }

    fn bst(&mut self, operand: i32) -> Option<()> {
        let operand = self.combo_operand_value(operand)?;
        self.register_b = operand % 8;
        Some(())
    }

    fn jnz(&mut self, operand: i32) {
        if self.register_a == 0 {
            return;
        }
        self.ip = operand as usize;
    }

    fn bxc(&mut self, _: i32) {
        let base = self.register_b;
        let operand = self.register_c;
        self.register_b = base ^ operand;
    }

    fn out(&mut self, operand: i32) -> Option<()> {
        let operand = self.combo_operand_value(operand)?;
        let value = operand % 8;
        self.output.push(value);
        Some(())
    }

    fn bdv(&mut self, operand: i32) -> Option<()> {
        let operand = self.combo_operand_value(operand)?;
        let numerator = self.register_a;
        let denominator = i32::pow(2, operand as u32);
        self.register_b = numerator / denominator; // integer division
        Some(())
    }

    fn cdv(&mut self, operand: i32) -> Option<()> {
        let operand = self.combo_operand_value(operand)?;
        let numerator = self.register_a;
        let denominator = i32::pow(2, operand as u32);
        self.register_c = numerator / denominator; // integer division
        Some(())
    }
}

/// Reads the contents of the input file and returns a result of the file contents.
///
/// # Arguments
///
/// * `puzzle_path` - A string slice that holds the path to the input file.
///
/// # Returns
///
/// * `Result<String, std::io::Error>` - The contents of the file as a string, or an error if the file could not be read.
fn read_to_string(puzzle_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(puzzle_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse(input: &str) -> Computer {
    let mut numbers = input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter_map(|s| s.parse::<i32>().ok());

    let registers: Vec<i32> = numbers.by_ref().take(3).collect();
    let program: Vec<i32> = numbers.collect();

    Computer {
        register_a: registers[0],
        register_b: registers[1],
        register_c: registers[2],
        program,
        ip: 0,
        output: Vec::new(),
    }
}

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let mut computer = parse(&puzzle_input);
    computer.run();
    let output = computer.output.iter().join(",");
    println!("The output is: {}", output);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_computer(
        program: Vec<i32>,
        register_a: i32,
        register_b: i32,
        register_c: i32,
    ) -> Computer {
        Computer {
            register_a,
            register_b,
            register_c,
            program,
            ip: 0,
            output: Vec::new(),
        }
    }

    #[test]
    fn test_adv() {
        let mut computer = setup_computer(vec![0, 2], 8, 0, 0);
        computer.run().unwrap();
        assert_eq!(computer.register_a, 2);
    }

    #[test]
    fn test_bxl() {
        let mut computer = setup_computer(vec![1, 3], 0, 5, 0);
        computer.run().unwrap();
        assert_eq!(computer.register_b, 6);
    }

    #[test]
    fn test_bst() {
        let mut computer = setup_computer(vec![2, 4], 0, 0, 0);
        computer.run().unwrap();
        assert_eq!(computer.register_b, 0);
    }

    #[test]
    fn test_jnz() {
        let mut computer = setup_computer(vec![3, 4, 0, 0, 0], 1, 0, 0);
        computer.run().unwrap();
        assert_eq!(computer.ip, 4);
    }

    #[test]
    fn test_bxc() {
        let mut computer = setup_computer(vec![4, 0], 0, 5, 3);
        computer.run().unwrap();
        assert_eq!(computer.register_b, 6);
    }

    #[test]
    fn test_out() {
        let mut computer = setup_computer(vec![5, 4], 7, 0, 0);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![7]);
    }

    #[test]
    fn test_bdv() {
        let mut computer = setup_computer(vec![6, 2], 8, 0, 0);
        computer.run().unwrap();
        assert_eq!(computer.register_b, 2);
    }

    #[test]
    fn test_cdv() {
        let mut computer = setup_computer(vec![7, 2], 8, 0, 0);
        computer.run().unwrap();
        assert_eq!(computer.register_c, 2);
    }

    #[test]
    fn test_example1() {
        let mut computer = setup_computer(vec![2, 6], 0, 0, 9);
        computer.run().unwrap();
        assert_eq!(computer.register_b, 1);
    }

    #[test]
    fn test_example2() {
        let mut computer = setup_computer(vec![5, 0, 5, 1, 5, 4], 10, 0, 0);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![0, 1, 2]);
    }

    #[test]
    fn test_example3() {
        let mut computer = setup_computer(vec![0, 1, 5, 4, 3, 0], 2024, 0, 0);
        computer.run().unwrap();
        assert_eq!(computer.output, vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0]);
        assert_eq!(computer.register_a, 0);
    }

    #[test]
    fn test_example4() {
        let mut computer = setup_computer(vec![1, 7], 0, 29, 0);
        computer.run().unwrap();
        assert_eq!(computer.register_b, 26);
    }

    #[test]
    fn test_example5() {
        let mut computer = setup_computer(vec![4, 0], 0, 2024, 43690);
        computer.run().unwrap();
        assert_eq!(computer.register_b, 44354);
    }
}
