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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "
forward 5
down 5
forward 8
up 3
down 8
forward 2
        "
        .trim();
        assert_eq!(part_1(&input), 150)
    }

    #[test]
    fn part2() {
        let input = "
forward 5
down 5
forward 8
up 3
down 8
forward 2
        "
        .trim();
        assert_eq!(part_2(&input), 900)
    }
}
