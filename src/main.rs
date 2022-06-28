use itertools::Itertools;

fn aoc1(input: String) -> usize {
    input
        .lines()
        .tuple_windows()
        .filter(|(d1, d2)| d2.len() > d1.len() || (d2.len() == d1.len() && d2 > d1))
        .count()
}

fn main() {
    let input = std::fs::read_to_string("input/aoc1.txt").unwrap();
    println!("{}", aoc1(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("1\n2\n3" => 2)]
    #[test_case("2\n1\n3" => 1)]
    #[test_case("12\n123\n1234" => 2)]
    fn aoc1_test(input: impl Into<String>) -> usize {
        aoc1(input.into())
    }
}
