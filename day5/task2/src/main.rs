use std::{collections::HashMap, collections::HashSet, fs::File, io::Read};

/// Represents the rules for page ordering.
/// The key is a page number, and the value is a set of page numbers that must come after the key page.
type Rules = HashMap<usize, HashSet<usize>>;

/// Represents a collection of page orders. Each page order is a vector of page numbers.
type Pages = Vec<Vec<usize>>;

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

// The input is split into 2 sections delimited by 2 newlines.
//
// The first section contains the rules for the puzzle. These are pipe-delimited
// rules that specify the allowed ordering of the pages, e.g. "2|3" means that
// page 2 must come before page 3.
//
// The second section contains the pages of the puzzle. These are comma-delimited
// page numbers that specify the current page order, e.g. "1,2,3,4" means that
// there are 4 pages in the puzzle and page 1 comes before page 2.
fn parse_input(input: &str) -> (Rules, Pages) {
    let mut parts = input.split("\n\n");
    let rules = parts.next().unwrap();
    let pages = parts.next().unwrap();
    let rules = parse_rules(rules);
    let pages = parse_pages(pages);
    (rules, pages)
}

/// Parses the rules for page ordering from a string.
///
/// # Arguments
///
/// * `rules` - A string slice that holds the rules for page ordering.
///
/// # Returns
///
/// A `Rules` map where the key is a page number, and the value is a set of page numbers
/// that must come after the key page.
fn parse_rules(rules: &str) -> Rules {
    let mut rules_map = std::collections::HashMap::new();
    for rule in rules.lines() {
        let mut parts = rule.split('|');
        let key = parts.next().unwrap().parse().unwrap();
        let value = parts.next().unwrap().parse().unwrap();
        rules_map
            .entry(key)
            .or_insert_with(HashSet::new)
            .insert(value);
    }
    rules_map
}

/// Parses the pages delimited by newlines. Each line contains a comma-separated
/// list of page numbers.
///
/// # Arguments
///
/// * `pages` - A string slice that holds the pages.
///
/// # Returns
///
/// A `Pages` vector where each element is a vector of page numbers.
fn parse_pages(pages: &str) -> Pages {
    pages
        .lines()
        .map(|line| line.split(',').map(|n| n.parse().unwrap()).collect())
        .collect()
}

/// Filters out invalid page orders based on the provided rules.
///
/// # Arguments
///
/// * `rules` - A reference to the rules for page ordering.
/// * `pages` - A reference to the collection of page orders.
///
/// # Returns
///
/// A vector of valid page orders. Each page order is checked to ensure that the page numbers
/// are in the correct order according to the rules.
fn filter_valid_pages(rules: &Rules, pages: &Pages) -> Vec<Vec<usize>> {
    pages
        .iter()
        .filter(|page| !is_valid_page_order(rules, page))
        .cloned()
        .collect()
}

/// Checks if a given page order is valid according to the provided rules.
///
/// # Arguments
///
/// * `rules` - A reference to the rules for page ordering.
/// * `page_order` - A reference to a page order (a slice of page numbers).
///
/// # Returns
///
/// `true` if the page order is valid, `false` otherwise.
fn is_valid_page_order(rules: &Rules, page_order: &[usize]) -> bool {
    page_order.iter().enumerate().all(|(i, page)| {
        let before = &page_order[..i];
        before.iter().all(|&b| is_valid_order(b, *page, rules))
    })
}

/// Checks if the order of two pages is valid according to the provided rules.
///
/// # Arguments
///
/// * `before` - The page number that comes before.
/// * `after` - The page number that comes after.
/// * `rules` - A reference to the rules for page ordering.
///
/// # Returns
///
/// `true` if the order is valid, `false` otherwise.
fn is_valid_order(before: usize, after: usize, rules: &Rules) -> bool {
    match rules.get(&after) {
        Some(allowed_pages) => !allowed_pages.contains(&before),
        None => true,
    }
}

fn correct_page_order(rules: &Rules, page_order: &[usize]) -> Vec<usize> {
    let mut page_order = page_order.to_vec();
    page_order.sort_by(|&a, &b| {
        if is_valid_order(a, b, rules) {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    });
    page_order
}

/// Sums the middle pages of each page order in the collection.
///
/// # Arguments
///
/// * `pages` - A reference to the collection of page orders.
///
/// # Returns
///
/// The sum of the middle pages of each page order.
fn sum_middle_pages(pages: &Pages) -> usize {
    pages.iter().map(|page| middle_page(page)).sum()
}

/// Finds the middle page of a given page order.
///
/// # Arguments
///
/// * `page` - A reference to a page order (a slice of page numbers).
///
/// # Returns
///
/// The middle page number of the page order.
fn middle_page(page: &[usize]) -> usize {
    let page = page.to_vec();
    let index = page.len() / 2;
    page[index]
}

fn main() {
    let puzzle_path = "input/input.txt";
    let puzzle_input = read_to_string(puzzle_path).unwrap();
    let (rules, pages) = parse_input(&puzzle_input);
    let invalid_pages = filter_valid_pages(&rules, &pages);
    let corrected_pages = invalid_pages
        .iter()
        .map(|page| correct_page_order(&rules, page))
        .collect();
    let sum = sum_middle_pages(&corrected_pages);
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    const PUZZLE_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_parse() {
        let (rules, pages) = parse_input(PUZZLE_INPUT);
        assert_eq!(rules.len(), 6);
        assert_eq!(pages.len(), 6);
    }

    #[test]
    fn test_filter_invalid_pages() {
        let (rules, pages) = parse_input(PUZZLE_INPUT);
        let valid_pages = filter_valid_pages(&rules, &pages);
        println!("{:?}", valid_pages);
        assert_eq!(valid_pages.len(), 3);
    }

    #[test]
    fn test_sum_middle_pages() {
        let (rules, pages) = parse_input(PUZZLE_INPUT);
        let valid_pages = filter_valid_pages(&rules, &pages);
        let sum = sum_middle_pages(&valid_pages);
        assert_eq!(sum, 135); // This is the invalid pages now as its changed for task2
    }

    #[test]
    fn test_correct_page_order() {
        let (rules, pages) = parse_input(PUZZLE_INPUT);
        let invalid_pages = filter_valid_pages(&rules, &pages);
        let corrected_pages: Pages = invalid_pages
            .iter()
            .map(|page| correct_page_order(&rules, page))
            .collect();
        let sum = sum_middle_pages(&corrected_pages);
        assert_eq!(sum, 123);
    }
}
