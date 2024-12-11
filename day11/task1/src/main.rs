use std::time::Instant;
use std::{fs::File, io::Read};

/// Reads the contents of the input file and returns a result of the file contents.
///
/// # Arguments
///
/// * `puzzle_path` - A string slice that holds the path to the input file.
///
/// # Returns
///
/// * `Result<String, std::io::Error>` - The contents of the file as a string or an error.
///
fn read_to_string(puzzle_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(puzzle_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse(puzzle_input: &str) -> Result<Vec<u64>, std::num::ParseIntError> {
    let first_line = puzzle_input.lines().next().unwrap();
    first_line.split_whitespace().map(|s| s.parse()).collect()
}

fn apply_rules_to_stones(stones: Vec<u64>) -> Vec<u64> {
    stones
        .iter()
        .flat_map(|&stone| apply_rules(stone))
        .collect()
}

fn apply_rules(stone: u64) -> Vec<u64> {
    // Rule 1
    if stone == 0 {
        return vec![1];
    }

    // Rule 2
    let digits = stone.ilog10() + 1;
    let even_digits = digits % 2 == 0;
    if even_digits {
        let half = digits / 2;
        let left = stone / 10u64.pow(half);
        let right = stone % 10u64.pow(half);
        return vec![left, right];
    }

    // Rule 3
    vec![stone * 2024]
}

/// The main function that reads the puzzle input, parses the map, and finds the starting points.
fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let mut stones = parse(&puzzle_input).unwrap();
    let iterations = 25;
    for i in 0..iterations {
        let start = Instant::now();
        stones = apply_rules_to_stones(stones);
        let duration = start.elapsed();
        println!("Iteration {} took: {:?}", i + 1, duration);
    }
    let count = stones.len();
    println!(
        "The number of stones after {} iterations is: {}",
        iterations, count
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    static PUZZLE_INPUT: &str = r#"125 17"#;

    #[test]
    fn test_parse() {
        let expected = vec![125, 17];
        let result = parse(PUZZLE_INPUT).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_apply_rules_to_stones() {
        let stones = vec![125, 17];
        let expected_iterations = vec![
            vec![125, 17],
            vec![253000, 1, 7],
            vec![253, 0, 2024, 14168],
            vec![512072, 1, 20, 24, 28676032],
            vec![512, 72, 2024, 2, 0, 2, 4, 2867, 6032],
            vec![1036288, 7, 2, 20, 24, 4048, 1, 4048, 8096, 28, 67, 60, 32],
            vec![
                2097446912, 14168, 4048, 2, 0, 2, 4, 40, 48, 2024, 40, 48, 80, 96, 2, 8, 6, 7, 6,
                0, 3, 2,
            ],
        ];

        // Run through the iterations
        let mut iterations = vec![stones];
        for _ in 0..expected_iterations.len() - 1 {
            let last = iterations.last().unwrap().clone();
            let next = apply_rules_to_stones(last);
            iterations.push(next);
        }

        assert_eq!(iterations, expected_iterations);
    }
}
