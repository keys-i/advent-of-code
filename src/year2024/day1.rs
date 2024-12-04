use aoc_runner_derive::aoc;
use smallvec::SmallVec;

/// Parses the input into two `SmallVec` vectors.
///
/// # Arguments
///
/// * `input` - The input string containing lines of numbers separated by whitespace.
///
/// # Returns
///
/// A tuple of two `SmallVec` vectors: the left and right columns of parsed numbers.
///
/// # Examples
///
/// ```
/// use smallvec::SmallVec;
/// use advent_of_code::year2024::day1::parse_input;
///
/// let input = "3 4\n4 3\n2 5";
/// let (left, right) = parse_input(input);
/// assert_eq!(left, SmallVec::from_buf([3, 4, 2]));
/// assert_eq!(right, SmallVec::from_buf([4, 3, 5]));
/// ```
#[inline(always)]
pub fn parse_input(input: &str) -> (SmallVec<[u32; 1024]>, SmallVec<[u32; 1024]>) {
    let mut left = SmallVec::<[u32; 1024]>::new();
    let mut right = SmallVec::<[u32; 1024]>::new();

    input.lines().for_each(|line| {
        let mut nums = line.split_whitespace();
        left.push(nums.next().unwrap().parse().unwrap());
        right.push(nums.next().unwrap().parse().unwrap());
    });

    (left, right)
}

/// Computes the result for part 1 by calculating the absolute differences.
///
/// # Arguments
///
/// * `input` - The input string containing pairs of numbers separated by whitespace.
///
/// # Returns
///
/// The sum of the absolute differences of the sorted pairs.
///
/// # Examples
///
/// ```
/// use advent_of_code::year2024::day1::part1;
///
/// let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
/// assert_eq!(part1(input), 11);
/// ```
#[aoc(day1, part1)]
#[must_use]
pub fn part1(input: &str) -> u32 {
    let (mut left, mut right) = parse_input(input);

    left.sort_unstable();
    right.sort_unstable();

    left.iter().zip(&right).fold(0, |sum, (&l, &r)| {
        sum + (l as i32 - r as i32).unsigned_abs()
    })
}

/// Computes the result for part 2 by summing weighted counts.
///
/// # Arguments
///
/// * `input` - The input string containing pairs of numbers separated by whitespace.
///
/// # Returns
///
/// The sum of the weights calculated using occurrences in the right column.
///
/// # Examples
///
/// ```
/// use advent_of_code::year2024::day1::part2;
///
/// let input = "3 4\n4 3\n2 5\n1 3\n3 9\n3 3";
/// assert_eq!(part2(input), 31);
/// ```
#[aoc(day1, part2)]
#[must_use]
pub fn part2(input: &str) -> u32 {
    let (left, right) = parse_input(input);

    let max_value = *right.iter().max().unwrap_or(&0) as usize;

    // Precompute counts
    let mut counts = vec![0u32; max_value + 1];
    right.iter().for_each(|&r| counts[r as usize] += 1);

    // Compute weighted sum directly
    left.into_iter()
        .filter_map(|l| counts.get(l as usize).map(|&count| count * l))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    const SAMPLE: &str = indoc! {"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3
    "};

    #[test]
    pub fn part1_example() {
        assert_eq!(part1(SAMPLE), 11);
    }

    #[test]
    pub fn part1_input() {
        let part1 = part1(include_str!("../../input/2024/day1.txt"));
        assert_eq!(part1, 2904518);
    }

    #[test]
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 31);
    }

    #[test]
    pub fn part2_input() {
        let part2 = part2(include_str!("../../input/2024/day1.txt"));
        assert_eq!(part2, 18650129);
    }
}
