use itertools::Itertools;
use std::{collections::HashMap, fmt, fs::File, io::Read};

#[derive(Debug, PartialEq)]
enum MapError {}

#[derive(Debug, PartialEq, Clone)]
enum SitesOfInterest {
    AntiNode(char),
    Antenna(char),
}

#[derive(Debug, PartialEq, Clone)]
struct Cell {
    x: usize,
    y: usize,
    sites_of_interest: Vec<SitesOfInterest>,
}

type AntennaMap = HashMap<char, Vec<(usize, usize)>>;
type AntinodeMap = HashMap<char, Vec<(usize, usize)>>;

#[derive(Debug, PartialEq, Clone)]
struct Map {
    cells: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut map_str = String::new();
        let width = self.cells[0].len();

        // Add the top axis
        map_str.push_str("   ");
        for x in 0..width {
            map_str.push_str(&format!("{:2} ", x));
        }
        map_str.push('\n');

        for (y, row) in self.cells.iter().enumerate() {
            // Add the left axis
            map_str.push_str(&format!("{:2} ", y));
            for cell in row {
                let cell_str = match cell.sites_of_interest.len() {
                    0 => '.',
                    1 => match cell.sites_of_interest[0] {
                        SitesOfInterest::Antenna(c) => c,
                        SitesOfInterest::AntiNode(c) => c,
                    },
                    _ => 'X',
                };
                map_str.push_str(&format!("{}  ", cell_str));
            }
            map_str.push('\n');
        }
        write!(f, "{}", map_str)
    }
}

impl Map {
    /// Returns a hashmap of all the antennas on the map. The key is the antenna
    /// type and the value is a vector of tuples containing the x and y coordinates
    /// of the antenna.
    ///
    /// # Returns
    ///
    /// * `AntennaMap` - A hashmap where the key is the antenna type and the value
    /// is a vector of tuples containing the x and y coordinates of the antenna.
    ///
    fn get_antennas(&self) -> AntennaMap {
        let mut antennas = AntennaMap::new();
        for row in &self.cells {
            for cell in row {
                for site in &cell.sites_of_interest {
                    if let SitesOfInterest::Antenna(c) = site {
                        antennas.entry(*c).or_default().push((cell.x, cell.y));
                    }
                }
            }
        }
        antennas
    }

    /// Adds all the antinodes to the map. The antinodes are calculated for all
    /// pairs of antennas of the same type. The antinodes are then stored in the
    /// cells of the map.
    fn add_all_antinodes(&mut self) {
        let antinodes = self.find_all_antinodes();
        for (antenna, coords) in antinodes {
            for coord in coords {
                self.cells[coord.1][coord.0]
                    .sites_of_interest
                    .push(SitesOfInterest::AntiNode(antenna));
            }
        }
    }

    /// Calculate antinodes for all pairs of antennas. The antinodes are calculated
    /// for all pairs of antennas of the same type. The antinodes are then stored
    /// in a hashmap where the key is the antenna type. This function will also
    /// remove duplicates and sort the antinodes for each antenna but will not
    /// remove duplications between antenna types.
    ///
    /// # Returns
    ///
    /// * `HashMap<char, Vec<(usize, usize)>>` - A hashmap where the key is the antenna
    ///
    fn find_all_antinodes(&self) -> AntinodeMap {
        let antennas = self.get_antennas();
        let mut antinodes = AntennaMap::new();
        for (antenna, coords) in &antennas {
            coords.iter().combinations(2).for_each(|v| {
                let a = v[0];
                let b = v[1];
                if a != b {
                    let new_antinodes = self.find_antinodes_within_map(*a, *b);
                    for antinode in new_antinodes {
                        antinodes.entry(*antenna).or_default().push(antinode);
                    }
                }
            });
        }

        // Remove duplicates and sort the antinodes
        antinodes.iter_mut().for_each(|(_, v)| {
            v.sort();
            v.dedup();
        });

        antinodes
    }

