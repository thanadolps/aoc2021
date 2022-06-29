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

mod aoc3 {
    use itertools::Itertools;

    pub fn part_1(input: &str) -> usize {
        let input = input.as_bytes();
        let (n_bit, _) = input.iter().find_position(|&&c| c == b'\n').unwrap();
        let n_row = input.len() / (n_bit + 1);

        let mut gamma_rate = 0u16;
        for i in 0..n_bit {
            // Count how many one are there at bit pos 'i'
            // Branch-less, except for the loop
            let mut one_count = 0;
            for r in (0..input.len()).step_by(n_bit + 1) {
                one_count += (input[r + i] - b'0') as usize;
            }
            let is_one = one_count > (n_row / 2);
            gamma_rate <<= 1;
            gamma_rate |= is_one as u16;
        }

        let epsilon_rate = !gamma_rate & ((1 << n_bit) - 1);

        gamma_rate as usize * epsilon_rate as usize
    }

    pub fn part_2(input: &str) -> usize {
        let input = input.as_bytes();
        let (n_bit, _) = input.iter().find_position(|&&c| c == b'\n').unwrap();
        let n_row = (input.len()+1) / (n_bit + 1);

        let retrieve_bit = |i: usize| {
            // let i = i-(i%(n_bit+1));
            (i..i+n_bit).fold(0_u32, |acc, i| {
                (acc << 1) | (input[i]-b'0') as u32
            })
        };

        // Split into oxy an co2 based on first bit
        let (zeros, ones) =
            (0..input.len()).step_by(n_bit + 1).partition::<Vec<_>, _>(|&i| input[i]==b'0');
        let (mut oxy_pos, mut co2_pos) = if ones.len() >= zeros.len() {
            (ones, zeros)
        }
        else {
            (zeros, ones)
        };


        // Discard until one left
        let mut offset = 1;
        let oxy = loop {
            if oxy_pos.len() == 1 { break oxy_pos[0] }
            debug_assert!(offset < n_bit);
            let (zeros, ones) =
                oxy_pos.iter().partition::<Vec<_>, _>(|&i| input[i+offset]== b'0');

            if ones.len() >= zeros.len() {
                oxy_pos = ones;
            } else {
                oxy_pos = zeros;
            }
            offset += 1;
        };

        let mut offset = 1;
        let co2 = loop {
            if co2_pos.len() == 1 { break co2_pos[0] }
            debug_assert!(offset < n_bit);
            let (zeros, ones) =
                co2_pos.iter().partition::<Vec<_>, _>(|&i| input[i+offset]== b'0');

            if zeros.len() <= ones.len() {
                co2_pos = zeros;
            } else {
                co2_pos = ones;
            }
            offset += 1;
        };

        let oxygen_generator = retrieve_bit(oxy);
        let co2_scrubber = retrieve_bit(co2);

        oxygen_generator as usize * co2_scrubber as usize
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
        ("AOC3 P1", "input/aoc3.txt", aoc3::part_1),
        ("AOC3 P2", "input/aoc3.txt", aoc3::part_2),
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

    #[test]
    fn aoc3() {
        let input = "
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
        "
        .trim();
        assert_eq!(aoc3::part_1(input), 198);
        assert_eq!(aoc3::part_2(input), 230);
    }
}
