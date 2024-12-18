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

    /// Calculates the antinode of two antennas. The antinode is twice the distance from the
    /// first antenna to the second antenna. The antinode is always outside of the two antennas.
    /// Also checks if the calculated antinode is valid within the map boundaries.
    fn calculate_and_validate_antinode(
        &self,
        a: (usize, usize),
        b: (usize, usize),
    ) -> Option<(usize, usize)> {
        // Account for out of bounds calculations
        let dx = (b.0 as isize - a.0 as isize) * 2;
        let dy = (b.1 as isize - a.1 as isize) * 2;
        let x = a.0 as isize + dx;
        let y = a.1 as isize + dy;
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            None
        } else {
            Some((x as usize, y as usize))
        }
    }

    /// Calculate antinodes for all pairs of antennas.
    fn find_all_antinodes(&self) -> AntinodeMap {
        let antennas = self.get_antennas();
        let mut antinodes = AntennaMap::new();
        for (antenna, coords) in &antennas {
            // Compute the cartesian product of the coords for the antennas
            // of the same type. This will give us all pairs of antennas.
            coords
                .iter()
                .cartesian_product(coords.iter())
                .for_each(|(a, b)| {
                    if a != b {
                        let antinode = self.calculate_and_validate_antinode(*a, *b);
                        if let Some(antinode) = antinode {
                            antinodes.entry(*antenna).or_default().push(antinode);
                        }
                    }
                });
        }
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
/// * `Result<String, std::io::Error>` - The contents of the file as a string, or an error if the file could not be read.
fn read_to_string(puzzle_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(puzzle_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

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
    let map = parse_input(&puzzle_input).expect("Failed to parse input");
    println!("Map:\n{}", map);
    let antinodes = map.find_all_antinodes();
    let count = antinodes.values().flatten().unique().count();
    println!("Antinode count: {}", count);
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
        let map = blank_map(7, 7);
        let antinode = map.calculate_and_validate_antinode(a, b);
        let antinode2 = map.calculate_and_validate_antinode(b, a);
        assert_eq!(antinode, Some((5, 5)));
        assert_eq!(antinode2, None); // out of bounds
    }

    #[test]
    fn test_find_all_antinodes() {
        let map = parsed_map2();
        let mut antinodes = map.find_all_antinodes();
        let mut expected_antinodes = AntinodeMap::new();
        expected_antinodes.insert('a', vec![(3, 1), (6, 7)]);
        antinodes.iter_mut().for_each(|(_, v)| v.sort());
        expected_antinodes.iter_mut().for_each(|(_, v)| v.sort());
        assert_eq!(antinodes, expected_antinodes);
    }

    #[test]
    fn test_find_all_antinodes2() {
        let map = parsed_map3();
        let mut antinodes = map.find_all_antinodes();
        let mut expected_antinodes = AntinodeMap::new();
        expected_antinodes.insert('a', vec![(3, 1), (6, 7), (0, 2), (2, 6)]);
        antinodes.iter_mut().for_each(|(_, v)| v.sort());
        expected_antinodes.iter_mut().for_each(|(_, v)| v.sort());
        assert_eq!(antinodes, expected_antinodes);
    }

    #[test]
    fn test_find_all_antinodes3() {
        let map = parsed_map();
        let antinodes: HashMap<char, Vec<(usize, usize)>> = map.find_all_antinodes();
        let count = antinodes.values().flatten().unique().count();
        assert_eq!(count, 14);
    }
}
