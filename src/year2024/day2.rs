use aoc_runner_derive::aoc;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[aoc(day2, part1)]
#[must_use]
pub fn part1(file_path: &str) -> usize {
    let reports = read_input(file_path);
    reports.iter().filter(|report| is_safe_p1(report)).count()
}

#[aoc(day2, part2)]
#[must_use]
pub fn part2(file_path: &str) -> usize {
    let reports = read_input(file_path);
    reports.iter().filter(|report| is_safe_p2(report)).count()
}

#[inline(always)]
fn read_input(file_path: &str) -> Vec<Vec<i32>> {
    let mut reports = Vec::new();
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let report: Vec<i32> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();
            reports.push(report);
        }
    }
    reports
}

fn is_safe_p1(report: &[i32]) -> bool {
    let diffs: Vec<i32> = report.windows(2).map(|pair| pair[1] - pair[0]).collect();
    diffs.iter().all(|&diff| diff.abs() > 0 && diff.abs() < 4)
        && !diffs.windows(2).any(|pair| pair[0] * pair[1] < 0)
}

fn is_safe_p2(report: &[i32]) -> bool {
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

#[inline(always)]
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
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
    fn test_parts() {
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
