use itertools::Itertools;


type Point = [u32; 2];

fn parse_input(input: &str) -> impl Iterator<Item = (Point, Point)> + '_ {
    input.lines().map(|line| {
        let (p1, p2) = line.split_once(" -> ").unwrap();
        let (p1x, p1y) = p1.split_once(',').unwrap();
        let (p2x, p2y) = p2.split_once(',').unwrap();
        let p1 = [p1x.parse().unwrap(), p1y.parse().unwrap()];
        let p2 = [p2x.parse().unwrap(), p2y.parse().unwrap()];
        (p1, p2)
    })
}

fn fill_horizontal(grid: &mut [u32], p1: Point, p2: Point, w: usize, _h: usize) -> u32 {
    let x = p1[0] as usize;

    let (mut y1, mut y2) = (p1[1] as usize, p2[1] as usize);
    if y1 > y2 {
        std::mem::swap(&mut y1, &mut y2);
    }

    let mut overlap = 0;
    for y in y1..=y2 {
        let v = &mut grid[w * y + x];
        overlap += (*v == 1) as u32;
        *v += 1;
    }
    overlap
}

fn fill_vertical(grid: &mut [u32], p1: Point, p2: Point, w: usize, _h: usize) -> u32 {
    let (x1, x2) = (p1[0] as usize, p2[0] as usize);
    let y = p1[1] as usize;

    let mut i1 = w * y + x1;
    let mut i2 = w * y + x2;
    if i1 > i2 {
        std::mem::swap(&mut i1, &mut i2);
    }

    let mut overlap = 0;
    for v in &mut grid[i1..=i2] {
        overlap += (*v == 1) as u32;
        *v += 1;
    }
    overlap
}

fn fill_points(grid: &mut [u32], points: impl Iterator<Item = (usize, usize)>, w: usize) -> u32 {
    points
        .map(|(x, y)| {
            let v = &mut grid[w * y + x];
            *v += 1;
            (*v == 2) as u32
        })
        .sum()
}

fn fill_diagonal(grid: &mut Vec<u32>, p1: Point, p2: Point, w: usize, _h: usize) -> u32 {
    let [x1, y1] = p1;
    let [x1, y1] = [x1 as usize, y1 as usize];
    let [x2, y2] = p2;
    let [x2, y2] = [x2 as usize, y2 as usize];

    let rev_x = x2 < x1;
    let rev_y = y2 < y1;

    match (rev_x, rev_y) {
        (false, false) => fill_points(grid, (x1..=x2).zip(y1..=y2), w),
        (false, true) => fill_points(grid, (x1..=x2).zip((y2..=y1).rev()), w),
        (true, false) => fill_points(grid, (x2..=x1).rev().zip(y1..=y2), w),
        (true, true) => fill_points(grid, (x2..=x1).rev().zip((y2..=y1).rev()), w),
    }
}

pub fn part_1(input: &str) -> usize {
    let spans = parse_input(input)
        .filter(|(p1, p2)| p1[0] == p2[0] || p1[1] == p2[1])
        .collect_vec();

    let xmax = spans
        .iter()
        .flat_map(|(p1, p2)| [p1[0], p2[0]])
        .max()
        .unwrap();
    let ymax = spans
        .iter()
        .flat_map(|(p1, p2)| [p1[1], p2[1]])
        .max()
        .unwrap();
    let (w, h) = (xmax as usize + 1, ymax as usize + 1);

    let mut grid = vec![0; w * h];

    spans
        .into_iter()
        .map(|(p1, p2)| {
            if p1[0] == p2[0] {
                fill_horizontal(&mut grid, p1, p2, w, h)
            } else if p1[1] == p2[1] {
                fill_vertical(&mut grid, p1, p2, w, h)
            } else {
                unreachable!()
            }
        })
        .sum::<u32>() as usize
}

pub fn part_2(input: &str) -> usize {
    let spans = parse_input(input).collect_vec();
    let xmax = spans
        .iter()
        .flat_map(|(p1, p2)| [p1[0], p2[0]])
        .max()
        .unwrap();
    let ymax = spans
        .iter()
        .flat_map(|(p1, p2)| [p1[1], p2[1]])
        .max()
        .unwrap();
    let (w, h) = (xmax as usize + 1, ymax as usize + 1);

    let mut grid = vec![0; w * h];
    spans
        .into_iter()
        .map(|(p1, p2)| {
            if p1[0] == p2[0] {
                fill_horizontal(&mut grid, p1, p2, w, h)
            } else if p1[1] == p2[1] {
                fill_vertical(&mut grid, p1, p2, w, h)
            } else {
                fill_diagonal(&mut grid, p1, p2, w, h)
            }
        })
        .sum::<u32>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let input = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"
        .trim();
        assert_eq!(part_1(&input), 5);
    }

    #[test]
    fn part2() {
        let input = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
"
        .trim();
        assert_eq!(part_2(&input), 12);
    }
}
