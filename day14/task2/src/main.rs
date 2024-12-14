use colored::Colorize;
use itertools::Itertools;
use pathfinding::prelude::Matrix;
use rayon::prelude::*;
use std::{fs::File, io::Read};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    fn simulate(&self, steps: i32, x_bound: i32, y_bound: i32) -> Robot {
        let mut x = self.x + self.dx * steps;
        let mut y = self.y + self.dy * steps;

        // Implement wrapping around the bounds
        x = ((x % x_bound) + x_bound) % x_bound;
        y = ((y % y_bound) + y_bound) % y_bound;

        Robot {
            x,
            y,
            dx: self.dx,
            dy: self.dy,
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

fn parse(input: &str) -> Vec<Robot> {
    input
        .split(|c: char| !c.is_ascii_digit() && c != '-')
        .filter_map(|s| s.parse::<i32>().ok())
        .tuples()
        .map(|(x, y, dx, dy)| Robot { x, y, dx, dy })
        .collect()
}

// Split the robots based on which of the 4 quadrants they are in.
fn split_into_quadrants(robots: Vec<Robot>, x_bound: i32, y_bound: i32) -> Vec<Vec<Robot>> {
    let mut quadrants = vec![vec![]; 4];
    for robot in robots {
        if robot.x < x_bound / 2 && robot.y < y_bound / 2 {
            quadrants[0].push(robot);
        } else if robot.x >= x_bound / 2 && robot.y < y_bound / 2 {
            quadrants[1].push(robot);
        } else if robot.x < x_bound / 2 && robot.y >= y_bound / 2 {
            quadrants[2].push(robot);
        } else {
            quadrants[3].push(robot);
        }
    }
    quadrants
}

// Robots that are perfectly on a quadrant boundary are filtered out.
fn filter_middle_robots(robots: Vec<Robot>, x_bound: i32, y_bound: i32) -> Vec<Robot> {
    robots
        .into_iter()
        .filter(|robot| robot.x != x_bound / 2 && robot.y != y_bound / 2)
        .collect()
}

fn safety_factor(quadrants: &[Vec<Robot>]) -> i32 {
    let safety_factor = quadrants
        .iter()
        .fold(1, |acc, quadrant| quadrant.len() * acc);
    safety_factor as i32
}

fn print_robot_map(robots: &[Robot], x_bound: i32, y_bound: i32) {
    let matrix = Matrix::from_fn(y_bound as usize, x_bound as usize, |(y, x)| {
        robots
            .iter()
            .filter(|robot| robot.x == x as i32 && robot.y == y as i32)
            .count()
    });

    for y in 0..y_bound {
        for x in 0..x_bound {
            let c = match matrix[(y as usize, x as usize)] {
                0 => " ".to_string(),
                _ => "█".to_string(),
            };

            print!("{:1}", c.red());
        }
        println!();
    }
}

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let robots = parse(&puzzle_input);
    let x_bound = 101;
    let y_bound = 103;

    // Use the safety factor to find the frame where the robots are the least
    // spread out.
    let tree_frame = (1..=(101 * 103))
        .into_par_iter()
        .map(|i| {
            let positions: Vec<_> = robots
                .iter()
                .map(|robot| robot.simulate(i, x_bound, y_bound))
                .collect();

            let filtered_positions = filter_middle_robots(positions, x_bound, y_bound);
            let quadrants = split_into_quadrants(filtered_positions, x_bound, y_bound);
            (i, safety_factor(&quadrants))
        })
        .min_by_key(|(_, safety_factor)| *safety_factor)
        .unwrap()
        .0;

    let positions: Vec<_> = robots
        .iter()
        .map(|robot| robot.simulate(tree_frame, x_bound, y_bound))
        .collect();

    print_robot_map(&positions, x_bound, y_bound);
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT: &str = r#"p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3"#;

    #[test]
    fn test_parse() {
        let input = "1 2 3 4\n5 6 7 8";
        let expected = vec![
            Robot {
                x: 1,
                y: 2,
                dx: 3,
                dy: 4,
            },
            Robot {
                x: 5,
                y: 6,
                dx: 7,
                dy: 8,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_parse2() {
        let input = PUZZLE_INPUT;
        let expected = [
            Robot {
                x: 0,
                y: 4,
                dx: 3,
                dy: -3,
            },
            Robot {
                x: 6,
                y: 3,
                dx: -1,
                dy: -3,
            },
            Robot {
                x: 10,
                y: 3,
                dx: -1,
                dy: 2,
            },
            Robot {
                x: 2,
                y: 0,
                dx: 2,
                dy: -1,
            },
            Robot {
                x: 0,
                y: 0,
                dx: 1,
                dy: 3,
            },
            Robot {
                x: 3,
                y: 0,
                dx: -2,
                dy: -2,
            },
            Robot {
                x: 7,
                y: 6,
                dx: -1,
                dy: -3,
            },
            Robot {
                x: 3,
                y: 0,
                dx: -1,
                dy: -2,
            },
            Robot {
                x: 9,
                y: 3,
                dx: 2,
                dy: 3,
            },
            Robot {
                x: 7,
                y: 3,
                dx: -1,
                dy: 2,
            },
            Robot {
                x: 2,
                y: 4,
                dx: 2,
                dy: -3,
            },
            Robot {
                x: 9,
                y: 5,
                dx: -3,
                dy: -3,
            },
        ];
        assert_eq!(parse(input), expected);
    }

    #[test]
    fn test_simulate() {
        let robot = Robot {
            x: 0,
            y: 0,
            dx: 1,
            dy: 1,
        };
        assert_eq!(
            robot.simulate(1, 10, 10),
            Robot {
                x: 1,
                y: 1,
                dx: 1,
                dy: 1
            }
        );
        assert_eq!(
            robot.simulate(2, 10, 10),
            Robot {
                x: 2,
                y: 2,
                dx: 1,
                dy: 1
            }
        );
        assert_eq!(
            robot.simulate(3, 10, 10),
            Robot {
                x: 3,
                y: 3,
                dx: 1,
                dy: 1
            }
        );
        assert_eq!(
            robot.simulate(4, 10, 10),
            Robot {
                x: 4,
                y: 4,
                dx: 1,
                dy: 1
            }
        );
        assert_eq!(
            robot.simulate(5, 10, 10),
            Robot {
                x: 5,
                y: 5,
                dx: 1,
                dy: 1
            }
        );
        assert_eq!(
            robot.simulate(6, 10, 10),
            Robot {
                x: 6,
                y: 6,
                dx: 1,
                dy: 1
            }
        );
        assert_eq!(
            robot.simulate(7, 10, 10),
            Robot {
                x: 7,
                y: 7,
                dx: 1,
                dy: 1
            }
        );
        assert_eq!(
            robot.simulate(8, 10, 10),
            Robot {
                x: 8,
                y: 8,
                dx: 1,
                dy: 1
            }
        );
        assert_eq!(
            robot.simulate(9, 10, 10),
            Robot {
                x: 9,
                y: 9,
                dx: 1,
                dy: 1
            }
        );
        assert_eq!(
            robot.simulate(10, 10, 10),
            Robot {
                x: 0,
                y: 0,
                dx: 1,
                dy: 1
            }
        );
    }
}
