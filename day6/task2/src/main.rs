use std::{fmt, fs::File, io::Read};

/// Custom error type for invalid maps.
#[derive(Debug)]
enum MapError {
    InvalidMap { details: String },
    WalkLoopDetected { details: String },
}

impl MapError {
    fn new_invalid_map(msg: &str) -> MapError {
        MapError::InvalidMap {
            details: msg.to_string(),
        }
    }

    fn new_walk_loop_detected(msg: &str) -> MapError {
        MapError::WalkLoopDetected {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for MapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MapError::InvalidMap { details } => write!(f, "Invalid map: {}", details),
            MapError::WalkLoopDetected { details } => write!(f, "Walk loop detected: {}", details),
        }
    }
}

/// Represents the map of the puzzle.
#[derive(Debug)]
struct Map {
    /// The map of the puzzle.
    map: Vec<Vec<String>>,

    /// The height and width of the map.
    height: usize,
    width: usize,

    /// The position of the guard.
    initial_position: (usize, usize),
    position: (usize, usize),
    direction: String,

    /// Potential loop coords
    loop_obstacle_coords: Vec<(usize, usize)>,
}

impl Map {
    const UP_CHAR: &'static str = "^";
    const DOWN_CHAR: &'static str = "v";
    const LEFT_CHAR: &'static str = "<";
    const RIGHT_CHAR: &'static str = ">";
    const VISITED_UP_CHAR: &'static str = "u";
    const VISITED_DOWN_CHAR: &'static str = "d";
    const VISITED_LEFT_CHAR: &'static str = "l";
    const VISITED_RIGHT_CHAR: &'static str = "r";
    const OBSTACLE_CHAR: &'static str = "#";
    const FREE_SPACE_CHAR: &'static str = ".";

    fn new(map: Vec<Vec<String>>) -> Result<Map, MapError> {
        let height = map.len();
        let width = map[0].len();
        let position = Self::find_guard(&map)?;
        let direction = map[position.1][position.0].clone();
        Ok(Self {
            map,
            height,
            width,
            initial_position: position,
            position,
            direction,
            loop_obstacle_coords: Vec::new(),
        })
    }

    fn find_guard(map: &[Vec<String>]) -> Result<(usize, usize), MapError> {
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
        Err(MapError::new_invalid_map("Guard not found"))
    }

    fn walk_path(&mut self) -> Result<(), MapError> {
        let mut is_loop = false;
        loop {
            let prev_position = self.position;
            self.mark_visited(prev_position);
            let c = self.peek();

            match (c, self.direction.as_str()) {
                (Some(c), _) if c == Self::OBSTACLE_CHAR => {
                    // We have reached an obstacle. We need to rotate the guard.
                    self.rotate_guard()
                }
                (Some(c), d)
                    if (c.contains(Self::VISITED_UP_CHAR) && d == Self::UP_CHAR)
                        || (c.contains(Self::VISITED_DOWN_CHAR) && d == Self::DOWN_CHAR)
                        || (c.contains(Self::VISITED_LEFT_CHAR) && d == Self::LEFT_CHAR)
                        || (c.contains(Self::VISITED_RIGHT_CHAR) && d == Self::RIGHT_CHAR) =>
                {
                    // We are going in the same direction we have previously
                    // visited. This is a loop.
                    is_loop = true;
                    break;
                }
                (Some(..), _) => {
                    // We are moving and there are no obstacles in our way.
                    // This is also not a loop even through we have visited
                    // this space before going in a different direction.
                    self.move_guard()?
                }
                (None, _) => {
                    // We have reached the end of the map.
                    break;
                }
            }

            // At this point we have moved the guard. We need to check if we are
            // able to create a loop.
            if self.check_loop_obstacle() {
                // println!("{}", self);
            }
        }
        if is_loop {
            Err(MapError::new_walk_loop_detected("Loop detected"))
        } else {
            Ok(())
        }
    }

    fn check_loop_obstacle(&mut self) -> bool {
        let mut found_loop_obstacle = false;
        match self.direction.as_str() {
            Self::UP_CHAR => {
                if let Some(c) = self.peek_up() {
                    if self.check_if_visited_right(self.position.0, self.position.1)
                        && c == Self::FREE_SPACE_CHAR
                    {
                        let obstacle_position = (self.position.0, self.position.1 - 1);
                        self.loop_obstacle_coords.push(obstacle_position);
                        found_loop_obstacle = true;
                    }
                }
            }
            Self::RIGHT_CHAR => {
                if let Some(c) = self.peek_right() {
                    if self.check_if_visited_down(self.position.0, self.position.1)
                        && c == Self::FREE_SPACE_CHAR
                    {
                        let obstacle_position = (self.position.0 + 1, self.position.1);
                        self.loop_obstacle_coords.push(obstacle_position);
                        found_loop_obstacle = true;
                    }
                }
            }
            Self::DOWN_CHAR => {
                if let Some(c) = self.peek_down() {
                    if self.check_if_visited_left(self.position.0, self.position.1)
                        && c == Self::FREE_SPACE_CHAR
                    {
                        let obstacle_position = (self.position.0, self.position.1 + 1);
                        self.loop_obstacle_coords.push(obstacle_position);
                        found_loop_obstacle = true;
                    }
                }
            }
            Self::LEFT_CHAR => {
                if let Some(c) = self.peek_left() {
                    if self.check_if_visited_up(self.position.0, self.position.1)
                        && c == Self::FREE_SPACE_CHAR
                    {
                        let obstacle_position = (self.position.0 - 1, self.position.1);
                        self.loop_obstacle_coords.push(obstacle_position);
                        found_loop_obstacle = true;
                    }
                }
            }
            _ => {}
        }
        found_loop_obstacle
    }

