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

fn create_robot_map_image(robots: &[Robot], x_bound: i32, y_bound: i32, name: &str) {
    let matrix = Matrix::from_fn(y_bound as usize, x_bound as usize, |(y, x)| {
        robots
            .iter()
            .filter(|robot| robot.x == x as i32 && robot.y == y as i32)
            .count()
    });

    let mut imgbuf = image::ImageBuffer::new(x_bound as u32, y_bound as u32);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let c = match matrix[(y as usize, x as usize)] {
            0 => image::Rgb([255u8, 255u8, 255u8]),
            _ => image::Rgb([255u8, 0u8, 0u8]),
        };

        *pixel = c;
    }

    imgbuf.save(name).unwrap();
}

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let mut robots = parse(&puzzle_input);
    let x_bound = 101;
    let y_bound = 103;
    for i in 0..=8000 {
        // Yep. Generate all the frames and go through them manually. There is
        // a pattern that emerges and you can see them aligning horizontally and
        // vertically. We need the frame where they both align based on the width
        // and height of the frame.
        //
        // Im too dumb to figure out the math to calculate the frame where they align
        // so I just went through them manually.

        robots
            .iter_mut()
            .for_each(|robot| robot.step(x_bound, y_bound));

        let name = format!("output/{}.png", i);
        create_robot_map_image(&robots, x_bound, y_bound, &name);
    }
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
