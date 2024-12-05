use aoc_runner_derive::aoc;
use regex::Regex;
use std::path::Path;

/// Part 1: Computes the total of valid `mul(X, Y)` operations found in the input.
///
/// # Example
/// ```
/// use advent_of_code::year2024::day3::part1;
///
/// let input = "test_input1.txt";
/// std::fs::write(input, "mul(2,3)\nmul(4,5)\nmul(a,b)\n").unwrap();
/// assert_eq!(part1(input), 26); // 2*3 + 4*5 = 26
/// std::fs::remove_file(input).unwrap();
/// ```
#[aoc(day3, part1)]
#[must_use]
pub fn part1(file_path: &str) -> i32 {
    let content = read_file_to_string(file_path);
    compute_part1(&content)
}

/// Part 2: Computes the total of valid `mul(X, Y)` operations, applying the `do()` and `don't()` rules.
///
/// # Example
/// ```
/// use advent_of_code::year2024::day3::part2;
///
/// let input = "test_input2.txt";
/// std::fs::write(input, "mul(2,3)\ndo()\nmul(4,5)\ndon't()\nmul(6,7)\ndo()\nmul(8,9)\n").unwrap();
/// assert_eq!(part2(input), 98); // 2*3 + 4*5 + 8*9 = 86
/// std::fs::remove_file(input).unwrap();
/// ```
#[aoc(day3, part2)]
#[must_use]
pub fn part2(file_path: &str) -> i32 {
    let content = read_file_to_string(file_path);
    compute_part2(&content)
}

/// Helper function for Part 1: Computes the total from valid `mul(X, Y)` operations in the input.
///
/// # Arguments
/// * `content` - The input string.
///
/// # Returns
/// The total sum of valid operations.
fn compute_part1(content: &str) -> i32 {
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut result = 0;

    for cap in mul_pattern.captures_iter(content) {
        let x: i32 = cap[1].parse().unwrap();
        let y: i32 = cap[2].parse().unwrap();
        result += x * y;
    }
    result
}

/// Helper function for Part 2: Computes the total with `do()` and `don't()` rules applied.
///
/// # Arguments
/// * `content` - The input string.
///
/// # Returns
/// The total sum after applying the rules.
fn compute_part2(content: &str) -> i32 {
    let instruction_pattern = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();
    let mul_pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut enabled = true;
    let mut result = 0;

    for mat in instruction_pattern.find_iter(content) {
        let token = mat.as_str();

        if token.starts_with("mul") {
            if let Some(caps) = mul_pattern.captures(token) {
                let x: i32 = caps[1].parse().unwrap();
                let y: i32 = caps[2].parse().unwrap();
                if enabled {
                    result += x * y;
                }
            }
        } else if token == "do()" {
            enabled = true;
        } else if token == "don't()" {
            enabled = false;
        }
    }
    result
}

/// Reads the content of a file into a string.
///
/// # Arguments
/// * `file_path` - The path to the input file.
///
/// # Returns
/// The content of the file as a `String`.
///
/// # Panics
/// Panics if the file cannot be read.
#[inline(always)]
fn read_file_to_string<P: AsRef<Path>>(file_path: P) -> String {
    std::fs::read_to_string(file_path).expect("Failed to read the file")
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::{
        fs::{self, File},
        io::Write,
    };

    #[test]
    fn test_part1() {
        let input =
            indoc! {"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"};

        let file_path = "test_input1.txt";
        create_test_file(file_path, input);

        // Test part1
        assert_eq!(part1(file_path), 161);

        // Clean up
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_part2() {
        let input =
            indoc! {"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"};

        let file_path = "test_input2.txt";
        create_test_file(file_path, input);

        // Test part2
        assert_eq!(part2(file_path), 48); // Process based on do/don't

        // Clean up
        fs::remove_file(file_path).unwrap();
    }

    fn create_test_file(file_path: &str, content: &str) {
        let path = Path::new(file_path);
        let mut file = File::create(path).expect("Unable to create test file");
        file.write_all(content.as_bytes())
            .expect("Unable to write to test file");
    }
}
