use colored::*;
use itertools::Itertools;
use pathfinding::prelude::Matrix;
use std::{fs::File, io::Read};

#[derive(Debug, PartialEq, Clone, Copy)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    fn step(&mut self, x_bound: i32, y_bound: i32) {
        self.x += self.dx;
        self.y += self.dy;

        // Implement wrapping around the bounds
        if self.x < 0 {
            self.x += x_bound;
        } else if self.x >= x_bound {
            self.x -= x_bound;
        }

        if self.y < 0 {
            self.y += y_bound;
        } else if self.y >= y_bound {
            self.y -= y_bound;
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

fn print_robot_map(robots: &[Robot], x_bound: i32, y_bound: i32) {
    let filtered_robots = filter_middle_robots(robots.to_vec(), x_bound, y_bound);

    let matrix = Matrix::from_fn(y_bound as usize, x_bound as usize, |(y, x)| {
        filtered_robots
            .iter()
            .filter(|robot| robot.x == x as i32 && robot.y == y as i32)
            .count()
    });

    // Print the x-axis labels
    print!("   ");
    for x in 0..x_bound {
        print!("{:2} ", x);
    }
    println!();

    for y in 0..y_bound {
        // Print the y-axis label
        print!("{:2}  ", y);

        // Skip the middle line
        if y == y_bound / 2 {
            println!();
            continue;
        }

        for x in 0..x_bound {
            // Skip the middle column
            if x == x_bound / 2 {
                print!("   ");
                continue;
            }

            let c = match matrix[(y as usize, x as usize)] {
                0 => ".".to_string(),
                n => n.to_string(),
            };

            let colored_c = if x < x_bound / 2 && y < y_bound / 2 {
                c.green()
            } else if x >= x_bound / 2 && y < y_bound / 2 {
                c.red()
            } else if x < x_bound / 2 && y >= y_bound / 2 {
                c.blue()
            } else {
                c.yellow()
            };

            print!("{:2} ", colored_c);
        }
        println!();
    }
}

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let mut robots = parse(&puzzle_input);
    let x_bound = 101;
    let y_bound = 103;
    for _ in 0..100 {
        robots
            .iter_mut()
            .for_each(|robot| robot.step(x_bound, y_bound));
    }
    print_robot_map(&robots, x_bound, y_bound);
    println!();
    let filtered_robots = filter_middle_robots(robots, x_bound, y_bound);
    let quadrants = split_into_quadrants(filtered_robots, x_bound, y_bound);
    let safety_factor = quadrants
        .iter()
        .fold(1, |acc, quadrant| quadrant.len() * acc);
    println!("Safety factor: {}", safety_factor);
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
    fn test_step() {
        let mut robot = Robot {
            x: 0,
            y: 0,
            dx: 1,
            dy: 1,
        };
        robot.step(10, 10);
        assert_eq!(
            robot,
            Robot {
                x: 1,
                y: 1,
                dx: 1,
                dy: 1
            }
        );
    }
}
