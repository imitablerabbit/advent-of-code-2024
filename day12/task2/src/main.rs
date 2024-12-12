use pathfinding::matrix::MatrixFormatError;
use pathfinding::prelude::Matrix;
use std::char;
use std::collections::{HashMap, HashSet};
use std::{fs::File, io::Read};

struct Plot {
    id: char,

    // Coordinates of the plot. These are (row, col) coordinates.
    coords: Vec<(usize, usize)>,

    // The edges in the plot. These are pairs of coordinates that make up the
    // start and end of the edge.
    edges: Vec<Edge>,

    area: usize,
    perimeter: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Edge {
    start: (usize, usize),
    end: (usize, usize),
    direction: Direction,
}

fn read_to_string(puzzle_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(puzzle_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Parses the input file and returns a 2d array of chars
/// If the input file is not formatted correctly, the function will return an
/// error as a result.
///
/// # Arguments
///
/// * `puzzle_input` - A string containing the contents of the input file.
///
/// # Returns
///
/// A 2d array of positive integers.
///
fn parse(puzzle_input: &str) -> Result<Matrix<char>, MatrixFormatError> {
    Matrix::from_rows(puzzle_input.lines().map(|l| l.chars()))
}

fn find_plots_dimensions(map: Matrix<char>) -> Vec<Plot> {
    let mut plots = vec![];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for point in map.keys() {
        if visited.contains(&point) {
            // Skip visited plots
            continue;
        }

        // Find the plots connected to this point.
        let start_char = map.get(point).expect("Invalid point");
        let reachable = map.bfs_reachable(point, false, |coord| {
            let c = map.get(coord).expect("Invalid point");
            c == start_char
        });
        visited.extend(reachable.iter().cloned());
        let coords: Vec<(usize, usize)> = reachable.iter().cloned().collect();
        let edges = plot_edges(&coords);
        let area = plot_area(&coords);
        let perimeter = plot_perimeter(&coords);
        let plot = Plot {
            id: *start_char,
            coords,
            edges,
            area,
            perimeter,
        };
        plots.push(plot);
    }

    plots
}

fn plot_area(coords: &[(usize, usize)]) -> usize {
    coords.len()
}

/// Given a list of coordinates, returns a list of edges. An edge consists
/// of a pair of coordinates that are the start of the edge and the end of the
/// edge. All the edges make up the boundary of the plot. We find the edges by
/// walking around the perimeter of the plot.
fn plot_edges(coords: &[(usize, usize)]) -> Vec<Edge> {
    let mut edges = vec![];
    let coors = coords.to_vec();

    // find the top left corner of the plot
    let start_node = coors
        .iter()
        .fold((usize::MAX, usize::MAX), |acc, (row, col)| {
            (acc.0.min(*row), acc.1.min(*col))
        });

    let mut move_direction = Direction::Right;
    let mut edge_direction = Direction::Up;
    let start_edge_direction = Direction::Up;

    let mut previous = start_node;
    let mut current = start_node;
    let mut edge_start = start_node;

    // Walk around the perimeter of the plot
    loop {
        let left = left_neighbour(current.0, current.1, &coors);
        let right = right_neighbour(current.0, current.1, &coors);
        let top = top_neighbour(current.0, current.1, &coors);
        let bottom = bottom_neighbour(current.0, current.1, &coors);

        match edge_direction {
            Direction::Up => {
                if top.is_some() {
                    // We are in an inner corner. We need to change direction.
                    let edge = Edge {
                        start: edge_start,
                        end: previous,
                        direction: edge_direction,
                    };
                    edges.push(edge);
                    move_direction = Direction::Up;
                    edge_direction = Direction::Left;
                    current = top.unwrap();
                    edge_start = current;
                    continue;
                }
            }

            Direction::Right => {
                if right.is_some() {
                    // We are in an inner corner. We need to change direction.
                    let edge = Edge {
                        start: edge_start,
                        end: previous,
                        direction: edge_direction,
                    };
                    edges.push(edge);
                    move_direction = Direction::Right;
                    edge_direction = Direction::Up;
                    previous = current;
                    current = right.unwrap();
                    edge_start = current;
                    continue;
                }
            }
            Direction::Down => {
                if bottom.is_some() {
                    // We are in an inner corner. We need to change direction.
                    let edge = Edge {
                        start: edge_start,
                        end: previous,
                        direction: edge_direction,
                    };
                    edges.push(edge);
                    move_direction = Direction::Down;
                    edge_direction = Direction::Right;
                    previous = current;
                    current = bottom.unwrap();
                    edge_start = current;
                    continue;
                }
            }
            Direction::Left => {
                if left.is_some() {
                    // We are in an inner corner. We need to change direction.
                    let edge = Edge {
                        start: edge_start,
                        end: previous,
                        direction: edge_direction,
                    };
                    edges.push(edge);
                    move_direction = Direction::Left;
                    edge_direction = Direction::Down;
                    previous = current;
                    current = left.unwrap();
                    edge_start = current;
                    continue;
                }
            }
        }

        match move_direction {
            Direction::Right => {
                if right.is_some() {
                    previous = current;
                    current = right.unwrap();
                } else {
                    let edge = Edge {
                        start: edge_start,
                        end: current,
                        direction: edge_direction,
                    };
                    edges.push(edge);
                    move_direction = Direction::Down;
                    edge_direction = Direction::Right;
                    edge_start = current;
                }
            }
            Direction::Down => {
                if bottom.is_some() {
                    previous = current;
                    current = bottom.unwrap();
                } else {
                    let edge = Edge {
                        start: edge_start,
                        end: current,
                        direction: edge_direction,
                    };
                    edges.push(edge);
                    move_direction = Direction::Left;
                    edge_direction = Direction::Down;
                    edge_start = current;
                }
            }
            Direction::Left => {
                if left.is_some() {
                    previous = current;
                    current = left.unwrap();
                } else {
                    let edge = Edge {
                        start: edge_start,
                        end: current,
                        direction: edge_direction,
                    };
                    edges.push(edge);
                    move_direction = Direction::Up;
                    edge_direction = Direction::Left;
                    edge_start = current;
                }
            }
            Direction::Up => {
                if top.is_some() {
                    previous = current;
                    current = top.unwrap();
                } else {
                    let edge = Edge {
                        start: edge_start,
                        end: current,
                        direction: edge_direction,
                    };
                    edges.push(edge);
                    move_direction = Direction::Right;
                    edge_direction = Direction::Up;
                    edge_start = current;
                }
            }
        }

        if current == start_node && edge_direction == start_edge_direction {
            // We have completed the loop
            break;
        }
    }

    edges
}

fn neighbours(row: usize, col: usize) -> Vec<(usize, usize)> {
    let mut neighbours = vec![(row + 1, col), (row, col + 1)];
    if col > 0 {
        neighbours.push((row, col - 1));
    }
    if row > 0 {
        neighbours.push((row - 1, col));
    }
    neighbours
}

fn left_neighbour(row: usize, col: usize, coords: &[(usize, usize)]) -> Option<(usize, usize)> {
    let neighbour = (row, col.wrapping_sub(1));
    if coords.contains(&neighbour) {
        Some(neighbour)
    } else {
        None
    }
}

fn right_neighbour(row: usize, col: usize, coords: &[(usize, usize)]) -> Option<(usize, usize)> {
    let neighbour = (row, col + 1);
    if coords.contains(&neighbour) {
        Some(neighbour)
    } else {
        None
    }
}

fn top_neighbour(row: usize, col: usize, coords: &[(usize, usize)]) -> Option<(usize, usize)> {
    let neighbour = (row.wrapping_sub(1), col);
    if coords.contains(&neighbour) {
        Some(neighbour)
    } else {
        None
    }
}

fn bottom_neighbour(row: usize, col: usize, coords: &[(usize, usize)]) -> Option<(usize, usize)> {
    let neighbour = (row + 1, col);
    if coords.contains(&neighbour) {
        Some(neighbour)
    } else {
        None
    }
}

/// Calculates the perimeter of a plot. This is the edges of the plot that are
/// not shared with another plot. A 2x2 plot has a perimeter of 8.
fn plot_perimeter(coords: &[(usize, usize)]) -> usize {
    let mut perimeter = 0;

    for (row, col) in coords {
        let neighbours = neighbours(*row, *col);
        let mut neighbour_count = 0;
        for (n_row, n_col) in neighbours {
            if coords.contains(&(n_row, n_col)) {
                neighbour_count += 1;
            }
        }
        let perimeter_diff = 4 - neighbour_count;
        perimeter += perimeter_diff;
    }

    perimeter
}

fn print_plot_shape(plot: &Plot) {
    let mut plot_map: HashMap<(usize, usize), char> = HashMap::new();
    for (row, col) in &plot.coords {
        plot_map.insert((*row, *col), plot.id);
    }

    let (min_row, max_row) = plot
        .coords
        .iter()
        .fold((usize::MAX, usize::MIN), |acc, (row, _)| {
            (acc.0.min(*row), acc.1.max(*row))
        });
    let (min_col, max_col) = plot
        .coords
        .iter()
        .fold((usize::MAX, usize::MIN), |acc, (_, col)| {
            (acc.0.min(*col), acc.1.max(*col))
        });

    for row in min_row..=max_row {
        for col in min_col..=max_col {
            let c = plot_map.get(&(row, col)).unwrap_or(&'.');
            print!("{}", c);
        }
        println!();
    }
}

fn main() {
    let puzzle_path = "input/input_example2.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let map = parse(&puzzle_input).unwrap();
    let plots = find_plots_dimensions(map);

    let mut price = 0;
    for (i, plot) in plots.iter().enumerate() {
        println!(
            "Plot {}: id = '{}', area = {}, perimeter = {}, coords = {:?}, edges = {:?}",
            i + 1,
            plot.id,
            plot.area,
            plot.perimeter,
            plot.coords,
            plot.edges
        );
        print_plot_shape(plot);
        println!();

        price += plot.area * plot.edges.len();
    }
    println!("Total price: {}", price);
}

#[cfg(test)]
mod tests {
    use super::*;

    static PUZZLE_INPUT: &str = r#"AAAA
BBCD
BBCC
EEEC"#;

    #[test]
    fn test_parse() {
        let result = parse(PUZZLE_INPUT).unwrap();
        let expected = Matrix::from_rows(vec![
            vec!['A', 'A', 'A', 'A'],
            vec!['B', 'B', 'C', 'D'],
            vec!['B', 'B', 'C', 'C'],
            vec!['E', 'E', 'E', 'C'],
        ])
        .unwrap();
        assert_eq!(result, expected);
    }
}
