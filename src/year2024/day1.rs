use aoc_runner_derive::aoc;
use smallvec::SmallVec;

#[inline(always)]
fn parse_input(input: &str) -> (SmallVec<[u32; 1024]>, SmallVec<[u32; 1024]>) {
    let mut left = SmallVec::<[u32; 1024]>::new();
    let mut right = SmallVec::<[u32; 1024]>::new();

    // Avoid allocations by directly iterating over input lines and splitting
    for line in input.lines() {
        let (l, r) = line.split_once(char::is_whitespace).unwrap();
        left.push(l.parse().unwrap());
        right.push(r.trim_start().parse().unwrap());
    }

    (left, right)
}

#[aoc(day1, part1)]
#[must_use]
pub fn part1(input: &str) -> u32 {
    let (mut left, mut right) = parse_input(input);

    // Sort in-place using unstable sort for performance
    left.sort_unstable();
    right.sort_unstable();

    // Reduce bounds checking with iterators
    left.iter()
        .zip(right.iter())
        .map(|(&l, &r)| (l as i32 - r as i32).abs() as u32)
        .sum()
}

#[aoc(day1, part2)]
#[must_use]
pub fn part2(input: &str) -> u32 {
    let (left, right) = parse_input(input);

    // Use a fixed-size array for counts if values are small
    let max_value = *right.iter().max().unwrap_or(&0) as usize;
    let mut counts = vec![0u32; max_value + 1];
    for &r in &right {
        // Count occurrences directly
        counts[r as usize] += 1;
    }

    // Avoid unnecessary bounds checks by iterating directly over `left`
    left.into_iter()
        .filter(|&l| l as usize <= max_value)
        .map(|l| counts[l as usize] * l)
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
