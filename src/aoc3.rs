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

    let retrieve_bit = |i: usize| {
        // let i = i-(i%(n_bit+1));
        (i..i + n_bit).fold(0_u32, |acc, i| (acc << 1) | (input[i] - b'0') as u32)
    };

    // Split into oxy an co2 based on first bit
    let (zeros, ones) = (0..input.len())
        .step_by(n_bit + 1)
        .partition::<Vec<_>, _>(|&i| input[i] == b'0');
    let (mut oxy_pos, mut co2_pos) = if ones.len() >= zeros.len() {
        (ones, zeros)
    } else {
        (zeros, ones)
    };

    // Discard until one left
    let mut offset = 1;
    let oxy = loop {
        if oxy_pos.len() == 1 {
            break oxy_pos[0];
        }
        debug_assert!(offset < n_bit);
        let (zeros, ones) = oxy_pos
            .iter()
            .partition::<Vec<_>, _>(|&i| input[i + offset] == b'0');

        if ones.len() >= zeros.len() {
            oxy_pos = ones;
        } else {
            oxy_pos = zeros;
        }
        offset += 1;
    };

    let mut offset = 1;
    let co2 = loop {
        if co2_pos.len() == 1 {
            break co2_pos[0];
        }
        debug_assert!(offset < n_bit);
        let (zeros, ones) = co2_pos
            .iter()
            .partition::<Vec<_>, _>(|&i| input[i + offset] == b'0');

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
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
        assert_eq!(part_1(input), 198);
    }

    #[test]
    fn part2() {
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
        assert_eq!(part_2(input), 230);
    }
}
