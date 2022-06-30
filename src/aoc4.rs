use itertools::Itertools;

use std::str::FromStr;

#[derive(Debug)]
struct Board {
    cols: [heapless::Vec<u8, 5>; 5],
    rows: [heapless::Vec<u8, 5>; 5],
    sum: u32,
}

impl Board {
    pub fn remove(&mut self, x: u8) {
        for col in self.cols.iter_mut() {
            if let Some((i, _)) = col.iter().find_position(|&&v| v == x) {
                col.swap_remove(i);
                self.sum -= x as u32;
            }
        }
        for row in self.rows.iter_mut() {
            if let Some((i, _)) = row.iter().find_position(|&&v| v == x) {
                row.swap_remove(i);
            }
        }
    }

    pub fn check_win(&self) -> bool {
        self.rows.iter().any(|row| row.is_empty()) || self.cols.iter().any(|row| row.is_empty())
    }
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.as_bytes();

        let mut rows: [heapless::Vec<u8, 5>; 5] = Default::default();
        let mut cols: [heapless::Vec<u8, 5>; 5] = Default::default();
        let mut sum = 0;

        for j in 0..5 {
            for i in 0..5 {
                let idx = 3 * i + 15 * j;
                let s = std::str::from_utf8(&s[idx..idx + 2]).unwrap();
                let x = s.trim().parse().unwrap();

                cols[i].push(x).unwrap();
                rows[j].push(x).unwrap();
                sum += x as u32;
            }
        }

        Ok(Board { rows, cols, sum })
    }
}

pub fn part_1(input: &str) -> usize {
    let mut blocks = input.split("\n\n");
    let draws = blocks
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap());

    let mut boards: Vec<Board> = blocks.map(|board_str| board_str.parse().unwrap()).collect();

    for draw in draws {
        for board in boards.iter_mut() {
            board.remove(draw);

            if board.check_win() {
                return draw as usize * board.sum as usize;
            }
        }
    }
    panic!("No board won")
}

pub fn part_2(input: &str) -> usize {
    let mut blocks = input.split("\n\n");
    let mut draws = blocks
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap());
    let mut boards: Vec<Board> = blocks.map(|board_str| board_str.parse().unwrap()).collect();

    // Play until one board (that hasn't won) is left
    for draw in draws.by_ref() {
        boards.retain_mut(|board| {
            board.remove(draw);
            !board.check_win()
        });
        if boards.len() <= 1 {
            break;
        }
    }

    // Play on the last board until win
    let last_board = &mut boards[0];
    for draw in draws {
        last_board.remove(draw);
        if last_board.check_win() {
            return draw as usize * last_board.sum as usize;
        }
    }

    panic!("No board won")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"
        .trim();
        assert_eq!(part_1(input), 4512);
    }

    #[test]
    fn part2() {
        let input = "
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
"
        .trim();
        assert_eq!(part_2(input), 1924);
    }
}
