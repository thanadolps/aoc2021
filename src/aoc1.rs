use itertools::Itertools;
use std::cmp::Ordering;

fn compare_numeric_str(x1: &str, x2: &str) -> Ordering {
    // Assume aligned, non-zero padded numeric string
    x1.len().cmp(&x2.len()).then(x1.cmp(x2))
}

pub fn part_1(input: &str) -> usize {
    // Simple linear scan and compare
    // Comparing string directly should be bit faster than converting string to number first

    input
        .lines()
        .tuple_windows()
        .filter(|(d1, d2)| compare_numeric_str(d2, d1).is_gt())
        .count()
}

pub fn part_2(input: &str) -> usize {
    // Compare "sum of three-measurement sliding window"
    // Notice that between 2 contiguous sliding window,
    // only the leftmost are put out from and rightmost are put into the sliding window.
    // So to compare if the sum of next sliding window are greater than the previous slider window,
    // we can just compare if the number that get put in are greater than the one which get put out.

    input
        .lines()
        .tuple_windows()
        .filter(|(discard_d, _, _, new_d)| compare_numeric_str(new_d, discard_d).is_gt())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1\n2\n3" => 2)]
    #[test_case("2\n1\n3" => 1)]
    #[test_case("12\n123\n1234" => 2)]
    fn part1(input: &str) -> usize {
        part_1(input)
    }

    #[test_case("199\n200\n208\n210\n200\n207\n240\n269\n260\n263" => 5)]
    fn part2(input: &str) -> usize {
        part_2(input)
    }
}
