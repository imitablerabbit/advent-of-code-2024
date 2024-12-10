use std::{fs::File, io::Read};

use pathfinding::directed::astar::*;
use pathfinding::matrix::{Matrix, MatrixFormatError};

type Map = Matrix<usize>;

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
/// * `Result<Matrix<usize>, MatrixFormatError>` - A 2d array of positive integers or an error.
///
fn parse(puzzle_input: &str) -> Result<Matrix<usize>, MatrixFormatError> {
    Matrix::from_rows(
        puzzle_input
            .lines()
            .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as usize)),
    )
}

fn find_all_points(map: &Map, point: usize) -> Vec<(usize, usize)> {
    let mut points = Vec::new();
    for (row_idx, row) in map.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == point {
                points.push((row_idx, col_idx));
            }
        }
    }
    points
}

/// Finds all paths from the start point to the end point on the given map.
///
/// # Arguments
///
/// * `start_point` - A tuple representing the starting coordinates (row, col).
/// * `end_point` - A tuple representing the ending coordinates (row, col).
/// * `map` - A reference to the map on which to find the paths.
///
/// # Returns
///
/// An `Option` containing a vector of vectors of tuples, where each tuple represents a coordinate (row, col) in a path.
fn find_all_paths(
    start_point: (usize, usize),
    end_point: (usize, usize),
    map: &Map,
) -> Option<Vec<Vec<(usize, usize)>>> {
    let successors = |&(row, col): &(usize, usize)| -> Vec<((usize, usize), usize)> {
        let cell_value = map.get((row, col)).unwrap();
        map.neighbours((row, col), false)
            .filter(|&(neighbour_row, neighbour_col)| {
                let n_cell_value = map.get((neighbour_row, neighbour_col)).unwrap();
                (*n_cell_value == cell_value + 1) && *n_cell_value <= 9
            })
            .map(|n| (n, 1))
            .collect()
    };
    let result = astar_bag_collect(
        &start_point,
        successors,
        |_: &(usize, usize)| -> usize { 1 },
        |&(row, col): &(usize, usize)| -> bool { (row, col) == end_point },
    )?;
    Some(result.0)
}

/// Prints the map with the given path highlighted.
///
/// # Arguments
///
/// * `map` - A reference to the map to be printed.
/// * `path` - A slice of tuples representing the coordinates (row, col) of the path to be highlighted.
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

/// The main function that reads the puzzle input, parses the map, and finds the starting points.
fn main() {
    let puzzle_path = "input/input_example_larger.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let map = parse(&puzzle_input).unwrap();
    println!("{:?}", map);
    let starting_points = find_all_points(&map, 0);
    let ending_points = find_all_points(&map, 9);
    let mut count = 0;
    starting_points.iter().for_each(|&start| {
        ending_points.iter().for_each(|&end| {
            if let Some(paths) = find_all_paths(start, end, &map) {
                count += paths.len();
            }
        });
    });
    println!("Count: {}", count);
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
            Matrix::from_rows(vec![
                vec![0, 1, 2, 3],
                vec![1, 2, 3, 4],
                vec![8, 7, 6, 5],
                vec![9, 8, 7, 6],
            ])
            .unwrap()
        );
    }

    #[test]
    fn test_find_all_points() {
        let map = parse(PUZZLE_INPUT).unwrap();
        let points = find_all_points(&map, 1);
        assert_eq!(points, vec![(0, 1), (1, 0)]);
    }

    #[test]
    fn test_find_all_paths() {
        let map = parse(PUZZLE_INPUT).unwrap();
        let start0 = find_all_points(&map, 0)[0];
        let end9 = find_all_points(&map, 9)[0];
        println!("{:?}", start0);
        println!("{:?}", end9);
        let paths = find_all_paths(start0, end9, &map).unwrap();
        assert_eq!(paths.len(), 16);
    }

    #[test]
    fn test_print_map_path() {
        let map = parse(PUZZLE_INPUT).unwrap();
        let start0 = find_all_points(&map, 0)[0];
        let end9 = find_all_points(&map, 9)[0];
        let paths = find_all_paths(start0, end9, &map).unwrap();
        print_map_path(&map, &paths[0]);
    }
}
