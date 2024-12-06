use std::{char, fmt, fs::File, io::Read};

/// Custom error type for invalid maps.
#[derive(Debug)]
struct MapError {
    details: String,
}

impl MapError {
    fn new(msg: &str) -> MapError {
        MapError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

/// Represents the map of the puzzle.
#[derive(Debug)]
struct Map {
    /// The map of the puzzle.
    map: Vec<Vec<char>>,

    /// The height and width of the map.
    height: usize,
    width: usize,

    /// The position of the guard.
    position: (usize, usize),
    direction: char,
}

impl Map {
    const UP_CHAR: char = '^';
    const DOWN_CHAR: char = 'v';
    const LEFT_CHAR: char = '<';
    const RIGHT_CHAR: char = '>';

    fn new(map: Vec<Vec<char>>) -> Result<Map, MapError> {
        let height = map.len();
        let width = map[0].len();
        let position = Self::find_guard(&map)?;
        let direction = map[position.1][position.0];
        Ok(Self {
            map,
            height,
            width,
            position,
            direction,
        })
    }

    fn find_guard(map: &[Vec<char>]) -> Result<(usize, usize), MapError> {
        for (y, row) in map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if *c == Self::UP_CHAR
                    || *c == Self::DOWN_CHAR
                    || *c == Self::LEFT_CHAR
                    || *c == Self::RIGHT_CHAR
                {
                    return Ok((x, y));
                }
            }
        }
        Err(MapError::new("Guard not found"))
    }

    fn walk_path(&mut self) -> Result<(), MapError> {
        loop {
            let prev_position = self.position;
            let c = self.peek();
            match c {
                Some('#') => self.rotate_guard(),
                Some('.') => self.move_guard()?,
                Some('X') => self.move_guard()?,
                Some(_) => self.rotate_guard(),
                None => {
                    self.mark_visited(prev_position);
                    break;
                }
            }
            self.mark_visited(prev_position);
        }
        Ok(())
    }

    fn peek(&self) -> Option<char> {
        match self.direction {
            Self::UP_CHAR => self.peek_up(),
            Self::DOWN_CHAR => self.peek_down(),
            Self::LEFT_CHAR => self.peek_left(),
            Self::RIGHT_CHAR => self.peek_right(),
            _ => None,
        }
    }

    fn peek_up(&self) -> Option<char> {
        let (x, y) = self.position;
        if y == 0 {
            None
        } else {
            Some(self.map[y - 1][x])
        }
    }

    fn peek_down(&self) -> Option<char> {
        let (x, y) = self.position;
        if y == self.height - 1 {
            None
        } else {
            Some(self.map[y + 1][x])
        }
    }

    fn peek_left(&self) -> Option<char> {
        let (x, y) = self.position;
        if x == 0 {
            None
        } else {
            Some(self.map[y][x - 1])
        }
    }

    fn peek_right(&self) -> Option<char> {
        let (x, y) = self.position;
        if x == self.width - 1 {
            None
        } else {
            Some(self.map[y][x + 1])
        }
    }

    fn move_guard(&mut self) -> Result<(), MapError> {
        match self.direction {
            Self::UP_CHAR => self.move_up(),
            Self::DOWN_CHAR => self.move_down(),
            Self::LEFT_CHAR => self.move_left(),
            Self::RIGHT_CHAR => self.move_right(),
            _ => Err(MapError::new("Invalid direction")),
        }
    }

    fn move_up(&mut self) -> Result<(), MapError> {
        let (x, y) = self.position;
        if y == 0 {
            Err(MapError::new("Out of bounds"))
        } else {
            self.position = (x, y - 1);
            Ok(())
        }
    }

    fn move_down(&mut self) -> Result<(), MapError> {
        let (x, y) = self.position;
        if y == self.height - 1 {
            Err(MapError::new("Out of bounds"))
        } else {
            self.position = (x, y + 1);
            Ok(())
        }
    }

    fn move_left(&mut self) -> Result<(), MapError> {
        let (x, y) = self.position;
        if x == 0 {
            Err(MapError::new("Out of bounds"))
        } else {
            self.position = (x - 1, y);
            Ok(())
        }
    }

    fn move_right(&mut self) -> Result<(), MapError> {
        let (x, y) = self.position;
        if x == self.width - 1 {
            Err(MapError::new("Out of bounds"))
        } else {
            self.position = (x + 1, y);
            Ok(())
        }
    }

    fn rotate_guard(&mut self) {
        let new_direction = match self.direction {
            Self::UP_CHAR => Self::RIGHT_CHAR,
            Self::RIGHT_CHAR => Self::DOWN_CHAR,
            Self::DOWN_CHAR => Self::LEFT_CHAR,
            Self::LEFT_CHAR => Self::UP_CHAR,
            _ => self.direction,
        };
        self.direction = new_direction;
    }

    fn mark_visited(&mut self, prev_position: (usize, usize)) {
        let (x, y) = prev_position;
        self.map[y][x] = 'X';
    }

    fn count_visited(&self) -> usize {
        let mut count = 0;
        for row in &self.map {
            for c in row {
                if *c == 'X' {
                    count += 1;
                }
            }
        }
        count
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for (y, row) in self.map.iter().enumerate() {
            for (x, c) in row.iter().enumerate() {
                if (x, y) == self.position {
                    s.push(self.direction);
                } else {
                    s.push(*c);
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
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

/// Parses the input of the puzzle and returns an initialised map.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input of the puzzle.
///
/// # Returns
///
/// * `Map` - The initialised map of the puzzle.
///
fn parse_input(input: &str) -> Result<Map, MapError> {
    let mut map = Vec::new();
    for line in input.lines() {
        let row: Vec<char> = line.chars().collect();
        map.push(row);
    }
    Map::new(map)
}

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let mut map = parse_input(&puzzle_input).expect("Failed to parse input");
    map.walk_path().expect("Failed to walk path");
    println!("{}", map);
    println!("Visited: {}", map.count_visited());
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_parse() {
        let map = parse_input(PUZZLE_INPUT).expect("Failed to parse input");
        assert_eq!(map.height, 10);
        assert_eq!(map.width, 10);
        assert_eq!(map.position, (4, 6));
    }

    #[test]
    fn test_walk_path() {
        let mut map = parse_input(PUZZLE_INPUT).expect("Failed to parse input");
        map.walk_path().expect("Failed to walk path");
        assert_eq!(map.position, (7, 9));
    }

    #[test]
    fn test_count_visited() {
        let mut map = parse_input(PUZZLE_INPUT).expect("Failed to parse input");
        map.walk_path().expect("Failed to walk path");
        assert_eq!(map.count_visited(), 41);
    }
}
