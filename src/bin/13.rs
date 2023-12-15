use itertools::{enumerate, Itertools};

fn main() {
    let input = aoc2023::readfile("inputs/13");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    input.split("\n\n").map(summarize).sum()
}

fn part2(input: &str) -> usize {
    input.split("\n\n").map(summarize2).sum()
}

fn summarize(board: &str) -> usize {
    let rows = board
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let columns = get_columns(&rows);

    for i in 1..rows.len() {
        if iters_match(rows[i..rows.len()].iter(), rows[0..i].iter().rev()) {
            return i * 100;
        }
    }
    for i in 1..columns.len() {
        if iters_match(columns[i..columns.len()].iter(), columns[0..i].iter().rev()) {
            return i;
        }
    }
    panic!();
}
fn summarize2(board: &str) -> usize {
    let rows = board
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();
    let columns = get_columns(&rows);

    for i in 1..rows.len() {
        if rows[i..rows.len()]
            .iter()
            .zip(rows[0..i].iter().rev())
            .map(|(a, b)| count_diffs(a, b))
            .sum::<usize>()
            == 1
        {
            return i * 100;
        }
    }
    for i in 1..columns.len() {
        if columns[i..columns.len()]
            .iter()
            .zip(columns[0..i].iter().rev())
            .map(|(a, b)| count_diffs(a, b))
            .sum::<usize>()
            == 1
        {
            return i;
        }
    }
    panic!();
}

fn get_columns(rows: &[Vec<char>]) -> Vec<Vec<char>> {
    let mut columns = vec![vec![]; rows[0].len()];
    for row in rows {
        for (j, cell) in enumerate(row) {
            columns[j].push(*cell);
        }
    }
    columns
}

fn iters_match<T, U>(mut a: T, mut b: U) -> bool
where
    T: Iterator,
    U: Iterator,
    T::Item: PartialEq<U::Item>,
{
    while let (Some(aa), Some(bb)) = (a.next(), b.next()) {
        if aa != bb {
            return false;
        }
    }
    true
}

fn count_diffs(a: &[char], b: &[char]) -> usize {
    a.iter().zip(b).filter(|(aa, bb)| **aa != **bb).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 400);
    }
}
