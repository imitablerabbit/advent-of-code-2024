use regex::Regex;
use std::{fs::File, io::Read};

/// An enum representing different types of instructions.
///
/// The `Instruction` enum has three variants:
/// - `Mul(i32, i32)`: Represents a multiplication operation with two operands.
/// - `Do()`: An enable flag to indicate that future instructions should be processed.
/// - `Dont()`: A disable flag to indicate that future instructions should not be processed.
#[derive(Debug, PartialEq)]
enum Instruction {
    Mul(i32, i32),
    Do(),
    Dont(),
}

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let reports = parse(&puzzle_input);
    let result = run_instructions(reports);
    println!("The result of the instructions is: {}", result);
}

/// Reads the contents of the input file and returns a result of the file contents.
///
/// # Arguments
///
/// * `puzzle_path` - A string slice that holds the path to the input file.
///
/// # Returns
///
/// * `Result<String, std::io::Error>` - The contents of the file as a string or an error.
fn read_to_string(puzzle_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(puzzle_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Parses the puzzle input and returns a vector of Instruction tuples.
/// Each tuple contains the two numbers that need to be multiplied together.
///
/// An instruction can be parsed by reading a 'mul(a, b)' where a and b
/// are numbers that can be 1-3 digits long.
///
/// # Arguments
///
/// * `puzzle_input` - A string slice that holds the puzzle input.
///
/// # Returns
///
/// * `Vec<Instruction>` - A vector of instructions, where each Instruction is
///   a tuple of integers.
fn parse(puzzle_input: &str) -> Vec<Instruction> {
    // Use regex to find the instances of 'mul(a, b)' in the puzzle input.
    let re = Regex::new(r"(mul\((\d{1,3}),\s*(\d{1,3})\)|do\(\)|don't\(\))").unwrap();
    re.captures_iter(puzzle_input)
        .map(|cap| {
            println!("{}", &cap[0]);
            match &cap[0] {
                "do()" => Instruction::Do(),
                "don't()" => Instruction::Dont(),
                _ => {
                    // Parse the two numbers from the nested capture group.
                    let a = cap[2].parse().unwrap();
                    let b = cap[3].parse().unwrap();
                    Instruction::Mul(a, b)
                }
            }
        })
        .collect()
}

/// Runs the instructions and returns the result of the multiplication.
///
/// The instructions are processed in order, and the result is calculated by
/// multiplying the two numbers together.
///
/// The Do instruction enables the processing of future instructions.
/// The Dont instruction disables the processing of future instructions.
///
/// # Arguments
///
/// * `instructions` - A vector of instructions to be processed.
///
/// # Returns
///
/// * `i32` - The result of the multiplication of the two numbers.
///
fn run_instructions(instructions: Vec<Instruction>) -> i32 {
    let mut result = 0;
    let mut do_flag = true;
    for instruction in instructions {
        match instruction {
            Instruction::Mul(a, b) => {
                if do_flag {
                    result += a * b;
                }
            }
            Instruction::Do() => {
                do_flag = true;
            }
            Instruction::Dont() => {
                do_flag = false;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let puzzle_input =
            r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;
        let expected = vec![
            Instruction::Mul(2, 4),
            Instruction::Dont(),
            Instruction::Mul(5, 5),
            Instruction::Mul(11, 8),
            Instruction::Do(),
            Instruction::Mul(8, 5),
        ];
        assert_eq!(parse(puzzle_input), expected);
    }

    #[test]
    fn test_run_instructions() {
        let instructions = vec![
            Instruction::Mul(2, 4),
            Instruction::Dont(),
            Instruction::Mul(5, 5),
            Instruction::Mul(11, 8),
            Instruction::Do(),
            Instruction::Mul(8, 5),
        ];
        assert_eq!(
            run_instructions(instructions),
            48 // 2 * 4 + 8 * 5
        );
    }
}
