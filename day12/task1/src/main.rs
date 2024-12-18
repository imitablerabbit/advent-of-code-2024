use pathfinding::matrix::MatrixFormatError;
use pathfinding::prelude::Matrix;
use std::char;
use std::collections::{HashMap, HashSet};
use std::{fs::File, io::Read};

struct Plot {
    id: char,

    // Coordinates of the plot. These are (row, col) coordinates.
    coords: Vec<(usize, usize)>,

    area: usize,
    perimeter: usize,
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
        let area = plot_area(&coords);
        let perimeter = plot_perimeter(&coords);
        let plot = Plot {
            id: *start_char,
            coords,
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

/// Calculates the perimeter of a plot. This is the edges of the plot that are
/// not shared with another plot. A 2x2 plot has a perimeter of 8.
fn plot_perimeter(coords: &[(usize, usize)]) -> usize {
    let mut perimeter = 0;

    for (row, col) in coords {
        let mut neighbours = vec![];
        neighbours.push((row + 1, *col));
        neighbours.push((*row, col + 1));
        if *row > 0 {
            neighbours.push((row - 1, *col));
        }
        if *col > 0 {
            neighbours.push((*row, col - 1));
        }

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
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let map = parse(&puzzle_input).unwrap();
    let plots = find_plots_dimensions(map);

    let mut price = 0;
    for (i, plot) in plots.iter().enumerate() {
        println!(
            "Plot {}: id = '{}', area = {}, perimeter = {}, coords = {:?}",
            i + 1,
            plot.id,
            plot.area,
            plot.perimeter,
            plot.coords,
        );
        print_plot_shape(plot);
        println!();

        price += plot.area * plot.perimeter;
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
