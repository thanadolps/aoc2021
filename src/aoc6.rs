use itertools::Itertools;

fn simulate(states: &[u8], n_day: i32) -> usize {
    // Potential optimization to O(log(n_day)):
    // Represent as linear recurrent equation and use fast matrix multiplication
    // count[n=6, t+1] = count[n=0, t] + count[n=7, t]
    // count[n=8, t+1] = count[n=0, t]
    // count[n, t+1] = count[n, t] if n != 6 nand n != 8

    // counter[zero_day] = number of lantern fish with state 0
    // counter[(zero_day+x)%9] = number of lantern fish with state x (queue like)
    let mut counter = [0; 9];
    let mut zero_day = 0;

    for state in states {
        counter[*state as usize] += 1;
    }

    for day in 0..n_day {
        // Reproduction (state 0 -> state 6 and state 8)
        // reproduce to state 7, so after state countdown it become day 6
        // state 0 -> 8 automatically happen when state countdown (because queue)
        counter[(zero_day+7)%9] += counter[zero_day];

        // State countdown
        zero_day = (zero_day+1) % 9;
    }

    counter.into_iter().sum()
}

pub fn part_1(input: &str) -> usize {
    let n_day = 80;

    let input = input.as_bytes();
    let mut states = input.chunks(2).map(|chunk| chunk[0]-b'0').collect_vec();

    simulate(&states, n_day)
}


pub fn part_2(input: &str) -> usize {
    let n_day = 256;

    let input = input.as_bytes();
    let mut states = input.chunks(2).map(|chunk| chunk[0]-b'0').collect_vec();

    simulate(&states, n_day)
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
        assert_eq!(part_2(&input), 26984457539);
    }
}
