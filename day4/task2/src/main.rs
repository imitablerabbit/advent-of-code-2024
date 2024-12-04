use std::{fs::File, io::Read};

/// A type alias for the puzzle, which is a 2D vector of characters.
type Puzzle = Vec<Vec<char>>;

/// A type alias for coordinates, represented as a tuple of two usize values.
type Coordinate = (usize, usize);

/// A struct to hold the result of a word search, containing the center coordinate.
#[derive(Debug)]
struct WordSearchResult {
    center: Coordinate,
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

/// Parses the puzzle input string into a 2D vector of characters.
///
/// # Arguments
///
/// * `input` - A string slice that holds the puzzle input.
///
/// # Returns
///
/// * `Puzzle` - The parsed puzzle as a 2D vector of characters.
fn parse_puzzle(input: &str) -> Puzzle {
    input.lines().map(|line| line.chars().collect()).collect()
}

/// Finds the specified word in the puzzle and returns a vector of word search results.
///
/// # Arguments
///
/// * `puzzle` - A reference to the puzzle.
/// * `word` - The word to search for.
///
/// # Returns
///
/// * `Vec<WordSearchResult>` - A vector of word search results containing the coordinates of the found word.
fn find_word(puzzle: &Puzzle, word: &str) -> Vec<WordSearchResult> {
    let reversed_word = word.chars().rev().collect::<String>();
    let mut results = find_word_downwards(puzzle, word); // MAS
    results.append(&mut find_word_downwards(puzzle, &reversed_word)); // SAM
    results
}

fn find_word_downwards(puzzle: &Puzzle, word: &str) -> Vec<WordSearchResult> {
    let mut results = vec![];
    let word = word.chars().collect::<Vec<char>>();
    let word_len = word.len();
    let center_diff = word_len / 2;

    for i in 0..puzzle.len() {
        for j in 0..puzzle[i].len() {
            if puzzle[i][j] == word[0] {
                if check_diagonal_down_right(puzzle, &word, i, j, word_len) {
                    results.push(WordSearchResult {
                        center: (i + center_diff, j + center_diff),
                    });
                }
                if check_diagonal_down_left(puzzle, &word, i, j, word_len) {
                    results.push(WordSearchResult {
                        center: (i + center_diff, j - center_diff),
                    });
                }
            }
        }
    }
    results
}

fn check_diagonal_down_right(
    puzzle: &Puzzle,
    word: &[char],
    i: usize,
    j: usize,
    word_len: usize,
) -> bool {
    if i + word_len <= puzzle.len() && j + word_len <= puzzle[i].len() {
        for (k, &ch) in word.iter().enumerate().skip(1) {
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
    word: &[char],
    i: usize,
    j: usize,
    word_len: usize,
) -> bool {
    if i + word_len <= puzzle.len() && j >= word_len - 1 {
        for (k, &ch) in word.iter().enumerate().skip(1) {
            if puzzle[i + k][j - k] != ch {
                return false;
            }
        }
        return true;
    }
    false
}

// find all occurences of the results where the word forms an X in the original
// puzzle. This means that the word is found in two directions and crosses
// itself in the middle.
fn find_crosses(results: Vec<WordSearchResult>) -> Vec<Coordinate> {
    let mut crosses = vec![];
    for i in 0..results.len() {
        for j in i + 1..results.len() {
            if results[i].center == results[j].center {
                crosses.push(results[i].center);
            }
        }
    }
    crosses
}

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let puzzle = parse_puzzle(&puzzle_input);
    let found_words = find_word(&puzzle, "MAS");
    let crosses = find_crosses(found_words);
    let count = crosses.len();
    println!("Number of crosses: {}", count);
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
        let found_words = find_word(&puzzle, "MAS");
        assert_eq!(found_words.len(), 25);
    }

    #[test]
    fn test_find_crosses() {
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
        let found_words = find_word(&puzzle, "MAS");
        let crosses = find_crosses(found_words);
        assert_eq!(crosses.len(), 9);
    }

    #[test]
    fn test_find_word2() {
        let puzzle_input = r#"M.S
.A.
M.S"#;
        let puzzle = parse_puzzle(puzzle_input);
        let found_words = find_word(&puzzle, "MAS");
        assert_eq!(found_words.len(), 2);
    }

    #[test]
    fn test_find_crosses2() {
        let puzzle_input = r#"M.S
.A.
M.S"#;
        let puzzle = parse_puzzle(puzzle_input);
        let found_words = find_word(&puzzle, "MAS");
        let crosses = find_crosses(found_words);
        assert_eq!(crosses.len(), 1);
    }
}