    fn count_loop_obstacles(&self) -> usize {
        // Make sure that there ar no obstacles placed on the guard's initial
        // position.
        self.loop_obstacle_coords
            .iter()
            .filter(|(x, y)| (*x, *y) != self.initial_position)
            .count()
    }

    // Loop over all the positions above the current position and check if
    // any of them have been visited in the direction we are moving. If we
    // encounter an obstacle or an out of bounds position, we stop.
    //
    // We need to check one cell at a time because we need to check if the
    // obstacle is before we hit a visited cell.
    fn check_if_visited_up(&self, x: usize, y: usize) -> bool {
        for y in (0..y).rev() {
            let c = &self.map[y][x];
            if c.contains(Self::VISITED_UP_CHAR) {
                return true;
            }
            if c == Self::OBSTACLE_CHAR {
                break;
            }
        }
        false
    }

    fn check_if_visited_down(&self, x: usize, y: usize) -> bool {
        for y in y + 1..self.height {
            let c = &self.map[y][x];
            if c.contains(Self::VISITED_DOWN_CHAR) {
                return true;
            }
            if c == Self::OBSTACLE_CHAR {
                break;
            }
        }
        false
    }

    fn check_if_visited_left(&self, x: usize, y: usize) -> bool {
        for x in (0..x).rev() {
            let c = &self.map[y][x];
            if c.contains(Self::VISITED_LEFT_CHAR) {
                return true;
            }
            if c == Self::OBSTACLE_CHAR {
                break;
            }
        }
        false
    }

    fn check_if_visited_right(&self, x: usize, y: usize) -> bool {
        for x in x + 1..self.width {
            let c = &self.map[y][x];
            if c.contains(Self::VISITED_RIGHT_CHAR) {
                return true;
            }
            if c == Self::OBSTACLE_CHAR {
                break;
            }
        }
        false
    }

    fn peek(&self) -> Option<&String> {
        match self.direction.as_str() {
            Self::UP_CHAR => self.peek_up(),
            Self::DOWN_CHAR => self.peek_down(),
            Self::LEFT_CHAR => self.peek_left(),
            Self::RIGHT_CHAR => self.peek_right(),
            _ => None,
        }
    }

    fn peek_up(&self) -> Option<&String> {
        let (x, y) = self.position;
        if y == 0 {
            None
        } else {
            Some(&self.map[y - 1][x])
        }
    }

    fn peek_down(&self) -> Option<&String> {
        let (x, y) = self.position;
        if y == self.height - 1 {
            None
        } else {
            Some(&self.map[y + 1][x])
        }
    }

    fn peek_left(&self) -> Option<&String> {
        let (x, y) = self.position;
        if x == 0 {
            None
        } else {
            Some(&self.map[y][x - 1])
        }
    }

    fn peek_right(&self) -> Option<&String> {
        let (x, y) = self.position;
        if x == self.width - 1 {
            None
        } else {
            Some(&self.map[y][x + 1])
        }
    }

    fn move_guard(&mut self) -> Result<(), MapError> {
        match self.direction {
            ref dir if dir == Self::UP_CHAR => self.move_up(),
            ref dir if dir == Self::DOWN_CHAR => self.move_down(),
            ref dir if dir == Self::LEFT_CHAR => self.move_left(),
            ref dir if dir == Self::RIGHT_CHAR => self.move_right(),
            _ => Err(MapError::new_invalid_map("Invalid direction")),
        }
    }

    fn move_up(&mut self) -> Result<(), MapError> {
        let (x, y) = self.position;
        if y == 0 {
            Err(MapError::new_invalid_map("Out of bounds"))
        } else {
            self.position = (x, y - 1);
            Ok(())
        }
    }

    fn move_down(&mut self) -> Result<(), MapError> {
        let (x, y) = self.position;
        if y == self.height - 1 {
            Err(MapError::new_invalid_map("Out of bounds"))
        } else {
            self.position = (x, y + 1);
            Ok(())
        }
    }

    fn move_left(&mut self) -> Result<(), MapError> {
        let (x, y) = self.position;
        if x == 0 {
            Err(MapError::new_invalid_map("Out of bounds"))
        } else {
            self.position = (x - 1, y);
            Ok(())
        }
    }

    fn move_right(&mut self) -> Result<(), MapError> {
        let (x, y) = self.position;
        if x == self.width - 1 {
            Err(MapError::new_invalid_map("Out of bounds"))
        } else {
            self.position = (x + 1, y);
            Ok(())
        }
    }

