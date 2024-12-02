use std::{fs::File, io::Read};

// A report is a vector of integers. The numbers in the report are called levels.
type Report = Vec<i32>;

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let reports = parse(&puzzle_input);
    let safe_count = safe_count(reports);
    println!("The number of safe reports is: {}", safe_count);
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

/// Parses the puzzle input and returns a vector of reports. Each report is a vector
/// of integers. The integers are separated by a space.
///
/// # Arguments
///
/// * `puzzle_input` - A string slice that holds the puzzle input.
///
/// # Returns
///
/// * `Vec<Report>` - A vector of reports, where each report is a vector of integers.
fn parse(puzzle_input: &str) -> Vec<Report> {
    puzzle_input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        })
        .collect()
}

/// Checks if a report is safe. A report is safe if it satisfies the following
/// conditions:
///  - The levels are either all increasing or all decreasing.
///  - Any two adjacent levels differ by at least one and at most three.
///
/// If a report is not safe, it is possible to remove one level from the report to
/// make it safe.
///
/// # Arguments
///
/// * `report` - A reference to a report (vector of integers).
///
/// # Returns
///
/// * `bool` - `true` if the report is safe, `false` otherwise.
fn is_safe(report: &Report) -> bool {
    if check_safety(report) {
        return true;
    }

    for i in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(i);
        if check_safety(&modified_report) {
            return true;
        }
    }

    false
}

/// Checks if a report satisfies the safety conditions:
///  - The levels are either all increasing or all decreasing.
///  - Any two adjacent levels differ by at least one and at most three.
///
/// # Arguments
///
/// * `report` - A reference to a report (vector of integers).
///
/// # Returns
///
/// * `bool` - `true` if the report satisfies the safety conditions, `false` otherwise.
fn check_safety(report: &Report) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;

    for window in report.windows(2) {
        let diff = (window[1] - window[0]).abs();
        if !(1..=3).contains(&diff) {
            return false;
        }

        match window[0].cmp(&window[1]) {
            std::cmp::Ordering::Less => is_decreasing = false,
            std::cmp::Ordering::Greater => is_increasing = false,
            std::cmp::Ordering::Equal => {}
        }

        if !is_increasing && !is_decreasing {
            return false;
        }
    }
    is_increasing || is_decreasing
}

/// Counts the number of safe reports in a vector of reports.
///
/// # Arguments
///
/// * `reports` - A vector of reports, where each report is a vector of integers.
///
/// # Returns
///
/// * `usize` - The number of safe reports.
fn safe_count(reports: Vec<Report>) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let puzzle_input = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;
        let expected = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(parse(puzzle_input), expected);
    }

    #[test]
    fn test_is_safe() {
        let report = vec![7, 6, 4, 2, 1];
        assert!(is_safe(&report));
        let report = vec![1, 2, 7, 8, 9];
        assert!(!is_safe(&report));
        let report = vec![9, 7, 6, 2, 1];
        assert!(!is_safe(&report));
        let report = vec![1, 3, 2, 4, 5];
        assert!(is_safe(&report));
        let report = vec![8, 6, 4, 4, 1];
        assert!(is_safe(&report));
        let report = vec![1, 3, 6, 7, 9];
        assert!(is_safe(&report));
    }

    #[test]
    fn test_safe_count() {
        let reports = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(safe_count(reports), 4);
    }
}
