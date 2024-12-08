use cached::proc_macro::cached;
use itertools::Itertools;
use rayon::prelude::*;
use std::{fmt, fs::File, io::Read};

#[derive(Debug, PartialEq)]
enum CalculationError {
    NoValidOperators,
}

#[derive(Debug, Clone, PartialEq)]
struct Equation {
    value: u64,
    operands: Vec<u64>,
}

#[derive(Debug, Clone, PartialEq)]
struct Calculation {
    has_calculated: bool,
    calculated_value: u64,
    equation: Equation,
    operators: Vec<Operator>,
}

#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Add,
    Multiply,
    Concatinate,
}

impl Equation {
    fn new(value: u64, operands: Vec<u64>) -> Self {
        Self { value, operands }
    }

    fn determine_calculation(&self) -> Result<Calculation, CalculationError> {
        let permutations = self.calculation_permutations();
        permutations
            .into_par_iter()
            .find_map_first(|mut calc| {
                calc.calculate();
                if calc.is_valid() {
                    Some(calc)
                } else {
                    None
                }
            })
            .ok_or(CalculationError::NoValidOperators)
    }

    /// Calculates the cartesian product of the available operators N times
    /// where N is the number of operands - 1. This will give us all the
    /// combinations of operators that can be used to calculate the equation.
    ///
    /// The cartesian product maps one set with every element in another set.
    /// In this case we are actually mapping the same set to itself N times.
    ///
    /// # Returns
    ///
    /// * `Vec<Calculation>` - A vector of calculations that can be used to
    /// calculate the equation.
    ///
    fn calculation_permutations(&self) -> Vec<Calculation> {
        let size = self.operands.len() - 1;
        operator_cartesian_product(size)
            .into_par_iter()
            .map(|operators| Calculation::new(self.clone(), operators))
            .collect()
    }
}

#[cached]
fn operator_cartesian_product(size: usize) -> Vec<Vec<Operator>> {
    let operators = vec![Operator::Add, Operator::Multiply, Operator::Concatinate];
    let repeated_iter = std::iter::repeat(operators).take(size);
    repeated_iter.multi_cartesian_product().collect()
}

impl Calculation {
    fn new(equation: Equation, operators: Vec<Operator>) -> Self {
        Self {
            has_calculated: false,
            equation,
            calculated_value: 0,
            operators,
        }
    }

    fn is_valid(&self) -> bool {
        self.calculated_value == self.equation.value
    }

    fn calculate(&mut self) {
        let init = self.equation.operands[0];
        let calculated = self.equation.operands.iter().skip(1).enumerate().fold(
            init,
            |acc, (index, operand)| match self.operators[index] {
                Operator::Add => acc + operand,
                Operator::Multiply => acc * operand,
                Operator::Concatinate => {
                    // Rather than concatenating both as strings and then parsing
                    // the string back to a number we can just multiply the left
                    // operand by 10 to the power of the right operand's length.
                    acc * 10_u64.pow(operand.to_string().len() as u32) + operand
                }
            },
        );
        self.calculated_value = calculated;
        self.has_calculated = true;
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Multiply => write!(f, "*"),
            Operator::Concatinate => write!(f, "||"),
        }
    }
}

impl fmt::Display for Calculation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string_operators = self
            .operators
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let equation_string = self
            .equation
            .operands
            .iter()
            .cloned()
            .map(|x| x.to_string())
            .interleave(string_operators)
            .join(" ");
        let equality = if self.calculated_value == self.equation.value {
            "="
        } else {
            "≠"
        };
        write!(
            f,
            "{} = {} {} {}",
            equation_string, self.calculated_value, equality, self.equation.value
        )
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

fn parse_input(input: &str) -> Result<Vec<Equation>, std::num::ParseIntError> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let value = parts.next().unwrap().parse()?;
            let operands = parts
                .next()
                .unwrap()
                .split_whitespace()
                .map(|operand| operand.parse())
                .collect::<Result<Vec<u64>, _>>()?;
            Ok(Equation::new(value, operands))
        })
        .collect()
}

fn filter_invalid_equations(equations: Vec<Equation>) -> Vec<Equation> {
    equations
        .into_par_iter()
        .filter(|equation| equation.determine_calculation().is_ok())
        .collect()
}

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let equations = parse_input(&puzzle_input).unwrap();
    let valid_equations = filter_invalid_equations(equations);
    let sum: u64 = valid_equations.into_par_iter().map(|eq| eq.value).sum();
    println!("Sum of valid equations: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test_parse() {
        let equations = parse_input(PUZZLE_INPUT).expect("Failed to parse input");
        assert_eq!(equations.len(), 9);
        assert_eq!(equations[0].value, 190);
        assert_eq!(equations[0].operands, vec![10, 19]);
    }

    #[test]
    fn test_calculate() {
        let operands = vec![10, 19];
        let operators = vec![Operator::Add];
        let equation = Equation::new(190, operands);
        let mut calculation = Calculation::new(equation, operators);
        calculation.calculate();
        assert_eq!(calculation.calculated_value, 29);
    }

    #[test]
    fn test_calculate2() {
        let operands = vec![10, 19];
        let operators = vec![Operator::Multiply];
        let equation = Equation::new(190, operands);
        let mut calculation = Calculation::new(equation, operators);
        calculation.calculate();
        assert_eq!(calculation.calculated_value, 190);
    }

    #[test]
    fn test_calculate3() {
        let eq = Equation::new(292, vec![11, 6, 16, 20]);
        let calculation = eq.determine_calculation();
        assert_eq!(
            calculation,
            Ok(Calculation {
                has_calculated: true,
                calculated_value: 292,
                equation: Equation::new(292, vec![11, 6, 16, 20]),
                operators: vec![Operator::Add, Operator::Multiply, Operator::Add],
            })
        );
    }

    #[test]
    fn test_determine_operator() {
        let equations = parse_input(PUZZLE_INPUT).expect("Failed to parse input");
        let calculation = equations[0].determine_calculation();
        let expected = Calculation {
            has_calculated: true,
            calculated_value: 190,
            equation: Equation::new(190, vec![10, 19]),
            operators: vec![Operator::Multiply],
        };
        assert_eq!(calculation, Ok(expected),);
    }

    #[test]
    fn test_filter_invalid_equations() {
        let equations = parse_input(PUZZLE_INPUT).expect("Failed to parse input");
        let valid_equations = filter_invalid_equations(equations);
        assert_eq!(valid_equations.len(), 6);
    }

    #[test]
    fn sum_of_valid_equations() {
        let equations = parse_input(PUZZLE_INPUT).expect("Failed to parse input");
        let valid_equations = filter_invalid_equations(equations);
        let sum: u64 = valid_equations.iter().map(|eq| eq.value).sum();
        assert_eq!(sum, 11387);
    }
}
