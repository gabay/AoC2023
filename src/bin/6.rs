use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref NUMBER_RE: Regex = Regex::new(r"\d+").unwrap();
}

fn main() {
    let input = aoc2023::readfile("inputs/6");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    let Some((times, distances)) = input.lines().map(|line| {
        NUMBER_RE
            .find_iter(line)
            .map(|number_match| number_match.as_str().parse::<u64>().unwrap())
            .collect::<Vec<_>>()
    }).collect_tuple() else {panic!("Invalid input");};

    times.iter().zip(distances).fold(1, |n, (time, distance)| {
        n * ((1..*time).filter(|n| (n * (time - n)) > distance).count() as u64)
    })
}

fn part2(input: &str) -> u64 {
    let Some((time, distance)) = input.lines().map(|line| {
        NUMBER_RE
            .find(&line.replace(' ', ""))
            .unwrap()
            .as_str()
            .parse::<u64>()
            .unwrap()
    }).collect_tuple() else {panic!("Invalid input");};

    (1..time).filter(|n| (n * (time - n)) > distance).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200";
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 288);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 71503);
    }
}
