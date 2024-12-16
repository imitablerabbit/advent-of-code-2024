use colored::Colorize;
use itertools::Itertools;
use pathfinding::prelude::Matrix;
use pathfinding::{matrix::directions, prelude::astar_bag_collect};
use rayon::prelude::*;
use std::{fs::File, io::Read};

/// Point in the matrix represenitng a (col, row).
type Point = (usize, usize);

type Direction = (isize, isize);

fn convert_direction(direction: Direction) -> char {
    match direction {
        directions::N => '^',
        directions::S => 'v',
        directions::E => '>',
        directions::W => '<',
        _ => '#',
    }
}

struct Map {
    map: Matrix<char>,

    // The col, row position of the reindeer and its destination.
    position: Point,
    start: Point,
    end: Point,

    // The direction the reindeer is facing.
    direction: Direction,
}

const FREE_SPACE: char = '.';
const START: char = 'S';
const END: char = 'E';
const WALL: char = '#';

impl Map {
    fn new(map: Matrix<char>) -> Self {
        let start_post = Self::find_char(&map, START).expect("Start not found");
        let end_post = Self::find_char(&map, END).expect("End not found");
        Self {
            map,
            position: start_post,
            start: start_post,
            end: end_post,
            direction: directions::E,
        }
    }

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

    /// If we are travveling in the same direction the weight is 1. Each time we
    /// rotate to the left or right the weight is invreased by 1000.
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

    fn print(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let colored_cell = if (x, y) == (self.position.1, self.position.0) {
                    '@'.to_string().yellow().bold()
                } else {
                    match cell {
                        FREE_SPACE => FREE_SPACE.to_string().white(),
                        START => START.to_string().green(),
                        END => END.to_string().green(),
                        WALL => WALL.to_string().red(),
                        _ => cell.to_string().normal(),
                    }
                };
                print!("{}", colored_cell);
            }
            println!();
        }
    }

    fn print_path(&self, path: Vec<(Point, Direction)>) {
        let mut map = self.map.clone();
        path.iter()
            .for_each(|(pos, dir)| map[*pos] = convert_direction(*dir));
        for (y, row) in map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let colored_cell = if (x, y) == (self.position.1, self.position.0) {
                    '@'.to_string().yellow().bold()
                } else {
                    match cell {
                        FREE_SPACE => FREE_SPACE.to_string().white(),
                        START => START.to_string().green(),
                        END => END.to_string().green(),
                        WALL => WALL.to_string().red(),
                        'X' => 'X'.to_string().blue(),
                        _ => cell.to_string().normal(),
                    }
                };
                print!("{}", colored_cell);
            }
            println!();
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
    let path = astar_bag_collect(&start, successors, |_| 1, |&(pos, _)| pos == map.end).unwrap();
    let lowest_score = path.1;
    map.print_path(path.0.first().unwrap().clone());
    println!("Lowest score: {}", lowest_score);
}

#[cfg(test)]
mod tests {
    use super::*;
}
