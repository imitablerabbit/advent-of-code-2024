use std::{fs::File, io::Read};

#[derive(Debug, Clone, PartialEq)]
struct Delta {
    x: i64,
    y: i64,
}

impl Delta {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Location {
    x: i64,
    y: i64,
}

impl Location {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PrizeMachine {
    button_a: Delta,
    button_b: Delta,
    location: Location,
}

impl PrizeMachine {
    fn new(button_a: Delta, button_b: Delta, location: Location) -> Self {
        Self {
            button_a,
            button_b,
            location,
        }
    }

    // We have simultaneous equations for the button presses:
    //
    // x = ni + mj
    // y = nk + ml
    //
    // we want to solve this for n and m as x, y, i, j, k, and l are known.
    //
    // x is the prize location x
    // y is the prize location y
    // i is the button A x
    // j is the button B x
    // k is the button A y
    // l is the button B y
    //
    fn min_tokens(&self) -> Option<i64> {
        let n = self.n();
        let m = self.m();
        let xy = (self.calculate_x(n, m), self.calculate_y(n, m));
        match xy {
            (lx, ly) if lx == self.location.x && ly == self.location.y => Some((n * 3) + m),
            _ => None,
        }
    }

    // Rearrange the equations to have m on one side:
    //
    // m = (x - ni) / j
    // m = (y - nk) / l
    //
    // (x - ni) / j = (y - nk) / l
    // l(x - ni) = j(y - nk)
    // lx - lni = jy - jnk
    // lx - jy = lni - jnk
    // lx - jy = n(li - jk)
    // n = (lx - jy) / (li - jk)
    //
    fn n(&self) -> i64 {
        let i = self.button_a.x;
        let j = self.button_b.x;
        let k = self.button_a.y;
        let l = self.button_b.y;
        let x = self.location.x;
        let y = self.location.y;
        (l * x - j * y) / (l * i - j * k)
    }

    // m = (x - ni) / j
    //
    // Just use the n value to calculate m.
    fn m(&self) -> i64 {
        let i = self.button_a.x;
        let j = self.button_b.x;
        let x = self.location.x;
        let n = self.n();
        (x - n * i) / j
    }

    fn calculate_x(&self, n: i64, m: i64) -> i64 {
        let i = self.button_a.x;
        let j = self.button_b.x;
        n * i + m * j
    }

    fn calculate_y(&self, n: i64, m: i64) -> i64 {
        let k = self.button_a.y;
        let l = self.button_b.y;
        n * k + m * l
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

/// Parses the input string in the format of:
///
/// Button A: X+[0-9]+, Y+[0-9]+
/// Button B: X+[0-9]+, Y+[0-9]+
/// Prize: X+[0-9]+, Y+[0-9]+
///
/// Each prize machine is defined as 3 lines where the first line is the button A
/// coordinates, the second line is the button B coordinates, and the third line
/// is the prize coordinates.
///
/// The prize machines are separated by a blank line.
fn parse_input(input: &str) -> Result<Vec<PrizeMachine>, std::num::ParseIntError> {
    input
        .split("\n\n")
        .map(|machine| {
            let mut lines = machine.lines();
            let button_a = lines.next().unwrap();
            let button_b = lines.next().unwrap();
            let prize = lines.next().unwrap();
            let button_a = parse_button(button_a);
            let button_b = parse_button(button_b);
            let prize = parse_location(prize);
            Ok(PrizeMachine::new(button_a?, button_b?, prize?))
        })
        .collect()
}

// Parse a button string in the format of:
//
// Button A: X+94, Y+34
//
fn parse_button(input: &str) -> Result<Delta, std::num::ParseIntError> {
    let xy = parse_x_y(input)?;
    Ok(Delta::new(xy.0, xy.1))
}

// Parse a location string in the format of:
//
// Prize: X=8400, Y=5400
//
fn parse_location(input: &str) -> Result<Location, std::num::ParseIntError> {
    let xy = parse_x_y(input)?;
    let increment: i64 = 10000000000000;
    Ok(Location::new(xy.0 + increment, xy.1 + increment))
}

fn parse_x_y(input: &str) -> Result<(i64, i64), std::num::ParseIntError> {
    let input = input.replace(|c: char| !c.is_ascii_digit() && c != ',', "");
    let mut parts = input.split(',');
    let x = parts.next().unwrap().parse()?;
    let y = parts.next().unwrap().parse()?;
    Ok((x, y))
}

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let prize_machines = parse_input(&puzzle_input).unwrap();
    let cost: i64 = prize_machines
        .into_iter()
        .map(|machine| machine.min_tokens().unwrap_or(0))
        .sum();
    println!("The total cost is: {}", cost);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_button() {
        let input = "Button A: X+94, Y+34";
        let expected = Delta::new(94, 34);
        let result = parse_button(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_location() {
        let input = "Prize: X=94, Y=34";
        let expected = Location::new(94 + 10000000000000, 34 + 10000000000000);
        let result = parse_location(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_input() {
        let input = "Button A: X+94, Y+34\nButton B: X+94, Y+34\nPrize: X=94, Y=34\n\nButton A: X+94, Y+34\nButton B: X+94, Y+34\nPrize: X=94, Y=34";
        let expected = vec![
            PrizeMachine::new(
                Delta::new(94, 34),
                Delta::new(94, 34),
                Location::new(94 + 10000000000000, 34 + 10000000000000),
            ),
            PrizeMachine::new(
                Delta::new(94, 34),
                Delta::new(94, 34),
                Location::new(94 + 10000000000000, 34 + 10000000000000),
            ),
        ];
        let result = parse_input(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_min_tokens() {
        let machine = PrizeMachine::new(
            Delta::new(94, 34),
            Delta::new(22, 67),
            Location::new(8400, 5400),
        );
        let expected = 280;
        let result = machine.min_tokens().unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_min_tokens2() {
        let machine = PrizeMachine::new(
            Delta::new(26, 66),
            Delta::new(67, 21),
            Location::new(12748, 12176),
        );
        let result = machine.min_tokens();
        assert_eq!(result, None);
    }
}