    fn rotate_guard(&mut self) {
        let new_direction = match self.direction.as_str() {
            Self::UP_CHAR => Self::RIGHT_CHAR.to_string(),
            Self::RIGHT_CHAR => Self::DOWN_CHAR.to_string(),
            Self::DOWN_CHAR => Self::LEFT_CHAR.to_string(),
            Self::LEFT_CHAR => Self::UP_CHAR.to_string(),
            _ => self.direction.clone(),
        };
        self.direction = new_direction;
    }

    fn mark_visited(&mut self, prev_position: (usize, usize)) {
        let (x, y) = prev_position;
        let current_string = &self.map[y][x];
        // Append the current direction to the visited cell if it does
        // not already contain it.
        if current_string.contains(self.direction.as_str()) {
            return;
        }
        let visited_string = match self.direction.as_str() {
            Self::UP_CHAR => Self::VISITED_UP_CHAR,
            Self::DOWN_CHAR => Self::VISITED_DOWN_CHAR,
            Self::LEFT_CHAR => Self::VISITED_LEFT_CHAR,
            Self::RIGHT_CHAR => Self::VISITED_RIGHT_CHAR,
            _ => "",
        };
        self.map[y][x].push_str(visited_string);
    }

    fn count_visited(&self) -> usize {
        let mut count = 0;
        for row in &self.map {
            for c in row {
                if c.contains(Self::VISITED_UP_CHAR)
                    || c.contains(Self::VISITED_DOWN_CHAR)
                    || c.contains(Self::VISITED_LEFT_CHAR)
                    || c.contains(Self::VISITED_RIGHT_CHAR)
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn convert_string_to_char(c: &str) -> char {
        let has_up = c.contains(Self::VISITED_UP_CHAR);
        let has_down = c.contains(Self::VISITED_DOWN_CHAR);
        let has_left = c.contains(Self::VISITED_LEFT_CHAR);
        let has_right = c.contains(Self::VISITED_RIGHT_CHAR);

        if c == Self::OBSTACLE_CHAR {
            return '#';
        }
        if c == Self::FREE_SPACE_CHAR {
            return '.';
        }

        // Pick an appropriate box drawing character based on the direction
        // the guard has visited the cell.
        match (has_up, has_down, has_left, has_right) {
            (false, false, false, false) => '?',
            (true, false, false, false) => '│',
            (false, true, false, false) => '│',
            (false, false, true, false) => '─',
            (false, false, false, true) => '─',
            (true, true, false, false) => '│',
            (false, false, true, true) => '─',
            (true, false, true, false) => '┘',
            (true, false, false, true) => '└',
            (false, true, true, false) => '┐',
            (false, true, false, true) => '┌',
            (true, true, true, false) => '┤',
            (true, true, false, true) => '├',
            (true, false, true, true) => '┴',
            (false, true, true, true) => '┬',
            (true, true, true, true) => '┼',
        }
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        s.push_str("   ");
        for x in 0..self.width {
            s.push_str(&format!("{:1}", x));
        }
        s.push('\n');
        for (y, row) in self.map.iter().enumerate() {
            s.push_str(&format!("{:2} ", y));
            for (x, c) in row.iter().enumerate() {
                if (x, y) == self.position {
                    s.push_str(&self.direction);
                } else {
                    let c = Self::convert_string_to_char(c);
                    s.push(c);
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
        let row = line.chars().map(|c| c.to_string()).collect();
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
    println!("Loop obstacles: {}", map.count_loop_obstacles());
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

    const PUZZLE_INPUT_LOOP: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#.#^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_parse() {
        let map = parse_input(PUZZLE_INPUT).expect("Failed to parse input");
        assert_eq!(map.height, 10);
        assert_eq!(map.width, 10);
        assert_eq!(map.position, (4, 6));
        assert_eq!(map.direction, Map::UP_CHAR);
    }

    #[test]
    fn test_walk_path() {
        let mut map = parse_input(PUZZLE_INPUT).expect("Failed to parse input");
        map.walk_path().expect("Failed to walk path");
        assert_eq!(map.position, (7, 9));
    }

    #[test]
    fn test_walk_path_loop() {
        let mut map = parse_input(PUZZLE_INPUT_LOOP).expect("Failed to parse input");
        let result = map.walk_path();
        assert!(result.is_err());
        assert!(matches!(result, Err(MapError::WalkLoopDetected { .. })));
    }

    #[test]
    fn test_count_visited() {
        let mut map = parse_input(PUZZLE_INPUT).expect("Failed to parse input");
        map.walk_path().expect("Failed to walk path");
        println!("{}", map);
        assert_eq!(map.count_visited(), 41);
    }

    #[test]
    fn test_count_potential_loop_obstacles() {
        let mut map = parse_input(PUZZLE_INPUT).expect("Failed to parse input");
        map.walk_path().expect("Failed to walk path");
        assert_eq!(map.loop_obstacle_coords.len(), 6);
        assert_eq!(
            map.loop_obstacle_coords,
            vec![(3, 6), (6, 7), (3, 8), (1, 8), (7, 7), (7, 9)]
        );
        assert_eq!(map.count_loop_obstacles(), 6);
    }
}
