use itertools::Itertools;
use pathfinding::prelude::Matrix;
use pathfinding::{matrix::directions, prelude::astar_bag_collect};
use std::{fs::File, io::Read};

/// Point in the matrix represenitng a (col, row).
type Point = (usize, usize);

type Direction = (isize, isize);

struct Map {
    map: Matrix<char>,

    // The col, row position of the reindeer and its destination.
    start: Point,
    end: Point,

    // The direction the reindeer is facing.
    direction: Direction,
}

const START: char = 'S';
const END: char = 'E';
const WALL: char = '#';

impl Map {
    /// Creates a new `Map` instance from a given matrix.
    ///
    /// # Arguments
    ///
    /// * `map` - A `Matrix<char>` representing the map.
    ///
    /// # Returns
    ///
    /// * `Self` - A new `Map` instance.
    fn new(map: Matrix<char>) -> Self {
        let start_post = Self::find_char(&map, START).expect("Start not found");
        let end_post = Self::find_char(&map, END).expect("End not found");
        Self {
            map,
            start: start_post,
            end: end_post,
            direction: directions::E,
        }
    }

    /// Finds the position of a given character in the matrix.
    ///
    /// # Arguments
    ///
    /// * `map` - A reference to a `Matrix<char>`.
    /// * `c` - The character to find.
    ///
    /// # Returns
    ///
    /// * `Option<(usize, usize)>` - The position of the character if found, or `None`.
    fn find_char(map: &Matrix<char>, c: char) -> Option<(usize, usize)> {
        for (col, row_chars) in map.iter().enumerate() {
            for (row, &cell) in row_chars.iter().enumerate() {
                if cell == c {
                    return Some((col, row));
                }
            }
        }
        None
    }

    /// Calculates the weight of changing direction.
    ///
    /// # Arguments
    ///
    /// * `current` - The current direction.
    /// * `new` - The new direction.
    ///
    /// # Returns
    ///
    /// * `usize` - The weight of changing direction.
    fn direction_weight(current: Direction, new: Direction) -> usize {
        match (current, new) {
            (directions::N, directions::N) => 1, // Same direction
            (directions::S, directions::S) => 1,
            (directions::E, directions::E) => 1,
            (directions::W, directions::W) => 1,
            (directions::N, directions::E) => 1001, // Clockwise
            (directions::E, directions::S) => 1001,
            (directions::S, directions::W) => 1001,
            (directions::W, directions::N) => 1001,
            (directions::N, directions::W) => 1001, // Counter clockwise
            (directions::W, directions::S) => 1001,
            (directions::S, directions::E) => 1001,
            (directions::E, directions::N) => 1001,
            (directions::N, directions::S) => 2001, // Turn around
            (directions::S, directions::N) => 2001,
            (directions::E, directions::W) => 2001,
            (directions::W, directions::E) => 2001,
            _ => panic!("Invalid direction"),
        }
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

/// Parses the input string into a `Map` instance.
///
/// # Arguments
///
/// * `input` - A string slice containing the input data.
///
/// # Returns
///
/// * `Map` - A new `Map` instance.
fn parse(input: &str) -> Map {
    let matrix = Matrix::from_rows(input.lines().map(|l| l.chars())).expect("Invalid matrix");
    Map::new(matrix)
}

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let map = parse(&puzzle_input);

    let start = (map.start, map.direction);
    let successors = |&(pos, dir): &(Point, Direction)| {
        let mut successors = Vec::new();
        for &new_dir in directions::DIRECTIONS_4.iter() {
            if let Some(new_pos) = map.map.move_in_direction(pos, new_dir) {
                if map.map[new_pos] == WALL {
                    continue;
                }
                let weight = Map::direction_weight(dir, new_dir);
                successors.push(((new_pos, new_dir), weight));
            }
        }
        successors
    };
    let results = astar_bag_collect(&start, successors, |_| 1, |&(pos, _)| pos == map.end).unwrap();
    let paths = results.0;
    let all_path_points = paths
        .iter()
        .flat_map(|path| path.iter().map(|(pos, _)| *pos))
        .unique()
        .collect_vec();
    let count = all_path_points.len();
    println!("The reindeer can visit {} unique points", count);
}
