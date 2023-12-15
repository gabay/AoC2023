use itertools::Itertools;

fn main() {
    let input = aoc2023::readfile("inputs/9");
    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.split(' ').map(aoc2023::parse).collect_vec())
        .map(deduce_next_value)
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| line.split(' ').map(aoc2023::parse).collect_vec())
        .map(deduce_previous_value)
        .sum()
}

fn deduce_next_value(values: Vec<i32>) -> i32 {
    if values.iter().all(|value| *value == 0) {
        return 0;
    }
    let derivitives = values
        .iter()
        .tuple_windows()
        .map(|(a, b)| *b - *a)
        .collect_vec();
    let diff = deduce_next_value(derivitives);
    return values.last().unwrap() + diff;
}

fn deduce_previous_value(values: Vec<i32>) -> i32 {
    if values.iter().all(|value| *value == 0) {
        return 0;
    }
    let derivitives = values
        .iter()
        .tuple_windows()
        .map(|(a, b)| *b - *a)
        .collect_vec();
    let diff = deduce_previous_value(derivitives);
    return values.first().unwrap() - diff;
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 114)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 2)
    }
}
