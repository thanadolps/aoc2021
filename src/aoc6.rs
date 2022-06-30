use itertools::Itertools;

pub fn part_1(input: &str) -> usize {
    let input = input.as_bytes();
    let mut states = input.chunks(2).map(|chunk| chunk[0]-b'0').collect_vec();

    for day in 0..80 {
        let new_born_count = states.iter_mut().fold(0, |count, state| {
            if *state == 0 {
                *state = 6;
                count + 1
            }
            else {
                *state -= 1;
                count
            }
        });

        states.extend(std::iter::repeat(8).take(new_born_count));
    }

    states.len()
}

pub fn part_2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "3,4,3,1,2";
        assert_eq!(part_1(&input), 5934);
    }

    #[test]
    fn part2() {
        let input = "3,4,3,1,2";
        assert_eq!(part_2(&input), 12);
    }
}
