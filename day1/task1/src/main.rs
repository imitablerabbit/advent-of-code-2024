use std::{fs::File, io::Read};

type List = Vec<i32>;

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let (mut list1, mut list2) = parse(puzzle_input).unwrap();

    list1.sort();
    list2.sort();
    let sum = sum_differences(&list1, &list2);
    println!("The sum of the differences is: {}", sum);
}

// Read the contents of the input file and return a result of the file contents
fn read_to_string(puzzle_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(puzzle_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

// Parse the input file and return a result of the two lists.
//
// The puzzle input is structured as 2 columns of numbers separated by any
// number of whitespace characters. The left column is the first list and the
// right column is the second list.
fn parse(puzzle_input: String) -> Result<(List, List), std::io::Error> {
    let mut list1 = List::new();
    let mut list2 = List::new();

    for line in puzzle_input.lines() {
        let mut nums = line.split_whitespace().map(|n| n.parse().unwrap());
        list1.push(nums.next().unwrap());
        list2.push(nums.next().unwrap());
    }

    Ok((list1, list2))
}

// Calculate the sum of the absolute differences between the elements of the two
// lists. The lists are assumed to be the same length and sorted in ascending
// order.
fn sum_differences(list1: &List, list2: &List) -> i32 {
    list1
        .iter()
        .zip(list2)
        .map(|(a, b)| a - b)
        .map(i32::abs)
        .sum::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_differences() {
        let mut list1 = vec![3, 4, 2, 1, 3, 3];
        let mut list2 = vec![4, 3, 5, 3, 9, 3];
        list1.sort();
        list2.sort();
        assert_eq!(sum_differences(&list1, &list2), 11);
    }

    #[test]
    fn test_parse() {
        let puzzle_input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3\n".to_string();
        let (list1, list2) = parse(puzzle_input).unwrap();
        assert_eq!(list1, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(list2, vec![4, 3, 5, 3, 9, 3]);
    }
}