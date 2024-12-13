use itertools::Itertools;
use rayon::prelude::*;
use std::{fs::File, io::Read};

#[derive(Debug, PartialEq)]
enum PrizeError {
    NoValidCombinations,
}

#[derive(Debug, Clone, PartialEq)]
struct Delta {
    x: i32,
    y: i32,
}

impl Delta {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PrizeMachine {
    button_a: Delta,
    button_b: Delta,
    location: Location,
}

struct Presses {
    a: i32,
    b: i32,
}

impl PrizeMachine {
    fn new(button_a: Delta, button_b: Delta, location: Location) -> Self {
        Self {
            button_a,
            button_b,
            location,
        }
    }

    fn find_min(&self, perms: &Vec<(usize, usize)>) -> Result<i32, PrizeError> {
        let possible_presses = perms.into_par_iter().filter_map(|(a, b)| {
            let location = Location::new(
                self.button_a.x * (*a as i32) + self.button_b.x * (*b as i32),
                self.button_a.y * (*a as i32) + self.button_b.y * (*b as i32),
            );
            if location == self.location {
                return Some(Presses {
                    a: *a as i32,
                    b: *b as i32,
                });
            }
            None
        });
        possible_presses
            .into_par_iter()
            .map(|presses| (presses.a * 3) + presses.b)
            .min()
            .ok_or(PrizeError::NoValidCombinations)
    }
}

fn button_permutations(size: usize) -> Vec<(usize, usize)> {
    (0..=size)
        .permutations(2)
        .map(|perm| (perm[0], perm[1]))
        .collect()
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

// Parse a button string in the format of:
//
// Button A: X+94, Y+34
//
fn parse_location(input: &str) -> Result<Location, std::num::ParseIntError> {
    let xy = parse_x_y(input)?;
    Ok(Location::new(xy.0, xy.1))
}

fn parse_x_y(input: &str) -> Result<(i32, i32), std::num::ParseIntError> {
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
    let last = prize_machines.last().unwrap();
    println!("Last: {:?}", last);
    let perms = button_permutations(100);
    let total_min = prize_machines.iter().fold(0, |mut acc, machine| {
        if let Ok(min) = machine.find_min(&perms) {
            acc += min
        }
        acc
    });
    println!("Total min: {}", total_min);
}