    /// Calculates all the antinode of two antennas. The antinodes are all the points
    /// on a line drawn from two antennas. The distance between the antennas is
    /// calculated and the antinodes must appear with this as a minimum distance.
    /// The antennas themselves are also considered antinodes.
    ///
    /// # Arguments
    ///
    /// * `a` - A tuple of the x and y coordinates of the first antenna.
    /// * `b` - A tuple of the x and y coordinates of the second antenna.
    ///
    /// # Returns
    ///
    /// * `Vec<(usize, usize)>` - A vector of tuples containing the x and y
    /// coordinates of the antinodes.
    ///
    fn find_antinodes_within_map(
        &self,
        a: (usize, usize),
        b: (usize, usize),
    ) -> Vec<(usize, usize)> {
        let (x1, y1) = a;
        let (x2, y2) = b;
        let dx = x2 as isize - x1 as isize;
        let dy = y2 as isize - y1 as isize;

        let mut antinodes = vec![a, b];

        // Go forwards in the line
        let mut step = 1;
        loop {
            let x = x2 as isize + dx * step;
            let y = y2 as isize + dy * step;
            if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
                break;
            }
            step += 1;
            antinodes.push((x as usize, y as usize));
        }

        // Go backwards in the line
        step = -1;
        loop {
            let x = x1 as isize + dx * step;
            let y = y1 as isize + dy * step;
            if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
                break;
            }
            step -= 1;
            antinodes.push((x as usize, y as usize));
        }

        // Remove duplicates and sort the antinodes
        antinodes.sort();
        antinodes.dedup();
        antinodes
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
/// * `Result<String, std::io::Error>` - The contents of the file as a string,
/// or an error if the file could not be read.
fn read_to_string(puzzle_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(puzzle_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Parses the input string and returns a result of the map.
///
/// # Arguments
///
/// * `input` - A string slice that holds the input string.
///
/// # Returns
///
/// * `Result<Map, MapError>` - The map struct, or an error if the input could
/// not be parsed.
///
fn parse_input(input: &str) -> Result<Map, MapError> {
    let cells: Vec<Vec<Cell>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    let sites_of_interest = match c {
                        '.' => vec![],
                        _ => vec![SitesOfInterest::Antenna(c)],
                    };
                    Cell {
                        x,
                        y,
                        sites_of_interest,
                    }
                })
                .collect()
        })
        .collect();
    let width = cells[0].len();
    let height = cells.len();
    Ok(Map {
        cells,
        width,
        height,
    })
}

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let mut map = parse_input(&puzzle_input).expect("Failed to parse input");
    println!("Map:\n{}", map);
    let antinodes = map.find_all_antinodes();
    let count = antinodes.values().flatten().unique().count();
    println!("Antinode count: {}", count);
    map.add_all_antinodes();
    println!("Map with Antinodes:\n{}", map);
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    const PUZZLE_INPUT2: &str = r#"..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
.........."#;

    const PUZZLE_INPUT3: &str = r#"..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
.........."#;

    const PUZZLE_INPUT4: &str = r#"T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
