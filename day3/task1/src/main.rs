use regex::Regex;
use std::{fs::File, io::Read};

type Instruction = (i32, i32);

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
    let re = Regex::new(r"mul\((\d{1,3}),\s*(\d{1,3})\)").unwrap();
    re.captures_iter(puzzle_input)
        .map(|cap| {
            (
                cap[1].parse::<i32>().unwrap(),
                cap[2].parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn run_instructions(instructions: Vec<Instruction>) -> i32 {
    instructions.iter().fold(0, |acc, (a, b)| acc + a * b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let puzzle_input =
            r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;
        let expected = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        assert_eq!(parse(puzzle_input), expected);
    }

    #[test]
    fn test_run_instructions() {
        let instructions = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        assert_eq!(
            run_instructions(instructions),
            161 // 2 * 4 + 5 * 5 + 11 * 8 + 8 * 5
        );
    }
}
