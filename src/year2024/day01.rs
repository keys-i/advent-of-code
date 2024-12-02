use aoc_runner_derive::aoc;
use smallvec::SmallVec;

#[inline(always)]
fn parse_input(input: &str) -> (SmallVec<[u32; 1024]>, SmallVec<[u32; 1024]>) {
    let mut left = SmallVec::<[u32; 1024]>::new();
    let mut right = SmallVec::<[u32; 1024]>::new();

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

    // Sort the values using faster, in-place sorting
    left.sort_unstable();
    right.sort_unstable();

    // Use unchecked operations to reduce bounds checks
    let mut sum = 0;
    for i in 0..left.len() {
        unsafe {
            let l = *left.get_unchecked(i);
            let r = *right.get_unchecked(i);
            sum += if l > r { l - r } else { r - l };
        }
    }
    sum
}

#[aoc(day1, part2)]
#[must_use]
pub fn part2(input: &str) -> u32 {
    let (left, right) = parse_input(input);

    // Optimize counting with direct array indexing if values are small
    let max_value = *right.iter().max().unwrap_or(&0) as usize;
    let mut counts = vec![0; max_value + 1];
    for &r in &right {
        counts[r as usize] += 1;
    }

    // Calculate the sum using pre-computed counts
    left.iter()
        .map(|&l| counts.get(l as usize).copied().unwrap_or(0) as u32 * l)
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
    pub fn part2_example() {
        assert_eq!(part2(SAMPLE), 31);
    }
}
