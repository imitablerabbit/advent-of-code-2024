use std::{fs::File, io::Read};

type List = Vec<i32>;

type FrequencyMap = std::collections::HashMap<i32, i32>;

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let (mut list1, mut list2) = parse(puzzle_input).unwrap();

    list1.sort();
    list2.sort();
    let sum = sum_simularity_score(&list1, &list2);
    println!("The sum of the simularity scores is: {}", sum);
}

/// Reads the contents of the input file and returns a result of the file contents.
///
/// # Arguments
///
/// * `puzzle_path` - A string slice that holds the path to the input file.
///
/// # Returns
///
/// * `Result<String, std::io::Error>` - The contents of the file as a string or an error.
fn read_to_string(puzzle_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(puzzle_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

/// Parses the input file and returns a result of the two lists.
///
/// The puzzle input is structured as 2 columns of numbers separated by any
/// number of whitespace characters. The left column is the first list and the
/// right column is the second list.
///
/// # Arguments
///
/// * `puzzle_input` - A string containing the contents of the input file.
///
/// # Returns
///
/// * `Result<(List, List), std::io::Error>` - A tuple containing the two lists or an error.
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

/// Performs a binary search for the first instance of a number in a list.
///
/// # Arguments
///
/// * `list` - A reference to a list of integers.
/// * `num` - The number to search for.
///
/// # Returns
///
/// * `Option<usize>` - The position of the first instance of the number in the list or None if not found.
fn binary_search_first_instance(list: &List, num: i32) -> Option<usize> {
    match list.binary_search(&num) {
        Ok(pos) => {
            // Find the first instance by using rposition to find the starting index
            let first_pos = list[..pos]
                .iter()
                .rposition(|&x| x != num)
                .map_or(0, |p| p + 1);
            Some(first_pos)
        }
        Err(_) => None,
    }
}

/// Creates a frequency map of the two lists. We want to check how many times
/// a number from list1 appears in list2.
///
/// # Arguments
///
/// * `list1` - A reference to the first list of integers.
/// * `list2` - A reference to the second list of integers.
///
/// # Returns
///
/// * `FrequencyMap` - A map where the keys are the numbers from list1 and the values are their frequencies in list2.
fn create_frequency_map(list1: &List, list2: &List) -> FrequencyMap {
    // Filter out duplicates from list1.
    let list1 = list1.iter().collect::<std::collections::HashSet<_>>();
    let mut freq_map = FrequencyMap::new();

    // List2 is sorted so we can use binary search to find the number in list2
    // and then increment the frequency count for how many times it appears.
    for num in list1 {
        match binary_search_first_instance(list2, *num) {
            Some(start_pos) => {
                // binary_search returns the position of any of the matching
                // elements. We need to find the first position of the number
                // and then count how many times it appears.
                let count = list2[start_pos..]
                    .iter()
                    .take_while(|&&n| n == *num)
                    .count();
                freq_map.insert(*num, count as i32);
            }
            None => {
                // Insert the number with a frequency of 0 if not found
                freq_map.entry(*num).or_insert(0);
            }
        }
    }

    freq_map
}

/// Calculates the similarity score for a number based on its frequency in the frequency map.
///
/// # Arguments
///
/// * `num` - The number to calculate the similarity score for.
/// * `freq_map` - A reference to the frequency map.
///
/// # Returns
///
/// * `i32` - The similarity score for the number.
fn simularity_score(num: i32, freq_map: &FrequencyMap) -> i32 {
    num * freq_map.get(&num).unwrap()
}

/// Sums the similarity scores of the two lists.
///
/// # Arguments
///
/// * `list1` - A reference to the first list of integers.
/// * `list2` - A reference to the second list of integers.
///
/// # Returns
///
/// * `i32` - The sum of the similarity scores of the two lists.
fn sum_simularity_score(list1: &List, list2: &List) -> i32 {
    let freq_map = create_frequency_map(list1, list2);
    list1
        .iter()
        .map(|&num| simularity_score(num, &freq_map))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let puzzle_input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3\n".to_string();
        let (list1, list2) = parse(puzzle_input).unwrap();
        assert_eq!(list1, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(list2, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_create_frequency_map() {
        let mut list1 = vec![3, 4, 2, 1, 3, 3];
        let mut list2 = vec![4, 3, 5, 3, 9, 3];
        list1.sort();
        list2.sort();
        let expected_freq_map = {
            let mut map = FrequencyMap::new();
            map.insert(3, 3);
            map.insert(4, 1);
            map.insert(2, 0);
            map.insert(1, 0);
            map
        };
        let freq_map = create_frequency_map(&list1, &list2);
        assert_eq!(freq_map, expected_freq_map);
    }

    #[test]
    fn test_similarity_score() {
        let mut list1 = [3, 4, 2, 1, 3, 3];
        list1.sort();
        let freq_map = {
            let mut map = FrequencyMap::new();
            map.insert(3, 3);
            map.insert(4, 1);
            map.insert(2, 0);
            map.insert(1, 0);
            map
        };
        let score = simularity_score(3, &freq_map);
        assert_eq!(score, 9);
    }

    #[test]
    fn test_sum_similarity_score() {
        let mut list1 = vec![3, 4, 2, 1, 3, 3];
        let mut list2 = vec![4, 3, 5, 3, 9, 3];
        list1.sort();
        list2.sort();
        let sum_score = sum_simularity_score(&list1, &list2);
        assert_eq!(sum_score, 31);
    }
}
