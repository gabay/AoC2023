use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref LOCATION_RE: Regex = Regex::new(r"\w{3}").unwrap();
}

fn main() {
    let input = aoc2023::readfile("inputs/8");
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

fn part1(input: &str) -> i64 {
    let pattern = Pattern::new(input.lines().next().unwrap());
    let location_map = LocationMap::from_data(input);
    location_map.count_steps(Location("AAA"), Location("ZZZ"), pattern)
}

fn part2(input: &str) -> i64 {
    let pattern = Pattern::new(input.lines().next().unwrap());
    let location_map = LocationMap::from_data(input);
    location_map.count_steps2(pattern)
}

#[derive(Clone)]
struct Pattern {
    chars: Vec<char>,
    index: usize,
}

impl Pattern {
    fn new(s: &str) -> Self {
        Pattern {
            chars: s.chars().collect(),
            index: 0,
        }
    }
}

impl Iterator for Pattern {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.chars[self.index];
        self.index = (self.index + 1) % self.chars.len();
        Some(item)
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Location<'a>(&'a str);

impl<'a> Location<'a> {
    fn is_source(&self) -> bool {
        self.0.ends_with('A')
    }

    fn is_dest(&self) -> bool {
        self.0.ends_with('Z')
    }
}
struct LocationMap<'a> {
    left: HashMap<Location<'a>, Location<'a>>,
    right: HashMap<Location<'a>, Location<'a>>,
}

impl<'a> LocationMap<'a> {
    fn from_data(data: &'a str) -> Self {
        let items = data
            .lines()
            .skip(2)
            .map(|line| {
                LOCATION_RE
                    .find_iter(line)
                    .map(|loc| loc.as_str())
                    .collect_tuple()
                    .unwrap()
            })
            .collect_vec();
        let left = items
            .iter()
            .map(|(loc, left, _)| (Location(loc), Location(left)))
            .collect::<HashMap<_, _>>();
        let right = items
            .iter()
            .map(|(loc, _, right)| (Location(loc), Location(right)))
            .collect::<HashMap<_, _>>();
        Self { left, right }
    }

    fn count_steps(&self, source: Location, destination: Location, pattern: Pattern) -> i64 {
        let mut current = &source;
        let mut steps = 0;
        for char in pattern {
            if *current == destination {
                break;
            }
            current = self.get_next_location(current, char);
            steps += 1;
        }
        steps
    }

    fn get_next_location(&self, loc: &'a Location, c: char) -> &'a Location {
        if c == 'L' {
            &self.left[loc]
        } else {
            &self.right[loc]
        }
    }

    fn count_steps2(&self, pattern: Pattern) -> i64 {
        let locations = self.get_initial_locations();
        let steps = locations
            .iter()
            .map(|loc| self.get_solution_steps(loc, pattern.clone()))
            .reduce(num::integer::lcm)
            .unwrap();
        steps
    }

    fn get_initial_locations(&self) -> Vec<&Location> {
        self.left.keys().filter(|loc| loc.is_source()).collect()
    }

    fn get_solution_steps(&self, loc: &Location, mut pattern: Pattern) -> i64 {
        let mut curr = loc;
        let mut steps = 0;
        while !curr.is_dest() {
            let c: char = pattern.next().unwrap();
            curr = self.get_next_location(curr, c);
            steps += 1;
        }
        steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn test_part1_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(input), 6);
    }
}
