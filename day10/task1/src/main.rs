use std::{fs::File, io::Read};

use pathfinding::directed::dfs::*;

type Map = Vec<Vec<usize>>;

#[derive(Debug, PartialEq)]
enum MapError {
    InvalidMap { message: String },
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

/// Parses the input file and returns a 2d array of positive integers.
/// If the input file is not formatted correctly, the function will return an
/// error as a result.
///
/// # Arguments
///
/// * `puzzle_input` - A string containing the contents of the input file.
///
/// # Returns
///
/// * `Result<Vec<Vec<usize>>, MapError>` - A 2d array of positive integers or an error.
///
fn parse(puzzle_input: &str) -> Result<Map, MapError> {
    let mut map = Vec::new();
    for (line_idx, line) in puzzle_input.lines().enumerate() {
        let mut row = Vec::new();
        for (col_idx, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                row.push(c.to_digit(10).unwrap() as usize);
            } else {
                return Err(MapError::InvalidMap {
                    message: format!(
                        "Invalid character '{}' at line {}, column {}",
                        c,
                        line_idx + 1,
                        col_idx + 1
                    ),
                });
            }
        }
        map.push(row);
    }
    Ok(map)
}

fn get_successors(map: &Map, row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut successors = Vec::new();
    if row > 0 {
        successors.push((row - 1, col));
    }
    if row < map.len() - 1 {
        successors.push((row + 1, col));
    }
    if col > 0 {
        successors.push((row, col - 1));
    }
    if col < map[0].len() - 1 {
        successors.push((row, col + 1));
    }
    let cell = map[row][col];
    successors
        .into_iter()
        .filter(|&(r, c)| map[r][c] == cell + 1)
        .collect()
}

fn find_all_starting_points(map: &Map) -> Vec<(usize, usize)> {
    let mut starting_points = Vec::new();
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == 0 {
                starting_points.push((row_idx, col_idx));
            }
        }
    }
    starting_points
}

fn print_map_path(map: &Map, path: &[(usize, usize)]) {
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if path.contains(&(row_idx, col_idx)) {
                print!("{}", cell);
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn find_paths(start: (usize, usize), map: &Map) -> Vec<Vec<(usize, usize)>> {
    let mut paths = Vec::new();
    let successors =
        |&(row, col): &(usize, usize)| -> Vec<(usize, usize)> { get_successors(map, row, col) };

    let mut stack = vec![(vec![start], start)];
    while let Some((path, (row, col))) = stack.pop() {
        if map[row][col] == 9 {
            paths.push(path.clone());
        }
        for succ in successors(&(row, col)) {
            let mut new_path = path.clone();
            new_path.push(succ);
            stack.push((new_path, succ));
        }
    }

    paths
}

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let map = parse(&puzzle_input).unwrap();
    println!("{:?}", map);
    let starting_points = find_all_starting_points(&map);
    let mut all_paths = Vec::new();
    let mut score = 0;
    for start in starting_points {
        let paths = find_paths(start, &map);
        all_paths.push(paths.clone());
        let mut end_points: Vec<(usize, usize)> =
            paths.iter().map(|path| path[path.len() - 1]).collect();
        end_points.sort();
        end_points.dedup();
        score += end_points.len();
    }
    for (idx, paths) in all_paths.iter().enumerate() {
        println!("Starting point {}", idx);
        println!("{:?}", paths);
        for path in paths {
            print_map_path(&map, path);
            println!();
        }
    }
    println!("Score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    static PUZZLE_INPUT: &str = r#"0123
1234
8765
9876"#;

    #[test]
    fn test_parse() {
        let map = parse(PUZZLE_INPUT).unwrap();
        assert_eq!(
            map,
            vec![
                vec![0, 1, 2, 3],
                vec![1, 2, 3, 4],
                vec![8, 7, 6, 5],
                vec![9, 8, 7, 6]
            ]
        );
    }
}
