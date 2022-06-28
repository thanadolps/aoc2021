mod aoc1 {
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
}

mod aoc2 {

    #[inline]
    fn digit_to_number(digit: u8) -> u8 {
        debug_assert!((b'0'..=b'9').contains(&digit));
        digit - b'0'
    }

    pub fn part_1(input: &str) -> usize {
        // 1. Input is very structure, we can exploit this and hardcode magic number and stuff.
        // 2. Notice that the number in input are all single digit, we can do quick and dirty conversion

        let [horizontal, depth] = input.lines().fold([0, 0], |[h, d], line| {
            let line = line.as_bytes();
            match line[0] {
                b'f' => [h + digit_to_number(line[8]) as usize, d],
                b'u' => [h, d - digit_to_number(line[3]) as usize],
                b'd' => [h, d + digit_to_number(line[5]) as usize],
                _ => panic!("Invalid input"),
            }
        });
        horizontal * depth
    }

    pub fn part_2(input: &str) -> usize {
        let [horizontal, depth, _aim] = input.lines().fold([0, 0, 0], |[h, d, a], line| {
            let line = line.as_bytes();
            match line[0] {
                b'f' => {
                    let x = digit_to_number(line[8]) as usize;
                    [h + x, d + a * x, a]
                }
                b'u' => [h, d, a - digit_to_number(line[3]) as usize],
                b'd' => [h, d, a + digit_to_number(line[5]) as usize],
                _ => panic!("Invalid input"),
            }
        });
        horizontal * depth
    }
}

fn main() {
    let mut table = comfy_table::Table::new();
    table
        .set_header(["Name", "Runtime", "Output"])
        .load_preset(comfy_table::presets::ASCII_MARKDOWN);

    let mut total_ns = 0.;

    for (name, input_path, f) in [
        (
            "AOC1 P1",
            "input/aoc1.txt",
            aoc1::part_1 as fn(&str) -> usize,
        ),
        ("AOC1 P2", "input/aoc1.txt", aoc1::part_2),
        ("AOC2 P1", "input/aoc2.txt", aoc2::part_1),
        ("AOC2 P2", "input/aoc2.txt", aoc2::part_2),
    ] {
        let input = std::fs::read_to_string(input_path).unwrap();
        let bench_stats = easybench::bench(|| f(&input));
        total_ns += bench_stats.ns_per_iter;

        table.add_row([
            name.to_string(),
            bench_stats.to_string(),
            f(&input).to_string(),
        ]);
    }

    let total_time = std::time::Duration::from_nanos(total_ns as u64);
    table.add_row([
        "Total".to_string(),
        format!("{:?}", total_time),
        String::new(),
    ]);

    println!("{}", table);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1\n2\n3" => 2)]
    #[test_case("2\n1\n3" => 1)]
    #[test_case("12\n123\n1234" => 2)]
    fn aoc1_part1(input: &str) -> usize {
        aoc1::part_1(input)
    }

    #[test_case("199\n200\n208\n210\n200\n207\n240\n269\n260\n263" => 5)]
    fn aoc1_part2(input: &str) -> usize {
        aoc1::part_2(input)
    }

    #[test]
    fn aoc2_part1() {
        let input = "
forward 5
down 5
forward 8
up 3
down 8
forward 2
        "
        .trim();
        assert_eq!(aoc2::part_1(&input), 150)
    }

    #[test]
    fn aoc2_part2() {
        let input = "
forward 5
down 5
forward 8
up 3
down 8
forward 2
        "
        .trim();
        assert_eq!(aoc2::part_2(&input), 900)
    }
}
