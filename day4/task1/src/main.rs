use std::{fs::File, io::Read};

type Puzzle = Vec<Vec<char>>;

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let puzzle = parse_puzzle(&puzzle_input);
    let count = count_word(&puzzle, "XMAS");
    println!("Found {} words", count);
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
///
fn read_to_string(puzzle_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(puzzle_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Parses the puzzle input into a 2D vector of characters.
///
/// # Arguments
///
/// * `input` - A string slice that holds the puzzle input.
///
/// # Returns
///
/// A 2D vector of characters representing the puzzle.
fn parse_puzzle(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// Counts the occurrences of a word in the puzzle.
///
/// # Arguments
///
/// * `puzzle` - A reference to a 2D vector of characters representing the puzzle.
/// * `word` - A string slice that holds the word to be counted.
///
/// # Returns
///
/// The number of times the word appears in the puzzle.
fn count_word(puzzle: &Vec<Vec<char>>, word: &str) -> usize {
    let reversed_word: String = word.chars().rev().collect();
    let mut count = count_word_downwards(puzzle, word);
    count += count_word_downwards(puzzle, &reversed_word);
    count
}

fn count_word_downwards(puzzle: &Puzzle, word: &str) -> usize {
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();
    let mut count = 0;
    let first_char = word_chars[0];
    for i in 0..puzzle.len() {
        for j in 0..puzzle[i].len() {
            if puzzle[i][j] == first_char {
                if check_right(puzzle, &word_chars, i, j, word_len) {
                    count += 1;
                }
                if check_down(puzzle, &word_chars, i, j, word_len) {
                    count += 1;
                }
                if check_diagonal_down_right(puzzle, &word_chars, i, j, word_len) {
                    count += 1;
                }
                if check_diagonal_down_left(puzzle, &word_chars, i, j, word_len) {
                    count += 1;
                }
            }
        }
    }
    count
}

fn check_right(puzzle: &Puzzle, word_chars: &[char], i: usize, j: usize, word_len: usize) -> bool {
    if j + word_len <= puzzle[i].len() {
        for (k, &ch) in word_chars.iter().enumerate().skip(1) {
            if puzzle[i][j + k] != ch {
                return false;
            }
        }
        return true;
    }
    false
}

fn check_down(puzzle: &Puzzle, word_chars: &[char], i: usize, j: usize, word_len: usize) -> bool {
    if i + word_len <= puzzle.len() {
        for (k, &ch) in word_chars.iter().enumerate().skip(1) {
            if puzzle[i + k][j] != ch {
                return false;
            }
        }
        return true;
    }
    false
}

fn check_diagonal_down_right(
    puzzle: &Puzzle,
    word_chars: &[char],
    i: usize,
    j: usize,
    word_len: usize,
) -> bool {
    if i + word_len <= puzzle.len() && j + word_len <= puzzle[i].len() {
        for (k, &ch) in word_chars.iter().enumerate().skip(1) {
            if puzzle[i + k][j + k] != ch {
                return false;
            }
        }
        return true;
    }
    false
}

fn check_diagonal_down_left(
    puzzle: &Puzzle,
    word_chars: &[char],
    i: usize,
    j: usize,
    word_len: usize,
) -> bool {
    if i + word_len <= puzzle.len() && j >= word_len - 1 {
        for (k, &ch) in word_chars.iter().enumerate().skip(1) {
            if puzzle[i + k][j - k] != ch {
                return false;
            }
        }
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let puzzle_input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let puzzle = parse_puzzle(puzzle_input);
        assert_eq!(puzzle.len(), 10);
        assert_eq!(puzzle[0].len(), 10);
    }

    #[test]
    fn test_find_word() {
        let puzzle_input = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;
        let puzzle = parse_puzzle(puzzle_input);
        let count = count_word(&puzzle, "XMAS");
        assert_eq!(count, 18);
    }
}
