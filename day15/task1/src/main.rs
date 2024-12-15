use colored::Colorize;
use itertools::Itertools;
use pathfinding::matrix::directions;
use pathfinding::prelude::Matrix;
use rayon::prelude::*;
use std::{fs::File, io::Read};

struct Map {
    map: Matrix<char>,

    // The col, row position of the robot. This is the position of the robot
    // in the matrix.
    position: (usize, usize),
}

const FREE_SPACE: char = '.';
const BOX: char = 'O';
const WALL: char = '#';

impl Map {
    fn new(map: Matrix<char>, position: (usize, usize)) -> Self {
        Self { map, position }
    }

    /// Moves the robot in the specified direction. The robot can move in any
    /// of the four cardinal directions (up, down, left, right).
    ///
    /// The robot can only move into a free space or push a box into a free space.
    /// When the robot pushes a box, the box is moved into the free space. The
    /// robot can push multiple boxes at once as long as there is a free space
    /// behind the boxes. If there is a wall behind the chain of boxes, the robot
    /// cannot push the boxes.
    ///
    /// # Arguments
    ///
    /// * `direction` - The direction in which the robot should move. The direction
    ///   can be one of the four cardinal directions: up, down, left, right represented
    ///   by the `Direction` enum.
    ///
    fn move_direction(&mut self, direction: Direction) {
        let new_position = self.map.move_in_direction(self.position, direction);
        if new_position.is_none() {
            // For some reason, the new position is out of bounds. This should
            // not be possible as the wall should have prevented the robot from
            // moving out of bounds.
            println!(
                "{}",
                "The robot is trying to move out of bounds.".red().bold()
            );
            return;
        }

        let new_position = new_position.unwrap();

        let value = self.map.get(new_position);
        match value {
            Some(&FREE_SPACE) => self.move_to_free_space(new_position),
            Some(&BOX) => self.move_to_box(new_position, direction),
            Some(&WALL) => (),
            Some(_) => {
                // The robot is trying to move into an invalid cell.
                println!(
                    "{}",
                    "The robot is trying to move into an invalid cell."
                        .red()
                        .bold()
                );
            }
            None => (),
        }
    }

    /// Moves the robot into the free space by updating the position.
    ///
    /// # Arguments
    ///
    /// * `to` - A tuple of the new position of the robot. The tuple is a
    ///   pair of the column and row of the new position.
    ///
    fn move_to_free_space(&mut self, to: (usize, usize)) {
        self.position = to;
    }

    /// Moves the robot to the box and pushes the box into the free space.
    /// The robot moves into the free space after pushing the box.
    ///
    /// It is possible to push multiple boxes at once as long as there is a free
    /// space behind the boxes. If there is a wall behind the chain of boxes, the
    /// robot cannot push the boxes.
    ///
    /// # Arguments
    ///
    /// * `to` - A tuple of the new position of the robot. The tuple is a
    ///   pair of the column and row of the new position.
    /// * `direction` - The direction in which the robot should move. The direction
    ///   can be one of the four cardinal directions: up, down, left, right represented
    ///   by the `Direction` enum.
    ///
    fn move_to_box(&mut self, to: (usize, usize), direction: Direction) {
        let mut free_space = None;
        let iter = self.map.in_direction(to, direction);
        for coord in iter {
            let value = self.map.get(coord);
            match value {
                Some(&FREE_SPACE) => {
                    free_space = Some(coord);
                    break;
                }
                Some(&BOX) => (),
                Some(&WALL) => break,
                None => break,
                Some(_) => (),
            }
        }

        if let Some(free_space) = free_space {
            // We found a free space behind the box. Push all the boxes
            // along in that direction. We can simulate this by swapping
            // the box with the free space. Then we move the robot forward.
            self.map.swap(to, free_space);
            self.position = to;
        }
    }

    fn find_all_boxes(&self) -> Vec<(usize, usize)> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, &cell)| if cell == BOX { Some((x, y)) } else { None })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    }

    fn print(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let colored_cell = if (x, y) == (self.position.1, self.position.0) {
                    '@'.to_string().yellow().bold()
                } else {
                    match cell {
                        FREE_SPACE => FREE_SPACE.to_string().white(),
                        BOX => BOX.to_string().blue(),
                        WALL => WALL.to_string().red(),
                        _ => cell.to_string().normal(),
                    }
                };
                print!("{}", colored_cell);
            }
            println!();
        }
    }
}

fn find_robot(map: &Matrix<char>) -> Option<(usize, usize)> {
    for (col, row_chars) in map.iter().enumerate() {
        for (row, &cell) in row_chars.iter().enumerate() {
            if cell == '@' {
                return Some((col, row));
            }
        }
    }
    None
}

/// Calculates the CPS value of the position. This is calculated by (100 * row) + column.
fn gps_value(position: &(usize, usize)) -> usize {
    let (col, row) = position;
    (100 * row) + col
}

type Direction = (isize, isize);
type Directions = Vec<Direction>;

fn convert_direction(direction: Direction) -> &'static str {
    let (x, y) = direction;
    match (x, y) {
        (0, -1) => "^",
        (0, 1) => "v",
        (-1, 0) => "<",
        (1, 0) => ">",
        _ => "Unknown",
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

fn parse(input: &str) -> (Map, Directions) {
    let (map_str, directions_str) = input
        .split("\n\n")
        .collect_tuple()
        .expect("Invalid input format");
    let map_data = map_str
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut matrix = Matrix::from_rows(map_data).expect("Invalid matrix");
    let position = find_robot(&matrix).expect("Robot not found");
    matrix[position] = '.'; // Overwrite the robot position with a free space.
    let map = Map {
        map: matrix,
        position,
    };
    let directions = directions_str
        .chars()
        .filter_map(|line| match line {
            '^' => Some(directions::N),
            'v' => Some(directions::S),
            '<' => Some(directions::W),
            '>' => Some(directions::E),
            _ => None,
        })
        .collect::<Vec<_>>();
    (map, directions)
}

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let (mut map, directions) = parse(&puzzle_input);
    directions.into_iter().for_each(|direction| {
        map.move_direction(direction);
    });
    map.print();

    let box_gps_sum: usize = map.find_all_boxes().iter().map(gps_value).sum();
    println!("The sum of the GPS values of the boxes is: {}", box_gps_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT: &str = r#"##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^"#;

    #[test]
    fn test_parse() {
        let (map, directions) = parse(PUZZLE_INPUT);
        assert_eq!(map.position, (4, 4));
        assert_eq!(directions.len(), 700);
        assert_eq!(map.map.columns, 10);
        assert_eq!(map.map.rows, 10);
    }
}
