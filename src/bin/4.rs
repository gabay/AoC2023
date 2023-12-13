use std::collections::HashSet;

use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref CARD_RE: Regex = Regex::new(r"Card\s*(\d+):\s*(.*?)\s*\|\s*(.*)").unwrap();
    static ref NUMBER_RE: Regex = Regex::new(r"\d+").unwrap();
}

fn main() {
    let input = aoc2023::readfile("inputs/4");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    input.lines().map(|line| Card::from(line).score()).sum()
}

fn part2(input: &str) -> i32 {
    let mut counts = vec![1; input.lines().count()];
    for (index, card) in input.lines().map(Card::from).enumerate() {
        for i in 0..card.score2() {
            counts[index + 1 + i] += counts[index];
        }
    }
    counts.iter().sum()
}

struct Card {
    win_numbers: HashSet<i32>,
    my_numbers: HashSet<i32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let (_, parts) = CARD_RE.captures(value).unwrap().extract::<3>();
        Card {
            win_numbers: NUMBER_RE
                .find_iter(parts[1])
                .map(|n| n.as_str().parse::<i32>().unwrap())
                .collect(),
            my_numbers: NUMBER_RE
                .find_iter(parts[2])
                .map(|n| n.as_str().parse::<i32>().unwrap())
                .collect(),
        }
    }
}

impl Card {
    fn score(&self) -> i32 {
        match self.win_numbers.intersection(&self.my_numbers).count() {
            0 => 0,
            n => 2_i32.pow((n - 1) as u32),
        }
    }
    fn score2(&self) -> usize {
        self.win_numbers.intersection(&self.my_numbers).count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .trim();
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part2() {
        let input = "
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
            .trim();
        assert_eq!(part2(input), 30);
    }
}
