use aoc_runner_derive::aoc;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

/// Part 1: Counts the number of safe reports for part 1 rules.
///
/// # Example
/// ```
/// use advent_of_code::year2024::day2::part1;
/// let input = "test_input.txt";
/// std::fs::write(input, "7 6 4 2 1\n1 2 7 8 9\n").unwrap();
/// assert_eq!(part1(input), 1); // Only the second report is safe.
/// std::fs::remove_file(input).unwrap();
/// ```
#[aoc(day2, part1)]
#[must_use]
pub fn part1(file_path: &str) -> usize {
    let reports = read_input(file_path);
    reports.iter().filter(|report| is_safe_p1(report)).count()
}

/// Part 2: Counts the number of safe reports for part 2 rules.
///
/// # Example
/// ```
/// use advent_of_code::year2024::day2::part2;
/// let input = "test_input.txt";
/// std::fs::write(input, "7 6 4 2 1\n1 3 2 4 5\n").unwrap();
/// assert_eq!(part2(input), 2); // Both reports are safe under part 2 rules.
/// std::fs::remove_file(input).unwrap();
/// ```
#[aoc(day2, part2)]
#[must_use]
pub fn part2(file_path: &str) -> usize {
    let reports = read_input(file_path);
    reports.iter().filter(|report| is_safe_p2(report)).count()
}

/// Reads input from a file and parses it into a vector of vectors of integers.
///
/// # Example
/// ```
/// use advent_of_code::year2024::day2::read_input;
/// let input = "test_input.txt";
/// std::fs::write(input, "1 2 3\n4 5 6\n").unwrap();
/// let data = read_input(input);
/// assert_eq!(data, vec![vec![1, 2, 3], vec![4, 5, 6]]);
/// std::fs::remove_file(input).unwrap();
/// ```
#[inline(always)]
pub fn read_input(file_path: &str) -> Vec<Vec<i32>> {
    let mut reports = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let report: Vec<i32> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();
            if !report.is_empty() {
                reports.push(report);
            }
        }
    }
    reports
}

/// Checks if a report is safe according to part 1 rules.
///
/// # Example
/// ```
/// use advent_of_code::year2024::day2::is_safe_p1;
/// assert!(is_safe_p1(&[1, 2, 3]));
/// assert!(!is_safe_p1(&[1, 3, 7]));
/// ```
pub fn is_safe_p1(report: &[i32]) -> bool {
    if report.len() < 2 {
        return false;
    }

    let diffs: Vec<i32> = report.windows(2).map(|pair| pair[1] - pair[0]).collect();
    diffs.iter().all(|&diff| (1..=3).contains(&diff.abs()))
        && !diffs.windows(2).any(|pair| pair[0] * pair[1] < 0)
}

/// Checks if a report is safe according to part 2 rules.
///
/// # Example
/// ```
/// use advent_of_code::year2024::day2::is_safe_p2;
/// assert!(is_safe_p2(&[1, 3, 6]));
/// assert!(!is_safe_p2(&[1, 5, 10]));
/// ```
pub fn is_safe_p2(report: &[i32]) -> bool {
    if is_safe_p1(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut modified_report = report.to_owned();
        modified_report.remove(i);
        if is_safe_p1(&modified_report) {
            return true;
        }
    }
    false
}

/// Reads lines from a file.
///
/// # Example
/// ```
/// use advent_of_code::year2024::day2::read_lines;
/// let input = "test_input.txt";
/// std::fs::write(input, "Line 1\nLine 2\n").unwrap();
/// let lines: Vec<_> = read_lines(input).unwrap().map(|line| line.unwrap()).collect();
/// assert_eq!(lines, vec!["Line 1", "Line 2"]);
/// std::fs::remove_file(input).unwrap();
/// ```
#[inline(always)]
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::{fs, io::Write};

    #[test]
    fn part1_example() {
        let input = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};

        let file_path = "test_input.txt";
        create_test_file(file_path, input);

        // Test part1
        assert_eq!(part1(file_path), 2);

        // Clean up
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn part2_example() {
        let input = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};

        let file_path = "test_input.txt";
        create_test_file(file_path, input);

        // Test part2
        assert_eq!(part2(file_path), 4);

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
