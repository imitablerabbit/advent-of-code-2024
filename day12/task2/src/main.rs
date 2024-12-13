use pathfinding::matrix::{Matrix, MatrixFormatError};
use std::char;
use std::collections::{HashMap, HashSet};
use std::{fs::File, io::Read};

struct Plot {
    id: char,

    // Coordinates of the plot. These are (row, col) coordinates.
    coords: Vec<(usize, usize)>,

    area: usize,
    edges: usize,
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
        let area = coords.len();
        let edges = count_edges(&coords);
        let plot = Plot {
            id: *start_char,
            coords,
            area,
            edges,
        };
        plots.push(plot);
    }

    plots
}

fn normalise_coords(coords: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let min_row = coords.iter().map(|(row, _)| row).min().unwrap();
    let min_col = coords.iter().map(|(_, col)| col).min().unwrap();
    coords
        .iter()
        .map(|(row, col)| (row - min_row, col - min_col))
        .collect()
}

fn count_edges(coords: &[(usize, usize)]) -> usize {
    let normalised_coords = normalise_coords(coords);
    let max_row = normalised_coords.iter().map(|(row, _)| row).max().unwrap();
    let max_col = normalised_coords.iter().map(|(_, col)| col).max().unwrap();
    let mut matrix = vec![vec!['0'; max_col + 1]; max_row + 1];
    for (row, col) in normalised_coords {
        matrix[row][col] = '1';
    }

    let mut first_row: Vec<char> = matrix[0].clone();
    let mut last_row: Vec<char> = matrix.last().unwrap().clone();
    first_row.dedup();
    last_row.dedup();
    let first_row_count = first_row.into_iter().filter(|&c| c == '1').count();
    let last_row_count = last_row.into_iter().filter(|&c| c == '1').count();
    let mut row_count = first_row_count + last_row_count;
    for rows in matrix.windows(2) {
        let row1 = &rows[0];
        let row2 = &rows[1];
        let mut previous = ' ';
        let mut was_previous_edge = false;
        for (c1, c2) in row1.iter().zip(row2.iter()) {
            // This is very fragile and I hate it with a passion.
            if c1 == c2 {
                // We are in the middle of a shape or a blank space so no edge.
                previous = *c2;
                was_previous_edge = false;
                continue;
            }

            if !was_previous_edge || previous != *c2 {
                // If we havent already counted this edge or if the edge is not
                // the same as the previous edge, e.g. to islands meet at a corner.
                //
                // 111111
                // 100111
                // 111001
                // 111111
                //
                // We need to check both the previous char and the current char
                // to make sure we are not switching over from one edge to another
                // for the same row.
                row_count += 1;
                was_previous_edge = true;
            }
            previous = *c2;
        }
    }

    row_count * 2
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
            "Plot {}: id = '{}', area = {}, edges = {}, coords = {:?}",
            i + 1,
            plot.id,
            plot.area,
            plot.edges,
            plot.coords,
        );
        print_plot_shape(plot);
        price += plot.area * plot.edges;
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