.........."#;

    fn blank_map(width: usize, height: usize) -> Map {
        let blank_cells = (0..height)
            .map(|y| {
                (0..width)
                    .map(|x| Cell {
                        x,
                        y,
                        sites_of_interest: vec![],
                    })
                    .collect::<Vec<Cell>>()
            })
            .collect::<Vec<Vec<Cell>>>();
        Map {
            cells: blank_cells,
            width,
            height,
        }
    }

    fn insert_antenna(map: &mut Map, coords: Vec<(usize, usize)>, c: char) {
        for (x, y) in coords {
            map.cells[y][x]
                .sites_of_interest
                .push(SitesOfInterest::Antenna(c));
        }
    }

    fn parsed_map() -> Map {
        let initial_map = blank_map(12, 12);
        let antennas_coords_a = vec![(6, 5), (8, 8), (9, 9)];
        let antennas_coords_0 = vec![(8, 1), (5, 2), (7, 3), (4, 4)];
        let mut map_with_antennas = initial_map.clone();
        insert_antenna(&mut map_with_antennas, antennas_coords_a, 'A');
        insert_antenna(&mut map_with_antennas, antennas_coords_0, '0');
        map_with_antennas
    }

    fn parsed_map2() -> Map {
        let initial_map = blank_map(10, 10);
        let antennas_coords_a = vec![(4, 3), (5, 5)];
        let mut map_with_antennas = initial_map.clone();
        insert_antenna(&mut map_with_antennas, antennas_coords_a, 'a');
        map_with_antennas
    }

    fn parsed_map3() -> Map {
        let initial_map = blank_map(10, 10);
        let antennas_coords_a = vec![(4, 3), (5, 5), (8, 4)];
        let mut map_with_antennas = initial_map.clone();
        insert_antenna(&mut map_with_antennas, antennas_coords_a, 'a');
        map_with_antennas
    }

    fn parsed_map4() -> Map {
        let initial_map = blank_map(10, 10);
        let antennas_coords_t = vec![(0, 0), (1, 2), (3, 1)];
        let mut map_with_antennas = initial_map.clone();
        insert_antenna(&mut map_with_antennas, antennas_coords_t, 'T');
        map_with_antennas
    }

    #[test]
    fn test_parse() {
        let map = parse_input(PUZZLE_INPUT).expect("Failed to parse input");
        let map_with_antennas = parsed_map();
        println!("Map:\n{}", map);
        println!("Map with Antennas:\n{}", map_with_antennas);
        assert_eq!(map, map_with_antennas);
    }

    #[test]
    fn test_parse2() {
        let map = parse_input(PUZZLE_INPUT2).expect("Failed to parse input");
        let map_with_antennas = parsed_map2();
        println!("Map:\n{}", map);
        println!("Map with Antennas:\n{}", map_with_antennas);
        assert_eq!(map, map_with_antennas);
    }

    #[test]
    fn test_parse3() {
        let map = parse_input(PUZZLE_INPUT3).expect("Failed to parse input");
        let map_with_antennas = parsed_map3();
        println!("Map:\n{}", map);
        println!("Map with Antennas:\n{}", map_with_antennas);
        assert_eq!(map, map_with_antennas);
    }

    #[test]
    fn test_parse4() {
        let map = parse_input(PUZZLE_INPUT4).expect("Failed to parse input");
        let map_with_antennas = parsed_map4();
        println!("Map:\n{}", map);
        println!("Map with Antennas:\n{}", map_with_antennas);
        assert_eq!(map, map_with_antennas);
    }

    #[test]
    fn test_get_antennas() {
        let map = parsed_map();
        let antennas = map.get_antennas();
        let mut expected_antennas = AntennaMap::new();
        expected_antennas.insert('A', vec![(6, 5), (8, 8), (9, 9)]);
        expected_antennas.insert('0', vec![(8, 1), (5, 2), (7, 3), (4, 4)]);
        assert_eq!(antennas, expected_antennas);
    }

    #[test]
    fn test_calculate_antinode() {
        let a = (1, 1);
        let b = (3, 3);
        let map = blank_map(8, 8);
        let antinodes = map.find_antinodes_within_map(a, b);
        let expected_antinodes = vec![(1, 1), (3, 3), (5, 5), (7, 7)]; // positive direction
        assert_eq!(antinodes, expected_antinodes);
        let antinodes2 = map.find_antinodes_within_map(b, a);
        let expected_antinodes2 = vec![(1, 1), (3, 3), (5, 5), (7, 7)]; // negative direction
        assert_eq!(antinodes2, expected_antinodes2);
    }

    #[test]
    fn test_find_all_antinodes() {
        let map = parsed_map2();
        let mut antinodes = map.find_all_antinodes();
        let mut expected_antinodes = AntinodeMap::new();
        expected_antinodes.insert('a', vec![(3, 1), (4, 3), (5, 5), (6, 7), (7, 9)]);
        antinodes.iter_mut().for_each(|(_, v)| v.sort());
        expected_antinodes.iter_mut().for_each(|(_, v)| v.sort());
        assert_eq!(antinodes, expected_antinodes);
    }

    #[test]
    fn test_find_all_antinodes2() {
        let map = parsed_map3();
        let mut antinodes = map.find_all_antinodes();
        let mut expected_antinodes = AntinodeMap::new();
        expected_antinodes.insert(
            'a',
            vec![
                (0, 2),
                (2, 6),
                (3, 1),
                (4, 3),
                (5, 5),
                (6, 7),
                (7, 9),
                (8, 4),
            ],
        );
        antinodes.iter_mut().for_each(|(_, v)| v.sort());
        expected_antinodes.iter_mut().for_each(|(_, v)| v.sort());
        assert_eq!(antinodes, expected_antinodes);
    }

    #[test]
    fn test_find_all_antinodes3() {
        let map = parsed_map();
        let antinodes: HashMap<char, Vec<(usize, usize)>> = map.find_all_antinodes();
        let count = antinodes.values().flatten().unique().count();
        assert_eq!(count, 34);
    }

    #[test]
    fn test_find_all_antinodes4() {
        let map = parsed_map4();
        let antinodes: HashMap<char, Vec<(usize, usize)>> = map.find_all_antinodes();
        let count = antinodes.values().flatten().unique().count();
        assert_eq!(count, 9);
    }
}
