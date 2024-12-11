use std::collections::HashMap;
use std::time::Instant;
use std::{fs::File, io::Read};

fn read_to_string(puzzle_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(puzzle_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn parse(puzzle_input: &str) -> Result<HashMap<u64, u64>, std::num::ParseIntError> {
    let first_line = puzzle_input.lines().next().unwrap();
    let mut stones = HashMap::new();
    for stone in first_line.split_whitespace() {
        let stone = stone.parse()?;
        *stones.entry(stone).or_insert(0) += 1;
    }
    Ok(stones)
}

fn apply_rules_to_stones(stones: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut new_stones = HashMap::new();
    for (&stone, &count) in stones.iter() {
        for new_stone in apply_rules(stone) {
            *new_stones.entry(new_stone).or_insert(0) += count;
        }
    }
    new_stones
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

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let mut stones = parse(&puzzle_input).unwrap();
    let iterations = 75;
    for i in 0..iterations {
        let start = Instant::now();
        stones = apply_rules_to_stones(stones);
        let duration = start.elapsed();
        println!("Iteration {} took: {:?}", i + 1, duration);
    }
    let count: u64 = stones.values().sum();
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
        let mut expected = HashMap::new();
        expected.insert(125, 1);
        expected.insert(17, 1);
        let result = parse(PUZZLE_INPUT).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_apply_rules_to_stones() {
        let mut stones = HashMap::new();
        stones.insert(125, 1);
        stones.insert(17, 1);
        let mut expected_iterations = vec![
            {
                let mut map = HashMap::new();
                map.insert(125, 1);
                map.insert(17, 1);
                map
            },
            {
                let mut map = HashMap::new();
                map.insert(253000, 1);
                map.insert(1, 1);
                map.insert(7, 1);
                map
            },
            {
                let mut map = HashMap::new();
                map.insert(253, 1);
                map.insert(0, 1);
                map.insert(2024, 1);
                map.insert(14168, 1);
                map
            },
            {
                let mut map = HashMap::new();
                map.insert(512072, 1);
                map.insert(1, 1);
                map.insert(20, 1);
                map.insert(24, 1);
                map.insert(28676032, 1);
                map
            },
            {
                let mut map = HashMap::new();
                map.insert(512, 1);
                map.insert(72, 1);
                map.insert(2024, 1);
                map.insert(0, 1);
                map.insert(2, 2);
                map.insert(4, 1);
                map.insert(2867, 1);
                map.insert(6032, 1);
                map
            },
            {
                let mut map = HashMap::new();
                map.insert(1036288, 1);
                map.insert(7, 1);
                map.insert(2, 1);
                map.insert(20, 1);
                map.insert(24, 1);
                map.insert(1, 1);
                map.insert(4048, 2);
                map.insert(8096, 1);
                map.insert(28, 1);
                map.insert(67, 1);
                map.insert(60, 1);
                map.insert(32, 1);
                map
            },
            {
                let mut map = HashMap::new();
                map.insert(2097446912, 1);
                map.insert(14168, 1);
                map.insert(4048, 1);
                map.insert(2, 4);
                map.insert(4, 1);
                map.insert(2024, 1);
                map.insert(40, 2);
                map.insert(48, 2);
                map.insert(80, 1);
                map.insert(96, 1);
                map.insert(8, 1);
                map.insert(7, 1);
                map.insert(6, 2);
                map.insert(0, 2);
                map.insert(3, 1);
                map
            },
        ];

        let mut iterations = vec![stones];
        for _ in 0..expected_iterations.len() - 1 {
            let last = iterations.last().unwrap().clone();
            let next = apply_rules_to_stones(last);
            iterations.push(next);
        }

        for (i, (expected, result)) in expected_iterations
            .iter()
            .zip(iterations.iter())
            .enumerate()
        {
            assert_eq!(result, expected, "Iteration {}", i + 1);
        }
    }
}
